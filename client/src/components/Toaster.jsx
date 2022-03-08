import toast, { Toaster } from 'react-hot-toast';

import { emitter } from '@event/event';

export default function Toast() {
  emitter.on('toast', (message, options) => {
    toast(message, options);
  });

  emitter.on('toast-promise', (promise, messages, options) => {
    toast.promise(promise, messages, options);
  });

  return <Toaster position="bottom-right" />;
}
/*  function editUrl() {
    toast.promise(
      new Promise<void>((res, rej) => {
        (async () => {
          const { error } = await supabase
            .from('urls')
            .update({ slug: url.slug, url: newUrl })
            .eq('slug', url.slug);

          if (!error) {
            setEditing(false);
            fetchUrls();
            res();
          } else {
            rej(error.message);
          }
        })();
      }),
      {
        loading: 'Editing short url',
        success: 'Edited short url',
        error: (err) => `Error: ${err}`,
      },
      {
        success: {
          icon: <PencilIcon className="h-5 w-5 text-green-600" />,
        },
      }
    );
  }
*/
