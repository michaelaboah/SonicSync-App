<script lang="ts">
  import printJS from "print-js"
  import { cableList } from "$lib/stores/equipment";
  let labels = Array(80).fill({
    name: "Name",
    address: "Address",
    city: "City",
  });


  function print() {
    printJS({
      printable: "home",
      type: 'html',
      showModal: true,
      css: "4x20-AveryLabels.css", 
      font_size: "",
    })
  }

  let outer: HTMLElement
  $: console.log(outer)
</script>

  <button class="btn variant-ghost" on:click={print}>Print</button>


<div class="w-fit mx-auto bg-gray-300 grid-style" id="home" bind:this={outer}>
  {#each labels as label, i (i)}
    <div class="label-style rounded-md bg-white" >
      <span class="cable-name"><p class="text-black">{label.name}</p></span>{i+1}
      <!-- {label.address} -->
      <!-- {label.city} -->
    </div>
    {#if (i + 1) % 80 === 0}
      <div class="clear-left block"></div>
    {/if}
  {/each}
</div>


<style>

.cable-name {
  font-size: 0.5rem;
  line-height: 0.5rem; 
  font-style: italic;
}


.label-style {
  margin-right: 0.50in;
  width: 1.75in;
  height: 0.50in;
  text-align: center;
}
.grid-style {
  display: grid;
  grid-template-columns: repeat(4, minmax(0, 1fr));
}

</style>
