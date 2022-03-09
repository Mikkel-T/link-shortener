<script>
  import { nanoid } from 'nanoid';
  import { emitter } from '@event/event';

  let newLink = {
    slug: '',
    url: '',
  };

  function submit() {
    emitter.emit(
      'toast-promise',
      new Promise((res, rej) => {
        if (!newLink.slug) newLink.slug = nanoid(7);
        fetch('/api/admin/links', {
          method: 'POST',
          body: JSON.stringify(newLink),
          headers: {
            'content-type': 'application/json',
          },
        })
          .then((response) => response.json())
          .then((response) => {
            if (response.success) {
              newLink = {
                slug: '',
                url: '',
              };

              emitter.emit('fetchUrls');
              res();
            } else {
              rej(response.message);
            }
          });
      }),
      {
        loading: `Creating short url with slug "${newLink.slug}"`,
        success: `Created short url with slug "${newLink.slug}"`,
        error: (err) => `Error: ${err}`,
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
        bind:value={newLink['slug']}
        name="slug"
        class="w-full rounded-md border border-dracula-light bg-transparent p-2"
      />
    </div>
    <div class="p-2">
      <label for="url">URL</label>
      <br />
      <input
        bind:value={newLink['url']}
        name="url"
        class="w-full rounded-md border border-dracula-light bg-transparent p-2"
        type="url"
        required
      />
    </div>
    <div class="p-2 text-center">
      <input
        type="submit"
        value="Create"
        class="cursor-pointer rounded-md bg-dracula-blue p-3 px-5"
      />
    </div>
  </div>
</form>
