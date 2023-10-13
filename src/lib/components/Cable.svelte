<script lang="ts">
  import { createEventDispatcher, type ComponentType } from "svelte";
	import type { Cable } from "$lib/@types/equipment";
  import TrashIcon from "~icons/bi/trash"
  import CloseIcon from "~icons/ri/close-circle-line"
  import CopyIcon from "~icons/ph/copy-duotone"
  import ColorPaletteIcon from "~icons/ic/outline-color-lens"
	import { type AutocompleteOption, type PopupSettings, Autocomplete, popup, modeCurrent } from "@skeletonlabs/skeleton";
	import { bundleList } from "$lib/stores/flow";
	import type { Bundle } from "$lib/@types/flow";
	import ConnectionCell from "$lib/components/tables/ConnectionCell.svelte";
  import ColorPicker from "svelte-awesome-color-picker"


  export let cable: Cable
  export let selected = false;

  let autocompleteComponent: ComponentType;

  let bundleOptions: AutocompleteOption[] = $bundleList.map((x) => {
    return { label: x.name, value: x.name } as AutocompleteOption
  })

  let nameHex: string = "#e2e1e5"

  const cellClass = "!p-0.5  border border-surface-300 dark:border-surface-500"
  const dispatch = createEventDispatcher();
  const popupTarget = (Math.random() + 1).toString(36).substring(7);
  const popupSettings: PopupSettings = {
    event: 'click',
    target: popupTarget,
    placement: 'bottom'
  };

  const popupColorPicker = (Math.random() + 1).toString(36).substring(7);
  const popupSettingsColorPicker: PopupSettings = {
    event: 'focus-click',
    target: popupColorPicker,
    placement: 'bottom'
  };


  $: bundleNames = bundleOptions.filter((x) => cable.bundle ? x.label.toLowerCase().includes(cable.bundle.name.toLowerCase()) : null) 

  $: brightness = Math.round(((cable.metadata.color.r * 299) + (cable.metadata.color.g * 587) + (cable.metadata.color.b * 114)) / 1000);

  // Determine text color based on brightness
  $: textColor = brightness > 155 ? 'black' : 'white'

  $: setTextColor(), textColor;

  $: hex = nameHex === "#000000" ? "#e2e1e5" : nameHex

  function addBundle(cable: Cable): Cable {
    cable.bundle.name = "";
    return cable
  }

  async function loadAutocomplete() {
    const { Autocomplete } = await import("@skeletonlabs/skeleton")
    autocompleteComponent = Autocomplete
  }

  function handleDelete() {
    dispatch("delete", cable)
  }


  function handleCopy() {
    dispatch("copy", cable)
  }

  function onBundleSelection(event: any): void {
    cable.bundle = event.detail.label;
    let foundBundle: Bundle | undefined = $bundleList.find((b) => b.name == cable.bundle.name)
    if (!foundBundle) {
      return
    } 
    foundBundle.cableIds.add(cable.data.name)
  } 
  
  function setTextColor() {
    cable.metadata.textColor = "black" //textColor
  }
  

</script>

<tr class="">

  <td class="!py-0.5 !px-1 border-surface-300 dark:border-surface-500">
    <!-- Fix theme switch bug -->
      {#key $modeCurrent}
        <input class="checkbox" type="checkbox" bind:checked={selected}/>
      {/key}
  </td>

  <td class={cellClass} >

    <p style="background-color: {hex}; color: {textColor}"
      class="px-4 py-1 w-full" 
      contenteditable="true"
      bind:textContent={cable.data.name}>
    </p>

  </td>

  <td class={cellClass} >
    <p class="rounded py-0.5" contenteditable="true" bind:textContent={cable.data.description}></p>
  </td>
  
  <td class={cellClass} >
    <p class="rounded py-0.5" contenteditable="true" bind:textContent={cable.data.model}></p>
  </td>

  <td class={cellClass}>

    <!-- Fix theme switch bug -->
    {#key $modeCurrent}
      <select class="select w-full h-7 text-sm pl-1 p-0" placeholder="Select Type" bind:value={cable.data.cableKind}>
        <option value="Power">Power</option>
        <option value="Analog">Analog</option>
        <option value="Digital">Digital</option>
        <option value="Network">Network</option>
      </select>
    {/key}
  </td>

  <td class={cellClass}>
      <!-- Fix theme switch bug -->
      {#key $modeCurrent}
        <input class="input h-7 py-0 w-full m-0" type="number" bind:value={cable.data.length}/>
      {/key}
  </td>

  <td class={cellClass}> 

    <div class="flex flex-row h-7">
 <!-- Fix theme switch bug -->
      {#key $modeCurrent}
     
      <input
        class="input w-full autocomplete text-xs"
        type="search"
        name="autocomplete-search"
        bind:value={cable.bundle.name}
        placeholder="Find Bundle..."
        use:popup={popupSettings}
        on:focus={loadAutocomplete}
      />
      {/key}
      <div data-popup={popupTarget}  class="card w-40 max-w-sm m-h-48 text-xs p-1 m-0 overflow-y-auto" tabindex="-1">
          {#if autocompleteComponent}

            <svelte:component this={autocompleteComponent} input={cable.bundle} bind:options={bundleOptions} on:selection={onBundleSelection} />
             
            {#if bundleNames.length === 0 && cable.bundle.name !== ""}
              <div class="flex justify-center mt-1">
                <button class="btn btn-xs px-1 py-0.5 variant-filled-secondary text-xs">Create</button>
              </div>
            {/if}

          {/if}
      </div>

      
    </div>

  </td>


   <ConnectionCell bind:connection={cable.source} connKind="source" cableKind={cable.data.cableKind}/>
   <ConnectionCell bind:connection={cable.destination} connKind="destination" cableKind={cable.data.cableKind}/> 

  <td class="!py-0.5 w-20">

    <div class="mx-4 flex flex-row gap-2 py-0.5">

      <button class="btn btn-sm p-0.5 bg-gradient-to-l variant-gradient-primary-secondary" use:popup={popupSettingsColorPicker} >
        <span><ColorPaletteIcon/></span>
      </button>

      <div class="card w-fit h-fit pt-2" data-popup={popupColorPicker}>
        <!-- {#if colorPickerComponent} -->
        <!--   {#await colorPickerComponent then { default: ColorPicker} } -->
              <ColorPicker bind:rgb={cable.metadata.color} bind:hex={nameHex} isAlpha={false}  isInput={false}/> 
        <!--   {/await}  -->
        <!-- {/if} -->
      </div>

      <button class="btn btn-sm variant-filled-tertiary p-0.5" on:click={handleCopy}>
        <span class=""><CopyIcon/></span>
      </button>
      <button class="btn btn-sm variant-filled-error p-0.5" on:click={handleDelete}>
        <span><TrashIcon/></span>
      </button>

    </div>

  </td>

</tr>

