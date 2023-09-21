
<script lang="ts">
  import { jsPDF } from "jspdf"
  // import html2canvas from "html2canvas"
  import { Tab, TabGroup } from "@skeletonlabs/skeleton";
	import { invoke } from "@tauri-apps/api/tauri";
  import PrintPdf, { Page } from "svelte-printpdf";
	import { onMount } from "svelte";
  
  let labels = Array(80).fill({
    name: "Name",
    address: "Address",
    city: "City",
  });

  let print = false;
  
  let tabSet = 0

  let target: HTMLElement

  onMount(() => {
    const doc = new jsPDF({orientation: "p", unit: "px", format: "letter", hotfixes: ["px_scaling"] });
    doc.html(target, {
      callback: (doc) => {
        let base64 = doc.output("datauristring").split(",")[1]
        // console.log(base64)
        invoke("write_to_pdf", { path: "example.pdf", base64 })
      },
    })
    // console.log(target.outerHTML)
    // doc.text("Hello World", 10, 10);
    // doc.save("a4.pdf");

    // console.log(doc.output("datauristring"))
  })
</script>

 

<TabGroup>
  <Tab bind:group={tabSet} value={0} name="Labels">
    <svelte:fragment slot="lead"></svelte:fragment>
    <span>Labels</span>
  </Tab>
</TabGroup>

<button class="btn variant-ghost" on:click={() => print = true}>Print</button>


<PrintPdf bind:print={print}>
  <div class="mx-auto px-auto w-[8.5in] h-[11in] bg-white" id="printTarget" bind:this={target}>
     <!-- Page -->
    <div class="grid grid-cols-4 gap-x-[0.4in] my-[0.5in] mx-[0.3in] h-[10in]">
    {#each labels as label, i (i)}
      <div class="w-[1.75in] max-w-[1.75in] rounded max-h-[0.5in] h-[0.5in] bg-red-100">
        <span class="text-lg italic">{label.name}</span>{i+1}
      </div>
      {#if (i + 1) % 80 === 0}
        <div class="clear-left block"></div>
      {/if}
    {/each}
</div>
  </div>
</PrintPdf>
<!---->
