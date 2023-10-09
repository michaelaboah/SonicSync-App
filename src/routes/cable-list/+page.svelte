<script lang="ts">
	import { Tab, TabGroup } from "@skeletonlabs/skeleton";
	import CableTab from "./CableTab.svelte";
	import BundleTab from "./BundleTab.svelte";


  let tabSet = 0;
				
</script>


<TabGroup>
  <Tab bind:group={tabSet} name="Equipment" title="Equipment" value={0} >
    <!-- <svelte:fragment slot="lead"> -->
      Cables 
    <!-- </svelte:fragment> -->
  </Tab>

  <Tab bind:group={tabSet} name="Cable" title="Cable" value={1} >
    <!-- <svelte:fragment slot="lead"> -->
      Bundles 
    <!-- </svelte:fragment> -->
  </Tab>

  <svelte:fragment slot="panel">
    {#if tabSet === 0 }
      <CableTab/> 
    {:else if tabSet === 1}
      <BundleTab/> 
    {:else if tabSet === 2}
      How did you get here?
    {/if} 
  </svelte:fragment>

</TabGroup>


