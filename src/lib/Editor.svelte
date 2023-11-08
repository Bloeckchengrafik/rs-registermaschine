<script lang="ts">
    import { onMount } from "svelte";
    import { readTextFile, writeTextFile } from "@tauri-apps/api/fs";
    import Monaco from "./Monaco.svelte";

    export let file: string;
    export let w = 0;
    export let h = 0;

    let text = ``;
    let loaded = false;
    onMount(() => {
        readTextFile(file).then((t) => {
            text = t;
            loaded = true;
        });
    });

    let timeout: any = null;
    function save(data: string) {
        if (timeout) {
            clearTimeout(timeout);
        }

        timeout = setTimeout(() => {
            timeout = null;
            writeTextFile(file, data);
            console.log("Saved");
        }, 500);
    }

    $: if (loaded) save(text);
</script>

{#if loaded}
    <Monaco {w} {h} bind:text />
{/if}
