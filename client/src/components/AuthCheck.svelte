<script>
  import { onMount } from "svelte";

  let link = "";
  let buttonText = "";
  onMount(() => {
    fetch("/api/admin/auth/status")
      .then((r) => r.json())
      .then((r) => {
        if (r.logged_in) {
          link = "/dash";
          buttonText = "Open dashboard";
        } else {
          link = "/login";
          buttonText = "Log in";
        }
      });
  });
</script>

{#if link && buttonText}
  <a
    href={link}
    class="mt-5 w-fit cursor-pointer rounded-lg bg-dracula-green-800 p-3 text-xl hover:bg-dracula-green-900"
  >
    {buttonText}
  </a>
{/if}
