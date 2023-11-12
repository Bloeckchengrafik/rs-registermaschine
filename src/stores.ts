import {get, writable} from "svelte/store";
import {invoke} from "@tauri-apps/api/tauri";
import type {EditorApi} from "./types";

export const workspace = writable<string>("");

invoke("get_workspace").then((newWorkspace) => {
    if (newWorkspace) {
        console.log("Setting workspace to " + newWorkspace)
        workspace.set(newWorkspace as string)
    }
})

export const editorTabs = writable<string[]>([]);
export const currentTab = writable<number>(-1);
export const expandedFiles = writable<string[]>([]);
export const editorApiRef = writable<EditorApi>({
    showFile: (path: string, line: number) => {
        console.log("showFile not implemented yet");
    }
});
export const currentAccumulator = writable<number>(0);
export const currentUserRegisters = writable<number[]>([0]);
export const currentSystemRegisters = writable<number[]>([]);

const defaultGlobalLog = (
    msg: string,
    level: "error" | "warn" | "info" | "debug" | "trace",
) => {
}

export const globalLog = writable<typeof defaultGlobalLog>(defaultGlobalLog);

export const cancelExecution = writable<() => void>(() => {
});

export function currentOpenFilePath(): String | null {
    if (get(currentTab) === -1) {
        return null;
    }

    return get(editorTabs)[get(currentTab)];
}