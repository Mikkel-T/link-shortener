<script>
  import { emitter } from "@event/event";
  import HeroiconsSolidChevronDown from "~icons/heroicons-solid/chevron-down";
  import HeroiconsSolidChevronRight from "~icons/heroicons-solid/chevron-right";
  import toast from "svelte-french-toast";

  let options = false;

  let newLink = {
    url: "",
  };

  let expire_enabled = {
    uses: false,
    time: false,
  };

  function submit() {
    toast.promise(
      new Promise((res, rej) => {
        let req_link = { url: newLink.url };
        if (options) {
          if (newLink["slug"]) req_link["slug"] = newLink["slug"];
          if (expire_enabled["uses"] && newLink["expires_uses"])
            req_link["expires_uses"] = newLink["expires_uses"];
          if (expire_enabled["time"] && newLink["expire_at"])
            req_link["expire_at"] = new Date(
              newLink["expire_at"]
            ).toISOString();
        }
        fetch("/api/admin/links", {
          method: "POST",
          body: JSON.stringify(req_link),
          headers: {
            "content-type": "application/json",
          },
        })
          .then((response) => response.json())
          .then((response) => {
            if (response.success) {
              newLink = {
                slug: "",
                url: "",
                expires_uses: null,
              };

              expire_enabled = {
                uses: false,
              };
              emitter.emit("fetchLinks");
              res(response.slug);
            } else {
              rej(response.message);
            }
          })
          .catch((e) => rej(e));
      }),
      {
        loading: `Creating short link`,
        success: (slug) => `Created short link with slug "${slug}"`,
        error: (err) => `Error creating short link: ${err}`,
      }
    );
  }
</script>

<form on:submit|preventDefault={submit}>
  <div class="m-auto mb-4 w-2/3 md:w-2/6">
    <div class="p-2">
      <label for="url">URL</label>
      <br />
      <input
        bind:value={newLink["url"]}
        name="url"
        class="w-full rounded-md border border-dracula-light bg-transparent p-2"
        type="url"
        required
      />
    </div>
    <button
      class="mx-auto block text-center"
      on:click={() => (options = !options)}
    >
      More options {#if options}
        <HeroiconsSolidChevronDown class="mr-1 inline h-5 w-5" />
      {:else}
        <HeroiconsSolidChevronRight class="mr-1 inline h-5 w-5" />
      {/if}
    </button>
    {#if options}
      <div class=" mx-auto w-1/2">
        <div>
          <label for="slug">Slug</label>
          <br />
          <input
            bind:value={newLink["slug"]}
            name="slug"
            class="w-full rounded-md border border-dracula-light bg-transparent p-2"
          />
        </div>
        <div>
          Expire
          <div>
            <input type="checkbox" bind:checked={expire_enabled["uses"]} />
            <span class:text-dracula-darker-600={!expire_enabled["uses"]}>
              after <input
                type="number"
                name="expires_uses"
                min="1"
                disabled={!expire_enabled["uses"]}
                bind:value={newLink["expires_uses"]}
                class="border-b {!expire_enabled['uses']
                  ? 'border-b-dracula-darker-600'
                  : 'border-b-dracula-light'} w-1/6 bg-transparent p-1"
              /> uses
            </span>
          </div>
          <div>
            <input type="checkbox" bind:checked={expire_enabled["time"]} />
            <span class:text-dracula-darker-600={!expire_enabled["time"]}>
              at date and time <input
                type="datetime-local"
                name="expire_at"
                disabled={!expire_enabled["time"]}
                bind:value={newLink["expire_at"]}
                class="rounded-md border bg-transparent {!expire_enabled['time']
                  ? 'border-dracula-darker-600'
                  : 'border-dracula-light'} "
              />
            </span>
          </div>
        </div>
      </div>
    {/if}
    <div class="p-2 text-center">
      <input
        type="submit"
        value="Create"
        class="cursor-pointer rounded-md bg-dracula-blue p-3 px-5 hover:bg-dracula-blue-700"
      />
    </div>
  </div>
</form>
