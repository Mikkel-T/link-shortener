<script>
  import Icon from "@iconify/svelte";
  import TableRow from "@components/TableRow.svelte";
  import { emitter } from "@event/event";
  import { onMount } from "svelte";

  let urls = [];

  emitter.on("fetchUrls", fetchUrls);

  onMount(fetchUrls);

  function fetchUrls() {
    emitter.emit(
      "toast-promise",
      new Promise((res, rej) => {
        fetch("/api/admin/links")
          .then((r) => r.json())
          .then((r) => {
            urls = r;
            res();
          })
          .catch((e) => rej(e));
      }),
      {
        loading: "Fetching URLs",
        success: "Fetched URLs",
        error: (err) => `Error fetching URLs: ${err}`,
      }
    );
  }
</script>

<button
  class="m-auto mb-3 flex items-center justify-center rounded-md bg-dracula-purple p-2 hover:bg-dracula-purple-400"
  on:click={fetchUrls}
>
  <Icon class="mr-1 inline h-5 w-5" icon="ic:round-refresh" />
  Refresh urls
</button>
{#if urls[0]}
  <table class="m-auto w-full px-1 md:w-5/6">
    <tr>
      <th>Slug</th>
      <th>URL</th>
      <th>Expires after</th>
      <th>Controls</th>
    </tr>
    {#each urls as url}
      <TableRow {url} />
    {/each}
  </table>
{/if}
