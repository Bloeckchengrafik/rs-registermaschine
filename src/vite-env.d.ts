/// <reference types="svelte" />
/// <reference types="vite/client" />

// define window.MonacoEnvironment for ts
import type * as monaco from 'monaco-editor/esm/vs/editor/editor.api';
declare global {
    interface Window {
        MonacoEnvironment: monaco.MonacoEnvironment;
    }
}

declare module 'svelte-split-pane' {
    export default class VSplitPane {
        constructor(options: any);
    }
}