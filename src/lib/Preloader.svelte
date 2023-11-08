<script lang="ts">
    import {onMount} from "svelte";
    import {fade} from "svelte/transition";

    export let loading = true

    onMount(() => setTimeout(() => loading = false, 3000))
</script>

{#if loading}
    <div class="absolute w-full h-full z-10 flex justify-center items-center flex-col bg-base-100"
         transition:fade>
        <div class="prose">
            <h1>Registermaschine-Editor l&auml;dt</h1>
        </div>
        <span class="loading loading-infinity loading-lg"></span>
    </div>
{/if}
