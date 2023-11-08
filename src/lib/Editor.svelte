<script lang="ts">
    import Monaco from "./Monaco.svelte";
    import {onMount} from "svelte";

    export let file: string;
    let monaco: Monaco;
    export let w = 0;
    export let h = 0;

    let content = `
loop:
    LOAD 1
    ADD #1
    STORE 1

    LOAD 1
    JNZERO loop
    END
`

    onMount(() => {
        // monaco.loadCode(content, "mySpecialLanguage");
    })
</script>

<Monaco bind:this={monaco} w={w} {h} />