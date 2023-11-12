<script lang="ts">
    import {CaretDown, CaretRight, FileCode} from "phosphor-svelte";
    import {currentTab, editorTabs, expandedFiles, workspace} from "../../stores";
    import {fpPos, makeSureEndsInSlash} from "../../utils";
    import FileContextMenu from "./FileContextMenu.svelte";

    export let values: string[];
    export let currentBasepath: string;

    function isFirstLevelFolder(value: string) {
        return value.endsWith("/") && value.indexOf("/") == value.length - 1;
    }

    function isInFolder(value: string) {
        return value.includes("/") || value.includes("\\");
    }

    function prepareRecursiveArgs(basepath: string) {
        // IMPORTANT only replace first occurence of basepath (do that by using the string after the length of the basepath)
        return {
            values: values
                .filter((x) => x.startsWith(basepath))
                .map((x) => x.substring(basepath.length)),
            currentBasepath: basepath,
        };
    }

    let showContextMenu = false;
    let contextMenuX = 0;
    let contextMenuY = 0;
    let contextMenuPath = "";
    let contextMenuIsFolder = false;
</script>

<div class="ml-1 pl-2" class:indicator-left={currentBasepath !== ""}>
    {#each values.filter((x) => isFirstLevelFolder(x)) as value}
        <button
            class="flex items-center space-x-1 w-full fpbutton"
            on:click={() => {
                let fullValue = currentBasepath + value;
                if ($expandedFiles.includes(fullValue)) {
                    $expandedFiles = $expandedFiles.filter(
                        (x) => x !== fullValue
                    );
                } else {
                    $expandedFiles = [...$expandedFiles, fullValue];
                }
            }}
            on:contextmenu|preventDefault={(event) => {
                contextMenuPath = makeSureEndsInSlash($workspace) + currentBasepath + value;
                contextMenuIsFolder = true;
                showContextMenu = true;
                contextMenuX = event.clientX;
                contextMenuY = event.clientY;
            }}
        >
            <div class="flex items-center space-x-1 my-sm">
                {#if $expandedFiles.includes(currentBasepath + value)}
                    <CaretDown class="w-4 h-4" />
                {:else}
                    <CaretRight class="w-4 h-4" />
                {/if}
                <span class="filename"
                    >{value.substring(0, value.length - 1)}</span
                >
            </div>
        </button>

        {#if $expandedFiles.includes(currentBasepath + value)}
            <svelte:self {...prepareRecursiveArgs(value)} />
        {/if}
    {/each}

    {#each values.filter((x) => !isFirstLevelFolder(x) && !isInFolder(x) && x) as value}
        <button class="flex items-center w-full space-x-1 my-sm fpbutton" on:click={() => {
            let fullFilePath = makeSureEndsInSlash($workspace) + currentBasepath + value;
            let filePathPosition = fpPos(fullFilePath, $editorTabs);
            
            if (filePathPosition === -1) {
                $editorTabs = [...$editorTabs, fullFilePath];
                $currentTab = $editorTabs.length - 1;
            } else {
                $currentTab = filePathPosition;
            }
        }}
        on:contextmenu|preventDefault={(event) => {
            contextMenuPath = makeSureEndsInSlash($workspace) + currentBasepath + value;
            contextMenuIsFolder = false;
            showContextMenu = true;
            contextMenuX = event.clientX;
            contextMenuY = event.clientY;
        }}>
            <FileCode class="w-4 h-4" />
            <span class="filename">{value}</span>
        </button>
    {/each}
</div>

{#if showContextMenu}
    <button class="absolute top-0 left-0 z-10 w-full h-full cursor-default" on:click={() => showContextMenu = false}>
        <div class="absolute"  style="left: {contextMenuX}px; top: {contextMenuY}px;">
            <FileContextMenu fullFp={contextMenuPath} isFolder={contextMenuIsFolder} />
        </div>
    </button>
{/if}

<style>
    .filename {
        font-size: 0.9rem;
        color: #ccc;
    }

    .my-sm {
        padding-top: 1px;
        padding-bottom: 1px;
    }

    .indicator-left {
        border-left: 1px solid #aaa;
    }

    .fpbutton {
        background-color: transparent;
        border: none;
        outline: none;
        cursor: pointer;
    }

    .fpbutton:hover {
        background-color: #33333369;
    }
</style>
