<script>
  import { nanoid } from "nanoid";
  import { emitter } from "@event/event";

  let newLink = {
    slug: "",
    url: "",
    expires_uses: null,
  };

  let expire_enabled = {
    uses: false,
  };

  function submit() {
    emitter.emit(
      "toast-promise",
      new Promise((res, rej) => {
        if (!newLink.slug) newLink.slug = nanoid(7);
        fetch("/api/admin/links", {
          method: "POST",
          body: JSON.stringify({
            ...newLink,
            expires_uses: expire_enabled["uses"]
              ? newLink["expires_uses"]
              : null,
          }),
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
              emitter.emit("fetchUrls");
              res();
            } else {
              rej(response.message);
            }
          })
          .catch((e) => rej(e));
      }),
      {
        loading: `Creating short url with slug "${newLink.slug}"`,
        success: `Created short url with slug "${newLink.slug}"`,
        error: (err) => `Error creating short url: ${err}`,
      }
    );
  }
</script>

<form on:submit|preventDefault={submit}>
  <div class="m-auto mb-4 w-2/3 md:w-2/6">
    <div class="p-2">
      <label for="slug">Slug</label>
      <br />
      <input
        bind:value={newLink["slug"]}
        name="slug"
        class="w-full rounded-md border border-dracula-light bg-transparent p-2"
      />
    </div>
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
    </div>
    <div class="p-2 text-center">
      <input
        type="submit"
        value="Create"
        class="cursor-pointer rounded-md bg-dracula-blue p-3 px-5 hover:bg-dracula-blue-700"
      />
    </div>
  </div>
</form>
