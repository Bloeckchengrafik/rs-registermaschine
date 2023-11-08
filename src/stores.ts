import {writable} from "svelte/store";

export const currentFile = writable("/projects/10_Rust/rs-registermaschine/demo_files/test.rm");
export const workspace = writable("/projects/10_Rust/rs-registermaschine/demo_files");

export const editorTabs = writable([
    "/projects/10_Rust/rs-registermaschine/demo_files/test.rm",
    "/projects/10_Rust/rs-registermaschine/demo_files/a.rm",
    "/projects/10_Rust/rs-registermaschine/demo_files/b.rm"
]);