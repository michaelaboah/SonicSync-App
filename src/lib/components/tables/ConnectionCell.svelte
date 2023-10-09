<script lang="ts">
  import { Analog, PowerConnector } from "$lib/@types/graphql";
  import type { Connection } from "$lib/@types/equipment"
	import { gearList } from "$lib/stores/equipment";
	import { type AutocompleteOption, type PopupSettings, Autocomplete, popup } from "@skeletonlabs/skeleton";
  import CloseIcon from "~icons/ri/close-circle-line"
  import EditIcon from "~icons/ri/edit-2-line"

  type ConnectionKind = "source" | "destination"

  export let cellClass = "!py-0.5 border-surface-300 dark:border-surface-500"
  export let connection: Connection;
  export let connKind: ConnectionKind;
  export let cableKind: "Power" | "Analog" | "Digital" = "Analog";

  let variants: string[] = []
  $: switch (cableKind) {
      case "Analog": {
        variants = Object.values(Analog).map((x) => x.replaceAll("_", " ").toUpperCase())
        break;
      }
      case "Power": {
        variants = Object.values(PowerConnector).map((x) => x.replaceAll("_", " ").toUpperCase()) 
        break;
      }
      case "Digital": {
        // variants = Object.values() 
        break;
      }
  }  

  const popupTarget = (Math.random() + 1).toString(36).substring(7);
  const popupSettings: PopupSettings = {
    event: 'focus-click',
    target: popupTarget,
    placement: 'bottom'
  };


  $: possibleEquips = $gearList.filter((x) => JSON.stringify(x).includes("analog_connections")).map((t) => {
    console.log(t)
    return { label: t.equipment.model, value: t.equipment.model } as AutocompleteOption 
  })



  function onConnSelection(e: any): void {
    connection.name = e.detail.label;
  }
</script>
  <td class={cellClass + " w-1/5"}>

    <div class="flex flex-row h-7 space-x-1">

      <input
        class="input w-full autocomplete text-xs"
        type="search"
        name="autocomplete-search"
        bind:value={connection.name}
        placeholder={connKind === "source" ? "Find Source..." : "Find Dest..."}
        use:popup={popupSettings}
      />

      <div data-popup={popupTarget} class="card w-40 max-w-sm max-h-48 text-xs m-0 overflow-y-auto" tabindex="-1">
          <Autocomplete bind:input={connection.name} bind:options={possibleEquips} on:selection={onConnSelection} />
      </div>
     
      <!-- {#if connection.kind } -->
      
        <select class="select w-full text-sm pl-1 p-0" placeholder="Select Termination" name="Choose" bind:value={connection.kind}>
          {#each variants as v }
            <option class="bg-surface-50" value={v}>{v}</option> 
          {/each} 
        </select>

      <!-- {:else} -->
      <!---->
      <!--   <select class="select w-full text-sm pl-1 p-0" placeholder="Select Type" name="Choose"> -->
      <!--       <option>Empty</option>  -->
      <!--   </select> -->
      <!---->
      <!-- {/if} -->
      <div class="flex flex-row">

        <!-- <button class="btn btn-icon p-0 m-0 h-7 w-fit" on:click={() => connection = null}><span><CloseIcon/></span></button> -->

      </div>

    </div>

  </td>
