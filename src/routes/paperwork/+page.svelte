
<script lang="ts">
  import { jsPDF } from "jspdf"
  // import html2canvas from "html2canvas"
  import { Tab, TabGroup } from "@skeletonlabs/skeleton";
	import { invoke } from "@tauri-apps/api/tauri";
  import PrintPdf, { Page } from "svelte-printpdf";
	import { onMount } from "svelte";
  
  let labels = Array(240).fill({
    name: "Name",
    address: "Address",
    city: "City",
  });

  let print = false;
  
  let tabSet = 0

  let target: HTMLElement

  function customPrint() {
    const doc = new jsPDF({orientation: "p", unit: "px", format: "letter", hotfixes: ["px_scaling"] });
    doc.html(target, {
      callback: (doc) => {
        let base64 = doc.output("datauristring").split(",")[1]
        invoke("write_to_pdf", { filename: "example.pdf", base64 })
      },
    })
  }

  let pages: any[][] = [];
  for (let i = 0; i < labels.length; i += 80) {
    pages.push(labels.slice(i, i + 80));
  }
</script>


  <button class="btn variant-ringed-primary" on:click={customPrint}>Cus</button>

<PrintPdf bind:print={print}>
    <div class="mx-auto px-auto w-[8.5in]  bg-white" bind:this={target}>
  {#each pages as pageLabels, pageIndex}
      <!-- Page -->
      <div class="grid grid-cols-4 gap-x-[0.4in] my-[0.5in] mx-[0.3in] h-[10in]">
        {#each pageLabels as label, i (i)}
          <div class="w-[1.75in] max-w-[1.75in] rounded max-h-[0.5in] h-[0.5in] bg-red-100">
            <span class="text-lg font-bold">{label.name}</span>{i+1}
          </div>
        {/each}
      </div>
      {#if pageIndex < pages.length - 1}
        <div style="page-break-after: always;" class="my-[1in]"></div>
      {/if}
  {/each}
    </div>
</PrintPdf>---button
