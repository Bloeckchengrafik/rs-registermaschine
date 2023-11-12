<script lang="ts">
    import {currentAccumulator, currentUserRegisters, currentSystemRegisters} from "../../stores";


    function addField() {
        $currentUserRegisters = [...$currentUserRegisters, 0];
    }

    function syncSizes() {
        let lenUsr = $currentUserRegisters.length;
        let lenSys = $currentSystemRegisters.length;

        if (lenUsr > lenSys) {
            $currentSystemRegisters = [...$currentSystemRegisters, ...Array(lenUsr - lenSys).fill(0)];
        } else if (lenSys > lenUsr) {
            $currentUserRegisters = [...$currentUserRegisters, ...Array(lenSys - lenUsr).fill(0)];
        }
    }

    $: {
        $currentSystemRegisters;
        syncSizes()
    }

    function handleUserChange(i: number) {
        if (i === $currentUserRegisters.length - 1) {
            addField();
            syncSizes();
        }
    }

    function validate(event: any) {
        // Remove all non-numeric characters
        event.target.value = event.target.value.replace(/[^0-9]/g, '');
        console.log(event.target.value)
    }
</script>

<div class="wrapper">
    <table>
        <thead>
        <tr>
            <th class="w-5">ACC</th>
            {#each $currentUserRegisters as _, i (i)}
                <th class="w-5">{i + 1}</th>
            {/each}
        </tr>
        </thead>
        <tbody>
        <tr>
            <th class="w-5"></th>
            {#each $currentUserRegisters as _, i (i)}
                <td class="w-5">
                    <input type="text"
                           bind:value={$currentUserRegisters[i]}
                           on:click={() => handleUserChange(i)}
                           on:focus={() => handleUserChange(i)}
                           pattern="[0-9]*"
                           inputmode="numeric"
                           autocomplete="off"
                           on:input={(event) => validate(event)}
                    />
                </td>
            {/each}
        </tr>
        <tr>
            <th>{$currentAccumulator}</th>
            {#each $currentSystemRegisters as _, i (i)}
                <td>
                    <input type="text" bind:value={$currentSystemRegisters[i]} disabled/>
                </td>
            {/each}
        </tr>
        </tbody>
    </table>
</div>

<style>
    .wrapper {
        display: block;
        overflow-x: auto;
        overflow-y: hidden;
        width: 100vw;
        color-scheme: dark;
        flex-grow: 1;
    }

    /* nice darkmode scrollbar */
    .wrapper::-webkit-scrollbar {
        height:8px;
        width:8px;
        background:transparent
    }
    .wrapper::-webkit-scrollbar-thumb {
        background: #626262;
        -webkit-border-radius:1ex;
        -webkit-box-shadow: 0 1px 2px rgba(0,0,0,.75)
    }
    .wrapper::-webkit-scrollbar-corner {
        background: transparent;
    }


    table {
        border-collapse: collapse;
        width: fit-content;

        table-layout: fixed;
        overflow-x: auto;
        overflow-y: hidden;
        white-space: nowrap;

        margin: 5px;
    }


    table, th, td {
        border: 1px solid rgba(0, 0, 0, 0.13);
    }

    th, td {
        width: 70px;
    }

    tr {
        height: 10px;
    }

    input {
        background: transparent;
        border: none;
        text-align: center;
        width: 100%;
        height: 10px;
    }
</style>