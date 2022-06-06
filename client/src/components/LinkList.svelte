<script>
  import IoMdRefresh from "svelte-icons/io/IoMdRefresh.svelte";
  import TableRow from "@components/TableRow.svelte";
  import { emitter } from "@event/event";
  import { onMount } from "svelte";

  let urls = [];

  emitter.on("fetchUrls", fetchUrls);

  onMount(fetchUrls);

  function fetchUrls() {
    emitter.emit(
      "toast-promise",
      new Promise((res, _rej) => {
        fetch("/api/admin/links")
          .then((r) => r.json())
          .then((r) => {
            urls = r;
            res();
          });
      }),
      {
        loading: "Fetching URLs",
        success: "Fetched URLs",
        error: (err) => `Error: ${err}`,
      }
    );
  }
</script>

<button
  class="m-auto mb-3 flex items-center justify-center rounded-md bg-dracula-purple p-2"
  on:click={fetchUrls}
>
  <span class="mr-1 h-5 w-5">
    <IoMdRefresh />
  </span>
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
