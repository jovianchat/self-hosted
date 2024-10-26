import { dev } from '$app/environment';
import { redirect, type HandleFetch } from '@sveltejs/kit';
import { url } from 'inspector';

export const handleFetch: HandleFetch = async ({ request, fetch, event }) => {
    // Send fetch request forward
    // Auth page going forward is login authentication so no need to check for auth
    if (event.url.pathname === "/auth") return fetch(request);

    let authTokens: AuthTokens = {
        refresh: {
            token: event.cookies.get("refresh_token")!,
            expiration: Number(event.cookies.get("refresh_token_expiration")),
        },
        access: {
            token: event.cookies.get("access_token")!,
            expiration: Number(event.cookies.get("access_token_expiration")),
        },
    }
    const currentTime = Math.ceil(Date.now() / 1000);
    const expiryInTwoDays = 2 * 24 * 60 * 60;
    const expiryInTenMins = 10 * 60;

    if (!authTokens.refresh?.token || authTokens.refresh?.expiration < currentTime) {
        return redirect(302, "/auth");
    } else if (authTokens.refresh.expiration - currentTime < expiryInTwoDays) {
        const res = await fetch("/http-server/auth/new-refresh-token", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(authTokens.refresh.token),
        })
        if (!res.ok) {
            throw new Error(await res.text());
        } else {
            const { refresh }: AuthTokens = await res.json();
            setCookies(event, "refresh_token", refresh!.token);
            setCookies(event, "refresh_token_expiration", refresh!.expiration.toString());
            authTokens.refresh = refresh!;
        }
    }
    //Get new access token if not exist or less than 10 mins
    if (!authTokens.access?.token || authTokens.access.expiration - currentTime < expiryInTenMins) {
        const res = await fetch("/http-server/auth/new-access-token", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(authTokens.refresh.token),
        })
        if (!res.ok) {
            throw new Error(await res.text());
        }
        const { access }: AuthTokens = await res.json();
        setCookies(event, "access_token", access!.token);
        setCookies(event, "access_token_expiration", access!.expiration.toString());
        authTokens.access = access!;
    }

    // Just to refresh tokens without calling backend
    // hooks_fetchHandler api route does exist and can connect to backend if returned fetch instead of response
    if (event.url.pathname === "/hooks_fetchHandler") return new Response("OK", { status: 200 });

    // Add the access token to the request and send it
    request.headers.set("Authorization", `Bearer ${authTokens.access.token}`);
    return fetch(request);
}

function setCookies(event: any, cookieName: string, cookieValue: string) {
    event.cookies.set(cookieName, cookieValue, {
        path: "/",
        httpOnly: true,
        secure: !dev,
        sameSite: "strict",
    });
}