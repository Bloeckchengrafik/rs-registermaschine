<script lang="ts">
    import {workspace, currentTab, editorTabs} from "../stores";
    import {GitBranch, GitCommit} from "phosphor-svelte";
    import {fileAbsoluteToRelative, workspaceUrlToName} from "../utils";

</script>

<div class="w-full h-full flex gap-1 items-center justify-between">
    <div class="text-xs breadcrumbs pl-1">
        <ul>
            {#if $workspace === null }
                <li><div class="w-2 h-2 rounded-sm bg-secondary mx-2"></div> Unknown</li>
            {:else}
                <li><div class="w-2 h-2 rounded-sm bg-accent mx-2"></div> {workspaceUrlToName($workspace)}</li>
                {#if $currentTab != -1}
                    {#each fileAbsoluteToRelative($workspace, $editorTabs[$currentTab]).split("/") as data}
                        <li>{data}</li>
                    {/each}
                {/if}
            {/if}
        </ul>
    </div>

    <!-- Show version & git branch -->
    <div class="text-xs mr-2">
        <span class="mr-1 inline-flex items-center gap-1"><GitCommit /> v1.0</span>
        <span class="mr-1 inline-flex items-center gap-1"><GitBranch /> main</span>
    </div>
</div>