<script>
  import FaSignOutAlt from "svelte-icons/fa/FaSignOutAlt.svelte";
</script>

<form action="/api/admin/logout" method="post">
  <button
    type="submit"
    class="m-auto my-3 flex w-fit items-center justify-center rounded-md bg-dracula-red-500 p-2 px-4 hover:bg-dracula-red-600"
  >
    <span class="mr-1 h-5 w-5"><FaSignOutAlt /></span>
    Sign out
  </button>
</form>
