<script lang="ts">
    import {
        AirplaneInFlight, ArrowFatLineRight,
        ArrowsInSimple, Bug,
        FrameCorners,
        PersonSimpleWalk, Play,
        Stop,
        X
    } from "phosphor-svelte";
    import {appWindow} from '@tauri-apps/api/window'
    import {
        editorApiRef,
        globalLog,
        currentOpenFilePath,
        cancelExecution,
        currentAccumulator,
        currentSystemRegisters
    } from "../../stores";
    import {invoke} from "@tauri-apps/api/tauri";
    import {currentUserRegisters} from "../../stores.js";

    let speed = 30
    let currentlyRunning = false
    let currentlyDebugging = false

    async function uploadRegisters() {
        await invoke("vm_upload", {
            "numbers": $currentUserRegisters.map((v) => v.toString())
        })
    }

    async function compileStep(): Promise<boolean> {
        let filepath = currentOpenFilePath()
        if (!filepath) {
            $globalLog("No file open", "error")
            return false
        }

        $globalLog("Running preflight checks...", "trace")

        // Compile
        let compileResult: string = await invoke("vm_compile", {
            "filepath": filepath
        })

        if (compileResult.includes("@")) {
            let [file, line] = compileResult.split("@")[1].split(":")
            let lineInt = parseInt(line)
            $globalLog("Compilation failed: " + compileResult.split("@")[0], "error")
            $globalLog("Error in " + file + " at line " + (lineInt + 1), "error")
            $editorApiRef.showFile(file, lineInt)
            return false
        }

        await uploadRegisters()

        return true
    }

    async function step(): Promise<{
        "end": boolean,
        "accumulator": number,
        "register": number[],
        "line": {
            "file": string,
            "line": number
        }
    }> {
        let execution: {
            "Executed": {
                "accumulator": number,
                "register": number[],
                "line": {
                    "file": string,
                    "line": number
                }
            }
        } | {
            "End": {
                "accumulator": number,
                "register": number[],
                "line": {
                    "file": string,
                    "line": number
                }
            }
        } = await invoke("vm_step")
        if ("End" in execution) {
            $globalLog("Execution ended", "info")
            return {
                "end": true,
                "accumulator": execution.End.accumulator,
                "register": execution.End.register,
                "line": execution.End.line
            }
        }

        return {
            "end": false,
            "accumulator": execution.Executed.accumulator,
            "register": execution.Executed.register,
            "line": execution.Executed.line
        }
    }

    let stop = false

    async function showDebugInfo(acc: number, registers: number[]) {
        currentAccumulator.set(acc)
        currentSystemRegisters.set(registers)
    }

    async function run() {
        if (!await compileStep()) {
            return
        }

        $globalLog("Done, running...", "info")

        let timePerStep = 1000 / speed
        stop = false
        currentlyRunning = true

        while (!stop) {
            try {
                let execution = await step()

                if (execution.end) {
                    await showDebugInfo(execution.accumulator, execution.register)
                    $editorApiRef.showFile(execution.line.file, execution.line.line + 1)
                    $globalLog("Execution stopped", "info")
                    stopExecution()
                    break
                }

                timePerStep = 1000 / speed

                await showDebugInfo(execution.accumulator, execution.register)
                $editorApiRef.showFile(execution.line.file, execution.line.line + 1)

                await new Promise(resolve => setTimeout(resolve, timePerStep))
            } catch (e) {
                $globalLog("Execution failed: " + e, "error")
                stopExecution()
                break
            }
        }
    }

    async function debugStart() {
        if (!await compileStep()) {
            return
        }

        $globalLog("Done, debugging...", "info")
        stop = false
        currentlyDebugging = true
        await debugStep()
    }
    async function debugStep() {
        if (!currentlyDebugging) {
            return
        }

        try {
            let execution = await step()

            if (execution.end) {
                await showDebugInfo(execution.accumulator, execution.register)
                $editorApiRef.showFile(execution.line.file, execution.line.line + 1)
                $globalLog("Execution stopped", "info")
                stopExecution()
                return
            }

            await showDebugInfo(execution.accumulator, execution.register)
            $editorApiRef.showFile(execution.line.file, execution.line.line + 1)
        } catch (e) {
            $globalLog("Execution failed: " + e, "error")
            stopExecution()
        }
    }

    async function debugToRun() {
        currentlyDebugging = false
        currentlyRunning = true

        let timePerStep = 1000 / speed

        while (!stop) {
            try {
                let execution = await step()

                if (execution.end) {
                    await showDebugInfo(execution.accumulator, execution.register)
                    $editorApiRef.showFile(execution.line.file, execution.line.line + 1)
                    $globalLog("Execution stopped", "info")
                    stopExecution()
                    break
                }

                timePerStep = 1000 / speed

                await showDebugInfo(execution.accumulator, execution.register)
                $editorApiRef.showFile(execution.line.file, execution.line.line + 1)

                await new Promise(resolve => setTimeout(resolve, timePerStep))
            } catch (e) {
                $globalLog("Execution failed: " + e, "error")
                stopExecution()
                break
            }
        }
    }

    function stopExecution() {
        stop = true
        currentlyRunning = false
        currentlyDebugging = false
    }

    $cancelExecution = stopExecution
</script>

<div
        class="w-full h-full flex items-center px-2 justify-between"
        data-tauri-drag-region
>
    <div class="flex items-center gap-2">
        <span class="text-xxl font-bold">RM Editor</span>
        {#if currentlyDebugging}
            <div>
                <button class="px-2 py-1 rounded-md"
                        on:click={stopExecution}>
                    <Stop/>
                </button>
            </div>
            <div>
                <button class="px-2 py-1 rounded-md"
                        on:click={debugStep}>
                    <ArrowFatLineRight/>
                </button>
            </div>
            <div>
                <button class="px-2 py-1 rounded-md"
                        on:click={debugToRun}>
                    <Play/>
                </button>
            </div>
        {:else}
            <div>
                {#if currentlyRunning}
                    <button class="px-2 py-1 rounded-md"
                            on:click={stopExecution}>
                        <Stop/>
                    </button>
                {:else}
                    <button class="px-2 py-1 rounded-md"
                            on:click={run}>
                        <Play/>
                    </button>
                    <button class="px-2 py-1 rounded-md"
                            on:click={debugStart}>
                        <Bug/>
                    </button>
                {/if}
            </div>

            <div class="flex items-center gap-2">
                <PersonSimpleWalk/>
                <input type="range" min="1" max="300" class="h-3 w-32 rounded-md" bind:value={speed}/>
                <AirplaneInFlight/>
            </div>
        {/if}
    </div>
    <div class="flex gap-2">
        <button on:click={appWindow.minimize}>
            <ArrowsInSimple/>
        </button>
        <button on:click={appWindow.maximize}>
            <FrameCorners/>
        </button>
        <button on:click={appWindow.close}>
            <X/>
        </button>
    </div>
</div>
