<script>
  import IoMdTrash from "svelte-icons/io/IoMdTrash.svelte";
  import FaPencilAlt from "svelte-icons/fa/FaPencilAlt.svelte";
  import FaCopy from "svelte-icons/fa/FaCopy.svelte";
  import FaCheck from "svelte-icons/fa/FaCheck.svelte";
  import Clipboard from "@components/Clipboard.svelte";
  import { emitter } from "@event/event";
  export let url;

  let editing = false;
  let newUrl = url.url;

  function deleteSlug() {
    emitter.emit(
      "toast-promise",
      new Promise((res, rej) => {
        fetch(`/api/admin/links/${url.slug}`, {
          method: "DELETE",
        })
          .then((response) => response.json())
          .then((response) => {
            if (response.success) {
              emitter.emit("fetchUrls");
              res();
            } else {
              rej(response.message);
            }
          });
      }),
      {
        loading: `Deleting short url with slug "${url.slug}"`,
        success: `Deleted short url with slug "${url.slug}"`,
        error: (err) => `Error: ${err}`,
      }
    );
  }

  function editUrl() {
    emitter.emit(
      "toast-promise",
      new Promise((res, rej) => {
        fetch(`/api/admin/links/${url.slug}`, {
          method: "PATCH",
          body: JSON.stringify({ new_url: newUrl }),
          headers: {
            "content-type": "application/json",
          },
        })
          .then((response) => response.json())
          .then((response) => {
            if (response.success) {
              emitter.emit("fetchUrls");
              res();
            } else {
              rej(response.message);
            }
            editing = false;
          });
      }),
      {
        loading: `Editing short url with slug "${url.slug}"`,
        success: `Edited short url with slug "${url.slug}"`,
        error: (err) => `Error: ${err}`,
      }
    );
  }

  function edit() {
    newUrl = url.url;
    editing = true;
  }
</script>

<tr key={url.slug}>
  <td class="w-1/4 text-center">
    <!-- TODO Dynamically get URL -->
    <Clipboard text={`https://link.mikkel-t.com/${url.slug}`}>
      <span class="cursor-pointer">
        {url.slug}
        <div
          class="m-1 inline-flex h-6 w-6 rounded-md bg-dracula-blue p-1 hover:bg-dracula-blue-700"
        >
          <FaCopy />
        </div>
      </span>
    </Clipboard>
  </td>
  <td class="w-1/2 text-center">
    {#if editing}
      <textarea
        bind:value={newUrl}
        class="w-full rounded-md border border-white bg-transparent"
      />
    {:else}
      <a href={url.url} class="underline text-dracula-cyan-700 break-all"
        >{url.url}</a
      >
    {/if}
  </td>
  <td class="w-1/4 text-center">
    <button
      class="m-1 h-6 w-6 rounded-md bg-dracula-red p-1 hover:bg-dracula-red-500"
      on:click={deleteSlug}
    >
      <IoMdTrash />
    </button>
    {#if editing}
      <button
        class="m-1 h-6 w-6 rounded-md bg-dracula-green p-1 hover:bg-dracula-green-500"
        on:click={editUrl}
      >
        <FaCheck />
      </button>
    {:else}
      <button
        class="bg-dracula-orange-300 hover:bg-dracula-orange-400 p-1 rounded-md m-1 h-6 w-6"
        on:click={edit}
      >
        <FaPencilAlt />
      </button>
    {/if}
  </td>
</tr>
