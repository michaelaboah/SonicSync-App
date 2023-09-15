<script lang="ts">
  import printJS from "print-js"
  let labels = Array(80).fill({
    name: "Name",
    address: "Address",
    city: "City",
  });


  function print() {
    console.log()
    printJS({
      printable: "home",
      type: 'html',
      showModal: true,
      // gridStyle: 'border: 2px solid #3971A5;',
      css: "/home/michaelaboah/SoftwareEngineering/SonicSync-App/src/lib/assets/4x20-AveryLabels.css", 
      // gridHeaderStyle: "display: none;",
    })
  }

  let outer: HTMLElement
  $: console.log(outer)
</script>

  <button class="btn variant-ghost" on:click={print}>Print</button>


<div class="w-fit mx-auto bg-gray-300 grid-style" id="home" bind:this={outer}>
  {#each labels as label, i (i)}
    <div class="label-style" >
      {label.name}
      <!-- {label.address} -->
      <!-- {label.city} -->
    </div>
    {#if (i + 1) % 80 === 0}
      <div class="clear-left block"></div>
    {/if}
  {/each}
</div>

<style>
.label-style {
  /* overflow: hidden; */
  /* padding: 1rem; */
  /* margin: 0.0rem; */
  margin-left: 0.25in;
  width: 1.75in;
  height: 0.50in;
  text-align: left;
  background-color: #ffffff;
  border-style: dotted;
  border-width: 2px;
  border-color: #6B7280;
}
.grid-style {
  display: grid;
  grid-template-columns: repeat(4, minmax(0, 1fr));
}

</style>
