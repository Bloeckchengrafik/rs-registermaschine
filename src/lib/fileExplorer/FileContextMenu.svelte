<script lang="ts">
    import {
        FileCode,
        FilePlus,
        FolderDashed,
        FolderPlus,
        TrashSimple,
    } from "phosphor-svelte";
    import Swal from "sweetalert2";
    import {createDir, writeFile, removeFile, removeDir} from "@tauri-apps/api/fs";

    export let fullFp: string;
    export let isFolder: boolean;

    let name = fullFp.split("/").pop()!.split("\\").pop();
    if (isFolder) {
        name = fullFp
            .substring(0, fullFp.length - 1)
            .split("/")
            .pop()!
            .split("\\")
            .pop();
    }
</script>

<div class="shadow bg-neutral rounded">
    <div class="flex items-center px-2 pt-2 space-x-2">
        {#if isFolder}
            <FolderDashed class="w-6 h-6" />
        {:else}
            <FileCode class="w-6 h-6" />
        {/if}
        <span class="text-md font-normal text-white">{name}</span>
    </div>
    <div class="my-2 bg-slate-400 h-[1px] w-full" />

    <div class="flex justify-center flex-col px-2 pb-2 space-y-1">
        {#if isFolder}
            <button
                class="flex items-center space-x-1 fpbutton"
                on:click={async () => {
                    // ask ther user for a name
                    // create a new file with that name
                    let answer = Swal.fire({
                        title: "Neue Datei",
                        input: "text",
                        inputPlaceholder: "Dateiname",
                        showCancelButton: true,
                        inputValidator: (value) => {
                            if (!value) {
                                return "Bitte gib einen Dateinamen ein";
                            }
                        },
                    });

                    if (!(await answer).isConfirmed) {
                        return;
                    }

                    let name = (await answer).value;
                    let fullName = fullFp + name;
                    
                    await writeFile({
                        path: fullName,
                        contents: "",
                    });
                }}
            >
                <FilePlus class="w-6 h-6" />
                <span class="text-md font-normal text-white">Neue Datei</span>
            </button>
            <button
                class="flex items-center space-x-1 fpbutton"
                on:click={async () => {
                    // ask ther user for a name
                    // create a new folder with that name
                    let answer = Swal.fire({
                        title: "Neuer Ordner",
                        input: "text",
                        inputPlaceholder: "Ordnername",
                        showCancelButton: true,
                        inputValidator: (value) => {
                            if (!value) {
                                return "Bitte gib einen Ordnernamen ein";
                            }
                        },
                    });

                    if (!(await answer).isConfirmed) {
                        return;
                    }

                    let name = (await answer).value;
                    let fullName = fullFp + name;

                    createDir(fullName);
                }}
            >
                <FolderPlus class="w-6 h-6" />
                <span class="text-md font-normal text-white">Neuer Ordner</span>
            </button>
        {/if}
        <button
            class="flex items-center space-x-1 fpbutton"
            on:click={async () => {
                // ask the user if they are sure
                // delete the file/folder
                let answer = Swal.fire({
                    title: "Bist du sicher?",
                    icon: "warning",
                    showCancelButton: true,
                    confirmButtonText: "Ja, löschen",
                    cancelButtonText: "Nein, abbrechen",
                });

                if (!(await answer).isConfirmed) {
                    return;
                }

                if (isFolder) {
                    await removeDir(fullFp, {recursive: true});
                } else {
                    await removeFile(fullFp);
                }
            }}
        >
            <TrashSimple class="w-6 h-6" />
            <span class="text-md font-normal text-white">Löschen</span>
        </button>
    </div>
</div>

<style>
    .fpbutton {
        width: 100%;
        border-radius: 0.25rem;
        transition: background-color 0.1s ease-in-out;
    }

    .fpbutton:hover {
        background-color: rgba(255, 255, 255, 0.1);
    }
</style>
