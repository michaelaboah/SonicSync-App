
<script lang="ts">
  import PrintPdf, { Page } from "svelte-printpdf";
  
  let labels = Array(80).fill({
    name: "Name",
    address: "Address",
    city: "City",
  });

  let print = false;

</script>

<button class="btn variant-ghost" on:click={() => print = true}>Print</button>

<PrintPdf bind:print={print}>
  <div class="w-fit mx-auto bg-gray-300 grid grid-cols-4 gap-4" id="home">
    {#each labels as label, i (i)}
      <div class="mr-12 w-44 h-12 text-center">
        <span class="text-xs leading-3 italic">{label.name}</span>{i+1}
      </div>
      {#if (i + 1) % 80 === 0}
        <div class="clear-left block"></div>
      {/if}
    {/each}
  </div>
</PrintPdf>

