<script>
  import TableRow from "@components/TableRow.svelte";
  import { emitter } from "@event/event";
  import toast from "svelte-french-toast";
  import IcRoundRefresh from "~icons/ic/round-refresh";
  import { onMount } from "svelte";

  let links = [];

  emitter.on("fetchLinks", fetchLinks);

  onMount(fetchLinks);

  function fetchLinks() {
    toast.promise(
      new Promise((res, rej) => {
        fetch("/api/admin/links")
          .then((r) => r.json())
          .then((r) => {
            links = r;
            res();
          })
          .catch((e) => rej(e));
      }),
      {
        loading: "Fetching links",
        success: "Fetched links",
        error: (err) => `Error fetching links: ${err}`,
      }
    );
  }
</script>

<button
  class="m-auto mb-3 flex items-center justify-center rounded-md bg-dracula-purple p-2 hover:bg-dracula-purple-400"
  on:click={fetchLinks}
>
  <IcRoundRefresh class="mr-1 inline h-5 w-5" />
  Refresh links
</button>
{#if links[0]}
  <table class="m-auto w-full px-1 md:w-5/6">
    <tr>
      <th>Slug</th>
      <th>URL</th>
      <th>Set to expire</th>
      <th>Controls</th>
    </tr>
    {#each links as link}
      <TableRow {link} />
    {/each}
  </table>
{/if}
