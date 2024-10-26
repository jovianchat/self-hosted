<script lang="ts">
  import MdiKeyboardArrowDown from '~icons/mdi/keyboard-arrow-down'
  let user = {
    email: 'admin',
  }
</script>

{#if user}
  <div class="dropdown dropdown-top dropdown-end mt-auto mb-1">
    <div tabindex="0" role="button" class="btn btn-ghost flex flex-nowrap gap-2 items-center p-2">
      <div class="min-w-0 flex items-center">
        <span class="text-lg truncate block">{user.email}</span>
        <MdiKeyboardArrowDown class="w-7 h-7" />
      </div>
    </div>
    <ul class="dropdown-content menu menu-lg z-[1] rounded-box bg-base-200 p-2 shadow mb-1 w-full">
      <li>
        <a href="/profile/password"> Profile </a>
      </li>
      <!-- <li>
        <a href="/llmSettings/prompt" class="justify-between"
          >Settings
          <span class="badge">API</span></a>
      </li> -->
      <form method="POST" action="/auth?/logout">
        <li>
          <button type="submit">Logout</button>
        </li>
      </form>
    </ul>
  </div>
{/if}
