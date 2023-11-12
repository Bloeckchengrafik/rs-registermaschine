<script lang="ts">
    import {currentTab, editorApiRef, editorTabs, workspace} from "../../stores";
    import { fileAbsoluteToRelative } from "../../utils";
    import Editor from "./Editor.svelte";

    $: if ($editorTabs.length === 0) {
        $currentTab = -1;
    }

    let editor: Editor | null = null;

    export let w = 0;
    export let h = 0;

    editorApiRef.set({
        showFile(path: string, line: number): void {
            const index = $editorTabs.findIndex((tab) => tab === path);
            if (index === -1) {
                $editorTabs.push(path);
                $editorTabs = [...$editorTabs];
                $currentTab = $editorTabs.length - 1;
            } else {
                $currentTab = index;
            }

            setTimeout(() => {
                editor?.highlightSingleLine(line);
            }, 100);
        }
    })
</script>

<div class="flex flex-col w-full h-full">
    <div class="tabs w-full border-b border-b-neutral">
        {#each $editorTabs as tab, i}
            <button
                class="tab"
                class:tab-active={i === $currentTab}
                on:click={() => ($currentTab = i)}
            >
                {fileAbsoluteToRelative($workspace, tab)}
                <button
                    class="ml-1 show-on-hover-parent"
                    on:click|stopPropagation={() => {
                        $editorTabs.splice(i, 1);
                        $editorTabs = [...$editorTabs];
                        if (i === $currentTab) {
                            $currentTab = 0;
                        } else if (i < $currentTab) {
                            $currentTab--;
                        }

                        if ($editorTabs.length === 0) {
                            $currentTab = -1;
                        }
                    }}
                    >&times;
                </button>
            </button>
        {/each}
        {#if $editorTabs.length === 0}
            <div class="tab pointer-events-none">No files open</div>
        {/if}
    </div>
    <div class="flex-grow">
        {#if $currentTab !== -1}
            <div class="flex flex-col items-center justify-center h-full">
                {#key $editorTabs[$currentTab]}
                    <Editor
                        file={$editorTabs[$currentTab]}
                        bind:this={editor}
                        {w}
                        h={Math.max(0, h - 33)}
                    />
                {/key}
            </div>
        {:else}
            <div class="flex flex-col items-center justify-center h-full">
                <div class="prose">
                    <h2>Willkommen im Registermaschinen-Editor!</h2>
                    <span>Um zu beginnen:</span>
                    <ul>
                        <li>Erstelle ein neues Projekt im Menu</li>
                        <li>Öffne eine Datei im Editor</li>
                        <li>Starte den Simulator</li>
                    </ul>
                </div>
            </div>
        {/if}
    </div>
</div>

<style>
    .show-on-hover-parent {
        opacity: 0;
        transition: opacity 0.2s;
    }

    *:hover > .show-on-hover-parent {
        opacity: 1;
    }
</style>
