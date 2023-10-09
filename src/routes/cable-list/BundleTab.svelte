<script lang="ts">
  import { AppBar, type PopupSettings, popup } from "@skeletonlabs/skeleton";
  import { bundleList } from "$lib/stores/flow";
  import PrintIcon from "~icons/material-symbols/print-outline"
  import CableIcon from '~icons/mdi/cable-data'
  import PlusIcon from "~icons/ri/add-fill"
  import NextIcon from "~icons/fluent/next-16-regular"
  import PrevIcon from "~icons/fluent/previous-16-regular"
  import { default as BundleComponent} from "$lib/components/Bundle.svelte"
	import type { Bundle } from "$lib/@types/flow";
  
  let numberOfBundlesToAdd = 0
  let selectAll = false;

  const size = 25;
  let page = 0;
  
  $: pages = Math.ceil($bundleList.length / size)
  $: top = $bundleList.length <= size ? $bundleList.length  : size * (page + 1) - 1
  $: activeBundles = $bundleList.slice(page * size, top)
  $: activePage = (i: number) => (page == i - 1 ? 'text-lg mx-2 font-bold text-primary-500' : 'text-lg mx-2 font-bold')


  function addEmptyBundles() {


  }

  function deleteBundle(bundle: Bundle) {

  }
</script>

<section class="px-2">
  <div class="w-full">
    <AppBar class="variant-ringed-surface rounded py-1 my-1" slotTrail="w-full">
      <svelte:fragment slot="lead">
        <div class="space-x-2 flex flex-row">

          <input class="input w-24 h-7" min={1} max={100} type="number" bind:value={numberOfBundlesToAdd}/>


          <button 
            type="button" 
            class="btn btn-sm variant-filled-primary space-x-0 px-1"
            on:click={() => addEmptyBundles()}
          >
            <span class="scale-75"><PlusIcon/></span>
            Add Bundle
            <!-- <span><CableIcon/></span> -->
          </button>


          <!-- <button type="button" class="btn btn-sm variant-filled-secondary" on:click={macosPrint}> -->
          <!--   <span><PrintIcon/></span> -->
          <!-- </button> -->

        </div>
      </svelte:fragment>
      <svelte:fragment slot="trail">

        <button type="button" class="btn btn-sm variant-filled-error" on:click={() => $bundleList = $bundleList.filter(() => false)}>Remove All</button>
      </svelte:fragment>
    </AppBar >
  </div>

</section>

{#if $bundleList.length !== 0 }

  <section class="flex flex-col ">
    <div class="flex-grow">
      <table class="table w-full">
          <thead class=""> 
              <th class="!p-1 text-center w-1">
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
          </thead> 

          <tbody class="w-full">
            {#each activeBundles as bundle, index (index) } 
              <!-- <BundleComponent selected={selectAll} bind:bundle on:delete={(e) => deleteBundle(e.detail)}/>        -->
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
    <h1 class="h3 font-bold">Empty Bundle List</h1> 
    <p class="italic">Use 
      <button type="button" class="btn p-1 btn-sm variant-filled-primary" on:click={() => {}}>Add</button>
       Button to Create a Cable</p>
    <p>Or</p>
    <p class="italic">
      Press <kbd class="kbd">Ctl + Shift + N</kbd>
    </p>
  </div>
{/if}
