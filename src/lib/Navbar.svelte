<script>
    import { ArrowsInSimple, FrameCorners, X } from "phosphor-svelte";
    import { appWindow } from '@tauri-apps/api/window'
</script>

<div
    class="w-full h-full flex items-center px-2 justify-between"
    data-tauri-drag-region
>
    <div>
        <span class="text-xxl font-bold">RM Editor</span>
    </div>
    <div class="flex gap-2">
        <button on:click={appWindow.minimize}>
            <ArrowsInSimple />
        </button>
        <button on:click={appWindow.maximize}>
            <FrameCorners />
        </button>
        <button on:click={appWindow.close}>
            <X />
        </button>
    </div>
</div>
