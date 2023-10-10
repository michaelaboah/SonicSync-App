
<script lang="ts">
  import { Tab, TabGroup } from "@skeletonlabs/skeleton";
  import CableLabelsTab from "./CableLabelsTab.svelte"; 
  let tabSet = 0


</script>

<div class="m-2">

  <TabGroup>
    <Tab bind:group={tabSet} name="labels" value={0}>
      <span>Labels</span>
    </Tab>
    <Tab bind:group={tabSet} name="equipment" value={1}>
      <span>Equipment</span>
    </Tab>
    <svelte:fragment slot="panel">

      {#if tabSet === 0}
        <CableLabelsTab/> 
      {:else if tabSet === 1}
        EquipmentTab
      {/if}

    </svelte:fragment>
  </TabGroup>

</div>
