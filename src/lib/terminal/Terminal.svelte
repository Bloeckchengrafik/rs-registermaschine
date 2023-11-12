<script lang="ts">
    // Terminal with a simple log function

    import {onMount} from "svelte";
    import {globalLog} from "../../stores";
    import Registers from "./Registers.svelte";

    let entries: {
        text: string,
        level: "error" | "warn" | "info" | "debug" | "trace",
    }[] = [];
    let div: HTMLDivElement;

    function log(text: string, level: "error" | "warn" | "info" | "debug" | "trace" = "info") {
        entries = [...entries, {text, level}];
        setTimeout(() => div.scrollTo({top: div.scrollHeight, behavior: "smooth"}), 0);
    }

    onMount(async () => {
        log("Welcome to RM Editor", "info")
        $globalLog = log;
    })
</script>
<div class="flex flex-col h-full">
    <div>
        <Registers/>
    </div>
    <div class="my-1 w-full border-b border-gray-600"
    ></div>
    <div class="p-2 font-mono overflow-y-scroll" bind:this={div}>
        {#each entries as entry}
            <div class="flex items-center">
                <div class="w-2 h-2 rounded-full mr-2" class:bg-red-500={entry.level === "error"}
                     class:bg-yellow-500={entry.level === "warn"} class:bg-blue-500={entry.level === "info"}
                     class:bg-gray-500={entry.level === "debug"} class:bg-gray-300={entry.level === "trace"}/>
                <div class="text-sm">{entry.text}</div>
            </div>
        {/each}
    </div>
</div>