<script lang="ts">
  import { enhance } from '$app/forms'
  let password = $state('')
</script>

<div class="h-screen flex flex-col">
  <form method="POST" action="?/login" use:enhance class="w-3/12 h-3/12 m-auto p-4">
    <div class="form-control">
      <label class="label" for="password">
        <span class="label-text">Password</span>
      </label>
      <input
        name="password"
        type="password"
        class="input input-bordered w-full"
        required
        bind:value="{password}" />
    </div>
    <button type="submit" class="btn btn_emerald w-full mt-3">Sign In</button>
  </form>
</div>
