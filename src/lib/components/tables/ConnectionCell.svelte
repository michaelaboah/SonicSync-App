<script lang="ts">
  import { Analog, ComputerConnKind, PowerConnector, NetworkCableKind,  Category } from "$lib/@types/graphql";
  import type { Connection } from "$lib/@types/equipment"
	import { gearList } from "$lib/stores/equipment";
	import { type AutocompleteOption, type PopupSettings, Autocomplete, popup, modeCurrent } from "@skeletonlabs/skeleton";
  import CloseIcon from "~icons/ri/close-circle-line"
  import EditIcon from "~icons/ri/edit-2-line"

  type ConnectionKind = "source" | "destination"

  export let cellClass = "!py-0.5 border border-surface-300 dark:border-surface-500"
  export let connection: Connection;
  export let connKind: ConnectionKind;
  export let cableKind: "Power" | "Analog" | "Digital" | "Network" = "Analog";

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
        variants = Object.values(ComputerConnKind).map((x) => x.replaceAll("_", " ").toUpperCase()) 
        break;
      }
      case "Network": {
        variants = Object.values(NetworkCableKind).map((x) => x.replaceAll("_", " ").toUpperCase())
      break;
    }
  }  

  const popupTarget = (Math.random() + 1).toString(36).substring(7);
  const popupSettings: PopupSettings = {
    event: 'focus-click', 
    target: popupTarget,
    placement: 'bottom'
  };


  $: possibleEquips = $gearList.filter((x) => {
      console.log(x.equipment)
    switch (cableKind) {
      case "Analog": {
        return JSON.stringify(x.equipment).includes("analog") || x.equipment.category == Category.Microphones 
      }
      case "Power": {
        return JSON.stringify(x.equipment).includes("power")  
      }
      case "Digital": {
        return JSON.stringify(x.equipment).includes("computer_ports")
      }
      case "Network": {
        return JSON.stringify(x.equipment).includes("network")
      }
    }
  }).flatMap((t) => {
      return t.items.map((item) => {
        let description = item.description.length === 0 ? "" : '~ ' + item.description
        return { label: `${t.equipment.model} ${description}`, value: t.equipment.model } as AutocompleteOption 
      })
  })



  function onConnSelection(e: any): void {
    connection.name = e.detail.label;
  }
</script>
  <td class={cellClass}>

    <div class="flex justify-around h-7 w-full space-x-1">

      <input
        class="input w-full autocomplete text-xs"
        type="search"
        name="autocomplete-search"
        bind:value={connection.name}
        placeholder={connKind === "source" ? "Find Source..." : "Find Dest..."}
        use:popup={popupSettings}
      />
      <div data-popup={popupTarget} class="card min-w-32 max-w-72 max-w-sm max-h-48 text-xs m-0 overflow-y-auto" tabindex="-1">
          <Autocomplete bind:input={connection.name} options={possibleEquips} on:selection={onConnSelection} />
      </div>
     
        <select class="select w-full text-xs p-0" placeholder="Select Termination" name="Choose" bind:value={connection.kind}>
          {#each variants as v }
            <option class="bg-surface-50" value={v}>{v}</option> 
          {/each} 
        </select>
      <div class="flex flex-row">
      </div>

    </div>

  </td>
