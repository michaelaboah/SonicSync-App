<script lang="ts">
  import { AppBar, SlideToggle, modeCurrent } from "@skeletonlabs/skeleton";
  import { gearList } from "$lib/stores/equipment";
  import Item from "./Item.svelte";

  $: categoryObject = $gearList.reduce((acc, item, index) => {
    (acc[item.equipment.category] = acc[item.equipment.category] || []).push(index);
    return acc;
  }, {});

  $: categories = Object.entries(categoryObject)

  $: cloudToggleAll = false;


  function deleteItem(itemToDelete: any ) {
    $gearList = $gearList.filter((i) => i !== itemToDelete);
  }

  function addEmptyGear(gearId: number) { 
    let empty = {
      equipment: { id: gearId, category: "", model: "", manufactuer: "", cost: 0, wattage: 0.0, details: {} },  
      items: [{id: 0, description: "", quantity: 1}]
    };

    $gearList = [...$gearList, empty];
  }

  function addCustomGear(gearId: number, category: string) { 
    let empty = {
      equipment: { id: gearId, category, model: "", manufactuer: "", cost: 0, wattage: 0.0, details: {} },  
      items: [{id: 0, description: "", quantity: 1}]
    };

    $gearList = [...$gearList, empty];
  }

  function newCategory() {
    addCustomGear($gearList.length, "MISC")
    categories = [...categories, ["MISC", [$gearList.length]]];
    console.log(categories)
    
  }

</script>



<header class="z-10 sticky top-0 px-1">
  <AppBar class="variant-ringed-surface py-2" slotTrail="w-full" slotLead="space-x-4">
    <svelte:fragment slot="lead">
      <button type="button" class="btn btn-sm variant-filled-primary" on:click={() => addEmptyGear($gearList.length)}>Add Gear</button>
      <button type="button" class="btn btn-sm variant-filled-tertiary" on:click={() => newCategory()}>Add Category</button>
      <!-- Fix theme switch bug -->
      {#key $modeCurrent}
        <SlideToggle name="" bind:checked={cloudToggleAll}>Global Cloud Search</SlideToggle>
      {/key}
    </svelte:fragment>
    <svelte:fragment slot="trail">
      <button type="button" class="btn btn-sm variant-filled-error" on:click={() => $gearList = $gearList.filter(() => false)}>Remove All</button>
    </svelte:fragment>
    <!-- <svelte:fragment slot="headline">(headline)</svelte:fragment> -->
  </AppBar >
</header>



<section class="p-1">
  {#each categories as [category, indicies]}
    <div class="bg-surface-100 variant-ringed px-2 p-1 my-1 rounded">
        <h2 class="font-bold text-primary-400">{category !== "" ? category.toUpperCase() : "EMPTY"}</h2>
      <ul class="list-disc list-inside">
        {#each indicies as i (i)}
          <Item bind:gear={$gearList[i]} cloudSearch={cloudToggleAll}  on:delete={(e) => deleteItem(e.detail)}/>
        {/each}
      </ul>
    </div>
  {:else}
    <div class="card px-2 p-4 my-1 h-full flex justify-center items-center variant-ringed-surface flex-col">
      <h1 class="h3 font-bold">Empty Equipment List</h1> 
      <p class="italic">Use 
        <button type="button" class="btn p-1 btn-sm variant-filled-primary" on:click={() => addEmptyGear($gearList.length)}>Add</button>
         Button to Create an Item</p>
      <p>Or</p>
      <p class="italic">
        Press <kbd class="kbd">Ctl + Shift + N</kbd>
      </p>
    </div>
  {/each}
</section>
