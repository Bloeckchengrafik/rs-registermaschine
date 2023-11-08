import {writable} from "svelte/store";

export const currentFile = writable("C:\\Users\\chris\\Documents\\rs-registermaschine\\demo_files\\test.rm");
export const workspace = writable("C:\\Users\\chris\\Documents\\rs-registermaschine\\demo_files");

export const editorTabs = writable<string[]>([]);
export const currentTab = writable<number>(-1);
export const expandedFiles = writable<string[]>([]);