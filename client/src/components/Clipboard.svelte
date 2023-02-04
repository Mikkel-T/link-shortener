<script>
  import copy from "copy-to-clipboard";
  import toast from "svelte-french-toast";
  export let text;
  export let message = "Copied link";
  function onCopy() {
    copy(text);
    toast.success(message);
  }
</script>

<button on:click={onCopy} class="cursor-pointer select-none">
  <slot />
</button>
