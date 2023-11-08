<script lang="ts">
    import {onDestroy, onMount} from 'svelte'

    // @ts-ignore
    import * as monaco from 'monaco-editor';
    import EditorWorker from 'monaco-editor/esm/vs/editor/editor.worker?worker';
    import JsonWorker from 'monaco-editor/esm/vs/language/json/json.worker?worker';
    import CssWorker from 'monaco-editor/esm/vs/language/css/css.worker?worker';
    import HtmlWorker from 'monaco-editor/esm/vs/language/html/html.worker?worker';
    import TsWorker from 'monaco-editor/esm/vs/language/typescript/ts.worker?worker';

    let editorElement: HTMLDivElement;
    let wrapperElement: HTMLDivElement;
    let editor: monaco.editor.IStandaloneCodeEditor;
    let model: monaco.editor.ITextModel;

    export let w = 0;
    export let h = 0;

    export function loadCode(code: string, language: string) {
        model = monaco.editor.createModel(code, language);
        console.log(model)
        editor.setModel(model);
    }

    onMount(async () => {
        self.MonacoEnvironment = {
            getWorker: function (_: any, label: string) {
                if (label === 'json') {
                    return new JsonWorker();
                }
                if (label === 'css' || label === 'scss' || label === 'less') {
                    return new CssWorker();
                }
                if (label === 'html' || label === 'handlebars' || label === 'razor') {
                    return new HtmlWorker();
                }
                if (label === 'typescript' || label === 'javascript') {
                    return new TsWorker();
                }
                return new EditorWorker();
            }
        };

        monaco.languages.register({id: "mySpecialLanguage"});

        // Register a tokens provider for the language
        monaco.languages.setMonarchTokensProvider("mySpecialLanguage", {
            // Set defaultToken to invalid to see what you do not tokenize yet
            defaultToken: 'invalid',

            ignoreCase: true,

            keywords: ["LOAD", "STORE", "ADD", "SUB", "MUL", "DIV", "GOTO", "JNZERO", "JZERO", "END"],

            // we include these common regular expressions
            symbols: /[=!~?:&|+\-*\/^%]+/,

            // The main tokenizer for our languages
            tokenizer: {
                root: [
                    // identifiers and keywords
                    [/[a-z_$][\w$]*:/, "label"],

                    [/[a-z_$][\w$]*/, {
                        cases: {
                            '@keywords': 'keyword',
                            '@default': 'label.sub'
                        }
                    }],

                    // whitespace
                    {include: '@whitespace'},

                    // delimiters and operators
                    [/[{}()\[\]]/, '@brackets'],


                    // numbers
                    [/\*\d+/, 'number.ptr'],
                    [/#-?0[bB][0-1][01_]*/, 'number.hash.bin'],
                    [/#-?\d[\d_]*/, 'number.lit'],
                    [/\d[\d_]*/, 'number'],

                    // delimiter: after number because of .\d floats
                    [/[,.]/, 'delimiter'],

                    [/^\s*#\s*include/, {token: "keyword.directive.include", next: "@include"}],
                    [/^\s*#\s*define/, {token: "keyword.directive.define", next: "@define"}],

                    // ARM comments
                    [/#(.*)/, 'comment'],

                ],

                whitespace: [
                    [/[ \t\r\n]+/, 'white'],
                ],

                include: [
                    [
                        /(\s*)([^"]*)/,
                        [
                            "",
                            {token: "string.include.identifier", next: "@pop"}
                        ]
                    ]
                ],

                define: [
                    [
                        /(\s*)(\S*)(\s*)(\S*)/,
                        [
                            "",
                            "string.define.identifier",
                            "",
                            {token: "string", next: "@pop"}
                        ]
                    ]
                ],
            }
        });

        // Define a new theme that contains only rules that match this language
        monaco.editor.defineTheme("myCoolTheme", {
            base: "vs-dark",
            inherit: true,
            rules: [
                {token: "label", foreground: "FFA500"},
                {token: "label.sub", foreground: "c17d00"},
                {token: "number.ptr", foreground: "8bc964"},
                {token: "number.lit", foreground: "c9c164"},
                {token: "number", foreground: "64c99d"},
                {token: "string.define.identifier", foreground: "64abc9"},
            ],
            colors: {
                "editor.background": "#1d232a",
            },
        });


        editor = monaco.editor.create(editorElement, {
            automaticLayout: true,
            theme: 'myCoolTheme',
            language: 'mySpecialLanguage',
        });

        loadCode(`#include test.rm
#define test 1
loop:
    LOAD 1
    ADD #1
    STORE *1

    LOAD 1
    JNZERO loop
    END
`, 'mySpecialLanguage')
    });

    onDestroy(() => {
        monaco?.editor.getModels().forEach((model) => model.dispose());
        editor?.dispose();
    });

    $:{
        if (editor && wrapperElement) {
            editor.layout({width: w, height: h});
        }
    }
</script>

<div class="flex flex-col flex-grow w-full h-full" bind:this={wrapperElement}>
    <div class="flex-grow" bind:this={editorElement}/>
</div>