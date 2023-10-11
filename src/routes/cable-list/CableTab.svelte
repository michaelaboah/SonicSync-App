<script lang="ts">
  import { AppBar, type PopupSettings, popup } from "@skeletonlabs/skeleton";
  import { cableList } from "$lib/stores/equipment"
	import { Analog } from "$lib/@types/graphql";
	import type { Cable } from "$lib/@types/equipment";
  import { default as CableComponent }  from "$lib/components/Cable.svelte"
	import { invoke } from "@tauri-apps/api/tauri";
  import PrintIcon from "~icons/material-symbols/print-outline"
  import CableIcon from '~icons/mdi/cable-data'
  import PlusIcon from "~icons/ri/add-fill"
  import NextIcon from "~icons/fluent/next-16-regular"
  import PrevIcon from "~icons/fluent/previous-16-regular"
	import { printDialog } from "tauri-plugin-printing-ext-api";
  


  let selectAll = false;
  let numberOfCablesToAdd = 1;
  let cableNumberPopup = false


  const cableNumberInputPopup: PopupSettings = {
    event: 'focus-click',
    target: "cableNumber",
    placement: 'bottom'
  };



  $: if (numberOfCablesToAdd > 100) {
    cableNumberPopup = true;

    setTimeout(() => {
      cableNumberPopup = false
      numberOfCablesToAdd = 100
    }, 5 * 1000)
  }

  const size = 25;
  let page = 0;
  
  $: pages = Math.ceil($cableList.length / size)
  $: top = $cableList.length <= size ? $cableList.length  : size * (page + 1) - 1
  $: activeCables = $cableList.slice(page * size, top)
  $: activePage = (i: number) => (page == i - 1 ? 'text-lg mx-2 font-bold text-primary-500' : 'text-lg mx-2 font-bold')

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
        textColor: 'black',
        color: { r: 0, b: 0, g: 0 },
        alignment: "Left"
      },
      bundle: { name: "", cableIds: []},
      source: { name: "", kind: Analog.XlrAnalog }, 
      destination: { name: "", kind: Analog.XlrAnalog },
    } as Cable

     return emptyCable
  }

  function addEmptyCables() {
    let newCables = [];
    for (let i = 1; i <= numberOfCablesToAdd; i++) {
      newCables.push(addEmptyCable($cableList.length + i))
    }

    $cableList = [...$cableList, ...newCables]
  }

  async function macosPrint() {
    const base64 = await invoke<string>("print_4x20_labels", { labels: $cableList } );

    if (navigator.userAgent.includes("Mac")) {
      printDialog(base64);
    }
  }
  
  function deleteCable(cableToDelete: any ) {
    $cableList = $cableList.filter((i) => i !== cableToDelete);
  }

</script>


<section class="px-2">
  <div class="w-full">
    <AppBar class="variant-ringed-surface rounded py-1 my-1" slotTrail="w-full">
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
  </div>

</section>

{#if $cableList.length !== 0 }

  <section class="flex flex-col ">
    <div class="flex-grow">
      <table class="table w-full table-fixed">
          <thead class=""> 
              <th class="!p-1 text-center w-8">
                <input class="checkbox" type="checkbox" bind:checked={selectAll}/>
              </th>
              <th class="!p-1 text-center w-2/12">Name</th>
              <th class="!p-1 text-center w-2/12">Description</th>
              <th class="!p-1 text-center w-1/12">Model</th>
              <th class="!p-1 text-center w-1/12">Type</th>
              <th class="!p-1 text-center w-1/12">Length</th>
              <th class="!p-1 text-center w-1/12">Bundle</th>
              <th class="!p-1 text-center w-2/12">Source</th>
              <th class="!p-1 text-center w-2/12">Destination</th>
              <th class="!p-1 text-center w-1/12">Actions</th>
          </thead> 

          <tbody class="w-full" >
            {#each activeCables as cable (cable.id) } 
              <CableComponent selected={selectAll} bind:cable on:delete={(e) => deleteCable(e.detail)}/>       
            {/each}
          </tbody>
      </table>
    </div>

  <!-- <section class="relative"> -->
    <div class="absolute bottom-0 w-full h-10 flex justify-between bg-surface-100 dark:bg-surface-600">
      <div></div>
      <div class="flex flex-row">
        <button class="btn btn-icon" on:click={() => page--}><span class="variant-filled-secondary rounded p-0.5"><PrevIcon/></span></button>
      </div>
      <div class="flex flex-row">

        {#each Array(pages) as _, index (index) }
          {#if index == 0}
            <button class={activePage(index)}  on:click={() => page = index - 1}>First </button> 
          {:else if Array(pages).length == index + 1 }
            <button class={activePage(index)} on:click={() => page = index - 1 }>Last </button> 
          {:else}
            <button class={activePage(index)} on:click={() => page = index - 1 }>{index} </button> 
          {/if}
        {/each}

      </div>
      <div class="flex flex-row">
        <button class="btn btn-icon" on:click={() => page++}><span class="variant-filled-secondary rounded p-0.5"><NextIcon/></span></button>
      </div>
      <div></div>
    </div>

  <!-- </section> -->
</section>
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
