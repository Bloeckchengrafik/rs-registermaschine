<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { workspace } from "../../stores";
    import { onMount } from "svelte";
    import { fileAbsoluteToRelative, workspaceUrlToName } from "../../utils";
    import FileExplorer from "./FileExplorer.svelte";

    let files: string[] = [];

    function request() {
        invoke("list_files", { basepath: $workspace }).then((res) => {
            files = (res as string[]).map((x) =>  fileAbsoluteToRelative($workspace, x))
        });
    }

    request();

    onMount(() => {
        const timeout = setInterval(() => {
            request();
        }, 500);

        return () => clearInterval(timeout);
    });
</script>

<span class="text-xs ml-3 pt-2 block text-gray-400">
    File Explorer
</span>
<span class="text-md font-normal ml-3 pb-1 block text-white">
    {workspaceUrlToName($workspace)}
</span>
<FileExplorer values={files} currentBasepath=""/>
