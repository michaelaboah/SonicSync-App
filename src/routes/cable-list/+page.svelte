<script lang="ts">
  import { AppBar, type PopupSettings, popup } from "@skeletonlabs/skeleton";
  import { cableList } from "$lib/stores/equipment"
	import type { Cable } from "$lib/@types/equipment";
  import { default as CableComponent }  from "./Cable.svelte"
	import { printDialog } from "tauri-plugin-printing-ext-api";
	import { invoke } from "@tauri-apps/api/tauri";
  import PrintIcon from "~icons/material-symbols/print-outline"
  import CableIcon from '~icons/mdi/cable-data'
  import PlusIcon from "~icons/ri/add-fill"
	import { writable } from "svelte/store";
	import { Analog } from "$lib/@types/graphql";


  let selectAll = false;
  let numberOfCablesToAdd = 1;
  let cableNumberPopup = false

  const cableNumberInputPopup: PopupSettings = {
    event: 'focus-click',
    target: "cableNumber",
    placement: 'bottom'
  };

  const selectedCables = writable<{ selected: boolean, id: number }[]>([])

  $: if (numberOfCablesToAdd > 100) {
    cableNumberPopup = true;

    setTimeout(() => {
      cableNumberPopup = false
      numberOfCablesToAdd = 100
    }, 5 * 1000)
  }


  function addEmptyCable(id: number): Cable {
    let emptyCable = {
      id: id,
      data: {
        name: "",
        description: "",
        bundleName: "", 
        model: "",
        length: 0,
        destinationName: "",
        connectionName: "",
        cableKind: "Analog"
      },
      metadata: {
        textColor: 'white',
        color: { r: 255, b: 0, g: 0 },
        alignment: "Left"
      },
      bundle: { name: "", cableIds: new Set("")},
      source: { name: "", kind: Analog.XlrAnalog }, 
      destination: { name: "", kind: Analog.XlrAnalog },
    } as Cable

     return emptyCable
  }

  function addEmptyCables() {
    let newCables = [];
    for (let i = 1; i <= numberOfCablesToAdd; i++) {
      newCables.push(addEmptyCable($cableList.length))
    }

    $cableList = [...$cableList, ...newCables]
  }

  async function macosPrint() {
    // let printLabels: string[] = $cableList.map((x) => {
    //   return x.data.name
    // });

    const base64 = await invoke<string>("print_4x20_labels", { labels: $cableList } );
      // printDialog(base64);
  }
  
  function deleteCable(cableToDelete: any ) {
    $cableList = $cableList.filter((i) => i !== cableToDelete);
  }
</script>


<section class="p-2">
  <AppBar class="variant-ringed-surface py-2 rounded mt-1" slotTrail="w-full">
    <svelte:fragment slot="lead">
      <div class="space-x-2 flex flex-row">

        <input class="input w-24 h-7" min={1} max={100} type="number" bind:value={numberOfCablesToAdd} use:popup={cableNumberInputPopup}/>

        {#if cableNumberPopup }
          <div data-popup="cableNumber" class="card w-28 p-2 text-center" tabindex="-1">Max Number is 100</div>
        {/if}

        <button 
          type="button" 
          class="btn btn-sm variant-filled-primary space-x-0 px-1"
          on:click={() => addEmptyCables()}
        >
          <span class="scale-75"><PlusIcon/></span>
          <span><CableIcon/></span>
        </button>


        <button type="button" class="btn btn-sm variant-filled-secondary" on:click={macosPrint}>
          <span><PrintIcon/></span>
        </button>

      </div>
    </svelte:fragment>
    <svelte:fragment slot="trail">

      <button type="button" class="btn btn-sm variant-filled-error" on:click={() => $cableList = $cableList.filter(() => false)}>Remove All</button>
    </svelte:fragment>
  </AppBar >

  {#if $cableList.length !== 0 }
    <div class="my-1">
      <table class="table w-full">
        <thead>
          <tr>
            <th class="!p-1 text-center w-10">
              <input class="checkbox" type="checkbox" bind:checked={selectAll}/>
            </th>
            <th class="!p-1 text-center">Name</th>
            <th class="!p-1 text-center">Description</th>
            <th class="!p-1 text-center">Model</th>
            <th class="!p-1 text-center">Type</th>
            <th class="!p-1 text-center">Length</th>
            <th class="!p-1 text-center">Bundle</th>
            <th class="!p-1 text-center">Source</th>
            <th class="!p-1 text-center">Destination</th>
            <th class="!p-1 text-center">Actions</th>
          </tr>
        </thead>
        <tbody>
          {#each $cableList as cable (cable) }
            <CableComponent selected={selectAll} bind:cable={cable} on:delete={(e) => deleteCable(e.detail)}/>       
          {/each}
        </tbody>
      </table>
    </div>
  {:else}
    <div class="card px-2 p-4 my-1 h-full flex justify-center items-center variant-ringed-surface flex-col">
      <h1 class="h3 font-bold">Empty Cable List</h1> 
      <p class="italic">Use 
        <button type="button" class="btn p-1 btn-sm variant-filled-primary" on:click={() => {}}>Add</button>
         Button to Create a Cable</p>
      <p>Or</p>
      <p class="italic">
        Press <kbd class="kbd">Ctl + Shift + N</kbd>
      </p>
    </div>
  {/if}
</section>
 
