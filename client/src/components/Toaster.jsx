import toast, { Toaster } from "react-hot-toast";

import { emitter } from "@event/event";

export default function Toast() {
  emitter.on("toast", (message, options) => {
    toast(message, options);
  });

  emitter.on("toast-promise", (promise, messages, options) => {
    toast.promise(promise, messages, options);
  });

  return <Toaster position="bottom-right" />;
}
