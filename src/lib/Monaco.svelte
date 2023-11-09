<script lang="ts">
    import { onDestroy, onMount } from "svelte";

    // @ts-ignore
    import * as monaco from "monaco-editor";
    import EditorWorker from "monaco-editor/esm/vs/editor/editor.worker?worker";
    import JsonWorker from "monaco-editor/esm/vs/language/json/json.worker?worker";
    import CssWorker from "monaco-editor/esm/vs/language/css/css.worker?worker";
    import HtmlWorker from "monaco-editor/esm/vs/language/html/html.worker?worker";
    import TsWorker from "monaco-editor/esm/vs/language/typescript/ts.worker?worker";

    let editorElement: HTMLDivElement;
    let wrapperElement: HTMLDivElement;
    let editor: monaco.editor.IStandaloneCodeEditor;
    let model: monaco.editor.ITextModel;

    export let w = 0;
    export let h = 0;
    export let text = "";

    let lastTextForeignSet = 0;
    let lastTextLocalSet = 0;

    export function loadCode(code: string, language: string) {
        model = monaco.editor.createModel(code, language);
        console.log(model);
        editor.setModel(model);
    }

    let disposes: (() => void)[] = [];

    onMount(async () => {
        self.MonacoEnvironment = {
            getWorker: function (_: any, label: string) {
                if (label === "json") {
                    return new JsonWorker();
                }
                if (label === "css" || label === "scss" || label === "less") {
                    return new CssWorker();
                }
                if (
                    label === "html" ||
                    label === "handlebars" ||
                    label === "razor"
                ) {
                    return new HtmlWorker();
                }
                if (label === "typescript" || label === "javascript") {
                    return new TsWorker();
                }
                return new EditorWorker();
            },
        };

        monaco.languages.register({ id: "mySpecialLanguage" });

        // Register a tokens provider for the language
        let { dispose: disposeTknProvider } = monaco.languages.setMonarchTokensProvider("mySpecialLanguage", {
            // Set defaultToken to invalid to see what you do not tokenize yet
            defaultToken: "invalid",

            ignoreCase: true,

            keywords: [
                "LOAD",
                "STORE",
                "ADD",
                "SUB",
                "MUL",
                "DIV",
                "GOTO",
                "JNZERO",
                "JZERO",
                "END",
            ],

            // we include these common regular expressions
            symbols: /[=!~?:&|+\-*\/^%]+/,

            // The main tokenizer for our languages
            tokenizer: {
                root: [
                    // identifiers and keywords
                    [/[a-z_$][\w$]*:/, "label"],

                    [
                        /[a-z_$][\w$]*/,
                        {
                            cases: {
                                "@keywords": "keyword",
                                "@default": "label.sub",
                            },
                        },
                    ],

                    // whitespace
                    { include: "@whitespace" },

                    // delimiters and operators
                    [/[{}()\[\]]/, "@brackets"],

                    // numbers
                    [/\*\d+/, "number.ptr"],
                    [/#-?0[bB][0-1][01_]*/, "number.hash.bin"],
                    [/#-?\d[\d_]*/, "number.lit"],
                    [/\d[\d_]*/, "number"],

                    // delimiter: after number because of .\d floats
                    [/[,.]/, "delimiter"],

                    [
                        /^\s*#\s*include/,
                        {
                            token: "keyword.directive.include",
                            next: "@include",
                        },
                    ],
                    [
                        /^\s*#\s*define/,
                        {
                            token: "keyword.directive.define",
                            next: "@define",
                        },
                    ],

                    // ARM comments
                    [/#(.*)/, "comment"],
                ],

                whitespace: [[/[ \t\r\n]+/, "white"]],

                include: [
                    [
                        /(\s*)([^"]*)/,
                        [
                            "",
                            {
                                token: "string.include.identifier",
                                next: "@pop",
                            },
                        ],
                    ],
                ],

                define: [
                    [
                        /(\s*)(\S*)(\s*)(\S*)/,
                        [
                            "",
                            "string.define.identifier",
                            "",
                            { token: "string", next: "@pop" },
                        ],
                    ],
                ],
            },
        });

        let { dispose: disposeComplItemProvider } =monaco.languages.registerCompletionItemProvider("mySpecialLanguage", {
            provideCompletionItems: (model, position) => {
                let word = model.getWordUntilPosition(position);
                let fullLine = model.getLineContent(position.lineNumber);
                let fullText = model.getValue();
                let range = {
                    startLineNumber: position.lineNumber,
                    endLineNumber: position.lineNumber,
                    startColumn: word.startColumn,
                    endColumn: word.endColumn,
                };

                function generateSuggestionForKeywords(keywords: string[]) {
                    // A keyword can be suggested if there was either the start of a new line or a : and then an arbitrary amount of whitespace before the cursor

                    let lineUntilWordBeginning = fullLine.substring(
                        0,
                        word.startColumn - 1
                    );
                    let lineUntilWordBeginningTrimmed =
                        lineUntilWordBeginning.trim();
                    if (
                        lineUntilWordBeginningTrimmed.length > 0 &&
                        !lineUntilWordBeginningTrimmed.endsWith(":")
                    ) {
                        return [];
                    }

                    return keywords.map((keyword) => {
                        return {
                            label: keyword,
                            kind: monaco.languages.CompletionItemKind.Function,
                            insertText: keyword,
                            range: range,
                        };
                    });
                }

                function generateLabelSuggestions() {
                    let labels = fullText.match(/[a-z_$][\w$]*:/g);
                    if (!labels) {
                        return [];
                    }

                    // Left of a label, there has to be a [GOTO, JZERO, JNZERO]
                    let lineUntilWordBeginning = fullLine.substring(
                        0,
                        word.startColumn - 1
                    );
                    let lineUntilWordBeginningTrimmed =
                        lineUntilWordBeginning.trim();
                    if (
                        !lineUntilWordBeginningTrimmed.endsWith("GOTO") &&
                        !lineUntilWordBeginningTrimmed.endsWith("JZERO") &&
                        !lineUntilWordBeginningTrimmed.endsWith("JNZERO")
                    ) {
                        return [];
                    }

                    return labels.map((label) => {
                        return {
                            label: label.substring(0, label.length - 1),
                            kind: monaco.languages.CompletionItemKind.Field,
                            insertText: label.substring(0, label.length - 1),
                            range: range,
                        };
                    });
                }

                function generateDefineSuggestions() {
                    let defines = fullText.match(/#define\s+[a-z_$][\w$]*/g);
                    if (!defines) {
                        return [];
                    }

                    // Left of a defined value, there has to be any token except END
                    let lineUntilWordBeginning = fullLine.substring(
                        0,
                        word.startColumn - 1
                    );
                    let lineUntilWordBeginningTrimmed =
                        lineUntilWordBeginning.trim();
                    const tokens = [
                        "LOAD",
                        "STORE",
                        "ADD",
                        "SUB",
                        "MUL",
                        "DIV",
                        "GOTO",
                        "JNZERO",
                        "JZERO",
                    ];
                    if (
                        lineUntilWordBeginningTrimmed.length === 0 ||
                        !tokens.some((token) =>
                            lineUntilWordBeginningTrimmed
                                .toLowerCase()
                                .endsWith(token.toLowerCase())
                        )
                    ) {
                        console.log(
                            lineUntilWordBeginningTrimmed,
                            tokens.some((token) =>
                                lineUntilWordBeginningTrimmed
                                    .toLowerCase()
                                    .endsWith(token.toLowerCase())
                            )
                        );
                        return [];
                    }

                    return defines.map((define) => {
                        return {
                            label: "Defined Value: " + define.substring(8),
                            kind: monaco.languages.CompletionItemKind.Field,
                            insertText: define.substring(8),
                            range: range,
                        };
                    });
                }

                let suggestions = [
                    ...generateSuggestionForKeywords([
                        "LOAD",
                        "STORE",
                        "ADD",
                        "SUB",
                        "MUL",
                        "DIV",
                        "GOTO",
                        "JNZERO",
                        "JZERO",
                        "END",
                    ]),
                    ...generateLabelSuggestions(),
                    ...generateDefineSuggestions(),
                ];
                return { suggestions: suggestions };
            },
        });

        let {dispose: disposeHvrProvider} = monaco.languages.registerHoverProvider("mySpecialLanguage", {
            provideHover: function (model, position) {
                // Hovering over a keyword shows the description of the keyword
                let word = model.getWordAtPosition(position);
                if (!word) {
                    return null;
                }
                let keywords: {
                    [key: string]: string;
                } = {
                    LOAD: "Lädt den Wert aus dem Parameter in den Akkumulator",
                    STORE: "Speichert den Wert aus dem Akkumulator in den Parameter",
                    ADD: "Addiert den Wert aus dem Parameter zum Akkumulator",
                    SUB: "Subtrahiert den Wert aus dem Parameter vom Akkumulator. Der Akkumulator wird auf 0 gesetzt, wenn der Wert negativ ist",
                    MUL: "Multipliziert den Wert aus dem Parameter mit dem Akkumulator",
                    DIV: "Dividiert den Wert aus dem Parameter durch den Akkumulator",
                    GOTO: "Springt zu dem Label",
                    JZERO: "Springt zu dem Label, wenn der Akkumulator 0 ist",
                    JNZERO: "Springt zu dem Label, wenn der Akkumulator nicht 0 ist",
                    END: "Beendet das Programm",
                };

                if (Object.keys(keywords).includes(word.word)) {
                    console.log(word.word);
                    return {
                        range: new monaco.Range(
                            position.lineNumber,
                            word.startColumn,
                            position.lineNumber,
                            word.endColumn
                        ),
                        contents: [
                            { value: "**Beschreibung des Keywords**" },
                            { value: keywords[word.word] },
                        ],
                    };
                }

                // Hovering over a define shows the value of the define
                let defines = model.findMatches(
                    "#define " + word.word,
                    true,
                    true,
                    false,
                    null,
                    true
                );
                if (defines.length > 0) {
                    let define = defines[0];
                    return {
                        range: new monaco.Range(
                            define.range.startLineNumber,
                            define.range.startColumn,
                            define.range.endLineNumber,
                            define.range.endColumn
                        ),
                        contents: [
                            { value: "**Define**" },
                            {
                                value:
                                    "Define `" +
                                    word.word +
                                    "` in Zeile " +
                                    define.range.startLineNumber +
                                    ":\n\n`" +
                                    model.getLineContent(
                                        define.range.startLineNumber
                                    ) +
                                    "`",
                            },
                        ],
                    };
                }

                // Hovering over a label shows the line of the label
                let labels = model.findMatches(
                    word.word + ":",
                    true,
                    true,
                    false,
                    null,
                    true
                );
                if (labels.length > 0) {
                    let label = labels[0];
                    return {
                        range: new monaco.Range(
                            label.range.startLineNumber,
                            label.range.startColumn,
                            label.range.endLineNumber,
                            label.range.endColumn
                        ),
                        contents: [
                            { value: "**Label**" },
                            {
                                value:
                                    "Label `" +
                                    word.word +
                                    "` in Zeile " +
                                    label.range.startLineNumber +
                                    ":\n\n`" +
                                    model.getLineContent(
                                        label.range.startLineNumber
                                    ) +
                                    "`",
                            },
                        ],
                    };
                }

                // Hovering over #define shows description of #define
                if (word.word === "define") {
                    return {
                        range: new monaco.Range(
                            position.lineNumber,
                            word.startColumn,
                            position.lineNumber,
                            word.endColumn
                        ),
                        contents: [
                            { value: "**Compiler-Direktive define**" },
                            {
                                value: "Mit der Compiler-Direktive `#define` können Werte definiert werden, die dann im Code verwendet werden können. Dies ist besonders nützlich, wenn ein Wert mehrmals verwendet wird und sich dieser Wert ändern kann. So muss der Wert nur an einer Stelle geändert werden.",
                            },
                            {
                                value: "Die Direktive wird wie folgt verwendet: `#define <name> <wert>`",
                            },
                            { value: "Beispiel: `#define test 1`" },
                            {
                                value: "Der Wert kann dann wie folgt verwendet werden: `LOAD test`",
                            },
                        ],
                    };
                }

                // Hovering over #include shows description of #include
                if (word.word === "include") {
                    return {
                        range: new monaco.Range(
                            position.lineNumber,
                            word.startColumn,
                            position.lineNumber,
                            word.endColumn
                        ),
                        contents: [
                            { value: "**Compiler-Direktive include**" },
                            {
                                value: "Mit der Compiler-Direktive `#include` können andere Dateien in die aktuelle Datei eingebunden werden. Dies ist besonders nützlich, wenn Code mehrfach verwendet wird. Die Datei wird im selben Ordner wie die Quelldatei gesucht.",
                            },
                            {
                                value: "Die Direktive wird wie folgt verwendet: `#include <dateiname>`",
                            },
                            { value: "Beispiel: `#include test.rm`" },
                        ],
                    };
                }

                return null;
            },
        });

        disposes.push(disposeTknProvider);
        disposes.push(disposeComplItemProvider);
        disposes.push(disposeHvrProvider);

        // Define a new theme that contains only rules that match this language
        monaco.editor.defineTheme("myCoolTheme", {
            base: "vs-dark",
            inherit: true,
            rules: [
                { token: "label", foreground: "FFA500" },
                { token: "label.sub", foreground: "c17d00" },
                { token: "number.ptr", foreground: "8bc964" },
                { token: "number.lit", foreground: "c9c164" },
                { token: "number", foreground: "64c99d" },
                { token: "string.define.identifier", foreground: "64abc9" },
            ],
            colors: {
                "editor.background": "#1d232a",
            },
        });

        editor = monaco.editor.create(editorElement, {
            automaticLayout: true,
            theme: "myCoolTheme",
            language: "mySpecialLanguage",
        });

        loadCode(text, "mySpecialLanguage");
        editor.focus();
        editor.onDidChangeModelContent(() => {
            text = editor.getValue();
            lastTextLocalSet = Date.now();
        });
    });

    onDestroy(() => {
        monaco?.editor.getModels().forEach((model) => model.dispose());
        disposes.forEach((dispose) => dispose());
        editor?.dispose();
    });

    $: {
        // Only set if the text was not overwritten by the local editor
        if (editor && text && lastTextLocalSet < lastTextForeignSet) {
            editor.setValue(text);
            lastTextForeignSet = Date.now();
        }
    }

    $: {
        if (editor && wrapperElement) {
            editor.layout({ width: w, height: h });
        }
    }
</script>

<div class="flex flex-col flex-grow w-full h-full" bind:this={wrapperElement}>
    <div class="flex-grow" bind:this={editorElement} />
</div>
