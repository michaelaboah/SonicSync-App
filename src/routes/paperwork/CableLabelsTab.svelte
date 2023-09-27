<script lang="ts">
  import { jsPDF } from "jspdf"
	import { invoke } from "@tauri-apps/api/tauri";
  import PrintPdf, { Page } from "svelte-printpdf";
	import { cableList } from "$lib/stores/equipment";
	import CableLable from "$lib/components/print/CableLable.svelte";
  import { printDialog } from "tauri-plugin-printing-ext-api" 
  let labels = $cableList.map((x) => {
    return {name: x.name}
  });

  // $: if (cableList)
  let print = false;
  

  let target: HTMLElement


  let pages: any[][] = [];
  for (let i = 0; i < labels.length; i += 80) {
    pages.push(labels.slice(i, i + 80));
  }


</script>


<button class="btn variant-ringed-primary" on:click={() => printDialog(target)}>Cus</button>

<PrintPdf bind:print={print}>
    <div class="mx-auto px-auto w-[8.5in]  bg-white" bind:this={target}>
      {#each pages as pageLabels, pageIndex}
        <!-- Page -->
        <div class="grid grid-cols-4 gap-x-[0.4in] my-[0.5in] mx-[0.3in] h-[10in]">
          {#each pageLabels as label, i (i)}
            <CableLable {label}/>
          {/each}
        </div>
        {#if pageIndex < pages.length - 1}
          <div style="page-break-after: always;" class="my-[1in]"></div>
        {/if}
      {/each}
    </div>
</PrintPdf>

