<script>
  import Icon from "@iconify/svelte";
</script>

<form action="/api/admin/logout" method="post">
  <button
    type="submit"
    class="m-auto my-3 flex w-fit items-center justify-center rounded-md bg-dracula-red-500 p-2 px-4 hover:bg-dracula-red-600"
  >
    <Icon class="mr-1 inline h-5 w-5" icon="ic:round-logout" />
    Sign out
  </button>
</form>
