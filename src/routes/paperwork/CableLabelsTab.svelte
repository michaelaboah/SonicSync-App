<script lang="ts">
  import { jsPDF } from "jspdf"
  import PrintPdf, { Page } from "svelte-printpdf";
	import { cableList } from "$lib/stores/equipment";
	import CableLable from "$lib/components/print/CableLable.svelte";
  import { printDialog } from "tauri-plugin-printing-ext-api" 
  let labels = $cableList.map((x) => {
    return {name: x.name}
  });

  let print = false;
  

  let target: HTMLElement

  function macosPrint(target: HTMLElement) {
    const doc = new jsPDF({ orientation: "p", unit: "px", format: "letter", hotfixes: ["px_scaling"] });

      doc.html(target, {

        callback: (doc) => {
          let base64 = doc.output("datauristring").split(",")[1]
          printDialog(base64);
        },

      })
  }

  let pages: any[][] = [];
  for (let i = 0; i < labels.length; i += 80) {
    pages.push(labels.slice(i, i + 80));
  }


</script>


{#if navigator.userAgent.includes("Mac")}  
  <button class="btn variant-ringed-primary" on:click={() => macosPrint(target)}>Cus</button>
{:else}
  <button class="btn variant-ringed-primary" on:click={() => print = true}>Cus</button>
{/if}

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

