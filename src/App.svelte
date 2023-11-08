<script lang="ts">
    import Preloader from "./lib/Preloader.svelte";
    // @ts-ignore
    import {HSplitPane, VSplitPane} from 'svelte-split-pane';
    import Navbar from "./lib/Navbar.svelte";
    import Sidebar from "./lib/fileExplorer/Sidebar.svelte";
    import MainContent from "./lib/MainContent.svelte";
    import Terminal from "./lib/Terminal.svelte";
    import Footer from "./lib/Footer.svelte";
    import {workspace} from "./stores";
    import {onDestroy, onMount} from "svelte";

    let loading = false
    let w = 0;
    let h = 0;
    let stop = false;
    let wrapperElement: HTMLDivElement;


    function resizeCheck() {
        if (stop) return;
        if (!wrapperElement) {
            requestAnimationFrame(resizeCheck);
            return;
        }
        const { width, height } = wrapperElement.getBoundingClientRect();
        if (w !== width || h !== height) {
            w = width;
            h = height;
        }
        requestAnimationFrame(resizeCheck);
    }

    onMount(resizeCheck)
    onDestroy(() => stop = true)
</script>

<Preloader bind:loading/>

{#if !$workspace}
    <!-- Message "Loading" -->
    <div class="w-full h-full flex justify-center items-center">
        <div class="text-2xl text-neutral">Loading...</div>
    </div>
{:else}
    <div class="w-full h-full grid" style="grid-template-rows: 2.5rem auto 2.5rem">
        <div class="h-10 bg-neutral">
            <Navbar/>
        </div>
        <div class="flex-grow flex h-full max-w-[100vw]" style="max-height: calc(100vh - 5rem)">
            <VSplitPane topPanelSize="70%" downPanelSize="30%" minTopPaneSize="300px" minDownPaneSize="100px">
                <div class="h-full" slot="top">
                    <HSplitPane leftPaneSize="15%" rightPaneSize="85%" minLeftPaneSize="300px" minRightPaneSize="300px">
                        <div class="h-full" slot="left">
                            <Sidebar/>
                        </div>
                        <div class="h-full" slot="right" bind:this={wrapperElement}>
                            <MainContent {w} {h}/>
                        </div>
                    </HSplitPane>
                </div>
                <div class="flex-grow flex h-full" slot="down">
                    <div class="flex-grow">
                        <Terminal/>
                    </div>
                </div>
            </VSplitPane>
        </div>
        <div class="h-10 bg-neutral">
            <Footer/>
        </div>
    </div>
{/if}