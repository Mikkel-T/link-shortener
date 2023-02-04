<script>
  import Clipboard from "@components/Clipboard.svelte";
  import { emitter } from "@event/event";
  import toast from "svelte-french-toast";
  import IcRoundCopyAll from "~icons/ic/round-copy-all";
  import IcRoundDelete from "~icons/ic/round-delete";
  import IcRoundCheck from "~icons/ic/round-check";
  import IcRoundEdit from "~icons/ic/round-edit";
  export let link;

  let editing = false;
  let newLink = link;

  function deleteSlug() {
    toast.promise(
      new Promise((res, rej) => {
        fetch(`/api/admin/links/${link.slug}`, {
          method: "DELETE",
        })
          .then((response) => response.json())
          .then((response) => {
            if (response.success) {
              emitter.emit("fetchLinks");
              res();
            } else {
              rej(response.message);
            }
          })
          .catch((e) => rej(e));
      }),
      {
        loading: `Deleting short link with slug "${link.slug}"`,
        success: `Deleted short link with slug "${link.slug}"`,
        error: (err) => `Error deleting short link: ${err}`,
      }
    );
  }

  function editUrl() {
    toast.promise(
      new Promise((res, rej) => {
        fetch(`/api/admin/links/${link.slug}`, {
          method: "PATCH",
          body: JSON.stringify(newLink),
          headers: {
            "content-type": "application/json",
          },
        })
          .then((response) => response.json())
          .then((response) => {
            if (response.success) {
              emitter.emit("fetchLinks");
              res();
            } else {
              rej(response.message);
            }
            editing = false;
          })
          .catch((e) => rej(e));
      }),
      {
        loading: `Editing short link with slug "${link.slug}"`,
        success: `Edited short link with slug "${link.slug}"`,
        error: (err) => `Error editing short link: ${err}`,
      }
    );
  }

  function edit() {
    newLink = link;
    editing = true;
  }
</script>

<tr key={link.slug}>
  <td class="w-1/4 text-center">
    <!-- TODO Dynamically get URL -->
    <Clipboard text={`https://link.mikkel-t.com/${link.slug}`}>
      <span class="cursor-pointer">
        {link.slug}
        <div
          class="m-1 inline-flex h-6 w-6 rounded-md bg-dracula-blue p-1 hover:bg-dracula-blue-700"
        >
          <IcRoundCopyAll class="h-fit w-fit" />
        </div>
      </span>
    </Clipboard>
  </td>
  <td class="w-1/2 text-center">
    {#if editing}
      <textarea
        bind:value={newLink.url}
        class="w-full rounded-md border border-white bg-transparent"
      />
    {:else}
      <a href={link.url} class="break-all text-dracula-cyan-700 underline"
        >{link.url}</a
      >
    {/if}
  </td>
  <td class="w-1/12 text-center">
    {#if link.expires_uses || link.expire_at}
      Yes
    {:else}
      No
    {/if}
  </td>
  <td class="w-1/6 text-center">
    <button
      class="m-1 h-6 w-6 rounded-md bg-dracula-red p-1 hover:bg-dracula-red-500"
      on:click={deleteSlug}
    >
      <IcRoundDelete class="h-fit w-fit" />
    </button>
    {#if editing}
      <button
        class="m-1 h-6 w-6 rounded-md bg-dracula-green-700 p-1 hover:bg-dracula-green-800"
        on:click={editUrl}
      >
        <IcRoundCheck class="h-fit w-fit" />
      </button>
    {:else}
      <button
        class="m-1 h-6 w-6 rounded-md bg-dracula-orange-300 p-1 hover:bg-dracula-orange-400"
        on:click={edit}
      >
        <IcRoundEdit class="h-fit w-fit" />
      </button>
    {/if}
  </td>
</tr>
