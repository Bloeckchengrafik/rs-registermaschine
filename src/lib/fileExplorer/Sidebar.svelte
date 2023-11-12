<script lang="ts">
    import {invoke} from "@tauri-apps/api/tauri";
    import {workspace} from "../../stores";
    import {onMount} from "svelte";
    import {fileAbsoluteToRelative, makeSureEndsInSlash, workspaceUrlToName} from "../../utils";
    import FileExplorer from "./FileExplorer.svelte";
    import FileContextMenu from "./FileContextMenu.svelte";

    let files: string[] = [];

    function request() {
        invoke("list_files", {basepath: $workspace}).then((res) => {
            files = (res as string[]).map((x) => fileAbsoluteToRelative($workspace, x))
        });
    }

    request();

    onMount(() => {
        const timeout = setInterval(() => {
            request();
        }, 500);

        return () => clearInterval(timeout);
    });

    let showContextMenu = false;
    let contextMenuX = 0;
    let contextMenuY = 0;
    let contextMenuPath = "";
    let contextMenuIsFolder = false;
</script>

<span class="text-xs ml-3 pt-2 block text-gray-400">
    File Explorer
</span>
<button class="text-md font-normal ml-3 pb-1 block text-white" on:contextmenu|preventDefault={(event) => {
            contextMenuPath = makeSureEndsInSlash($workspace) + contextMenuPath;
            contextMenuIsFolder = true;
            showContextMenu = true;
            contextMenuX = event.clientX;
            contextMenuY = event.clientY;
        }}>
    {workspaceUrlToName($workspace)}
</button>
<FileExplorer values={files} currentBasepath=""/>
{#if showContextMenu}
    <button class="absolute top-0 left-0 z-10 w-full h-full cursor-default" on:click={() => showContextMenu = false}>
        <div class="absolute"  style="left: {contextMenuX}px; top: {contextMenuY}px;">
            <FileContextMenu isRoot={true} fullFp={contextMenuPath} isFolder={contextMenuIsFolder} />
        </div>
    </button>
{/if}