<script lang="ts">
  import { FileDropzone } from "@skeletonlabs/skeleton";
  import DownloadIcon from "~icons/ri/file-upload-line"
  import CloseIcon from "~icons/ri/close-circle-line"
  import PhotoIcon from "~icons/ri/image-2-line"
  import StampIcon from "~icons/ph/stamp"
  import TrashIcon from "~icons/bi/trash"
  import { prodInfo, audioTeam } from "$lib/stores/project";

  let acceptedMimes = ['image/webp', 'image/jpeg', 'image/png', 'image/gif', 'image/svg+xml'];

  const reader = new FileReader()

  
  function handleFileChange(event: any, setImage: (value: string) => void) {
    const file = event.target.files[0];
    // const reader = new FileReader();
    reader.onloadend = function() {
        setImage(reader.result as string);
    }
    reader.readAsDataURL(file);
  }




  let newRole = "Assistant Production Sound"

  function addNewRole() {
    $audioTeam = [...$audioTeam, { role: newRole, name: "", email: "", phone: "" }]
  }

  function removeRole(id: number) {
    $audioTeam = $audioTeam.filter((_, i) => i !== id) 
  }

</script>

<div class="flex flex-row w-full h-full p-4">
  <section class="flex-grow w-1/2 ">
    <h2 class="h2">Team</h2> <h3 class="h6 italic mb-4 opacity-70">Add Team Members</h3> <hr class="my-4 mr-4"/>

    <!-- <p class="italic">TBA</p>  -->


    <div class="mr-4">
    <table class="table">
    
      <thead>
        <tr>
          <th class="text-center">Role</th>
          <th class="text-center">Name</th>
          <th class="text-center">Email</th>
          <th class="text-center">Phone</th>
          <th class="text-center">Actions</th>
        </tr>
      </thead>

      <tbody>
        {#each $audioTeam as { role, name, email, phone }, id }
          <tr>
            <td class="text font-semibold whitespace-nowrap p-5 border border-surface-300 dark:border-surface-500 max-w-fit" contenteditable bind:innerText={role}></td>
            <td class="text-sm whitespace-nowrap p-5  border border-surface-300 dark:border-surface-500" contenteditable bind:innerText={name}></td>
            <td class="text-sm whitespace-nowrap p-5  border border-surface-300 dark:border-surface-500" contenteditable bind:innerText={email}></td>
            <td class="text-sm whitespace-nowrap p-5  border border-surface-300 dark:border-surface-500" contenteditable bind:innerText={phone}></td>
            <td class="border border-surface-300 dark:border-surface-500">
              <div class="flex justify-end mx-4">
                <button class="btn btn-sm variant-filled-error" on:click={() => removeRole(id)}><TrashIcon/></button>
              </div>
            </td>
          </tr> 
        {/each}
      </tbody>

      <tfoot class="sticky bottom-0">
        <tr class="">
          <!-- <th colspan="3">Where?</th> -->
          <td colspan="5">
            <div class="flex items-center space-x-4 ">
              <h3 class="h5 whitespace-nowrap">Add New Role</h3>
              <input class="input text-sm" type="text" bind:value={newRole}>
              <button class="btn variant-filled-secondary" on:click={addNewRole}>Add Role</button>
            </div>
          </td>
        </tr>
      </tfoot>


    </table>    
</div>


<!-- Team Details -->
</section>


  <span class="divider-vertical h-full w-0" />
  <section class="flex-grow w-1/2 ml-4 ">
    <h2 class="h2">Details</h2>
    <h4 class="h6 italic mb-4 opacity-70">Additional Information for Paperwork</h4>

    <hr class="my-4"/>
<!-- Production Details -->
    <div class="table-container">
      <table class="table table-hover">
        <tbody>
            <tr>
              <td class="w-1/5"><p class="h6 py-auto font-semibold">Production:</p></td>
              <td><input class="input" bind:value={$prodInfo.productionName}/></td>
            </tr>
            <tr>
              <td class="flex"><p class="h6 py-auto font-semibold">Director:</p></td>
              <td><input class="input" bind:value={$prodInfo.director}/></td>
            </tr>
            <tr>
              <td class="flex"><p class="h6 py-auto font-semibold">Venue:</p></td>
              <td><input class="input" bind:value={$prodInfo.venue}/></td>
            </tr>
            <tr>
              <td class="flex"><p class="h6 py-auto font-semibold">Notes:</p></td>
              <td><textarea class="textarea" rows="3" bind:value={$prodInfo.notes}/></td>
            </tr>
        </tbody>
      </table>
  </div>

    <hr class="my-4"/>
<!-- File Drop Zones -->

    <div class="flex flex-row">
      <FileDropzone on:change={(e) => handleFileChange(e, url => $prodInfo.showImage = url)} class="w-1/2" name="Designer Stamp" slotLead="object-center" accept="image/*">
        <svelte:fragment slot="lead"><span class="text-4xl"><DownloadIcon/></span></svelte:fragment>
        <svelte:fragment slot="message">Place Show Image</svelte:fragment>
        <svelte:fragment slot="meta">Only PNG, JPEG, JPG allowed</svelte:fragment>
      </FileDropzone>
      <div class="card relative mx-4 w-1/2 flex justify-center items-center variant-ringed-surface">
        {#if $prodInfo.showImage}
          <button class="absolute btn btn-icon top-0 right-0" on:click={() => $prodInfo.showImage = "" }><CloseIcon/></button>
          <img class="p-6" src={$prodInfo.showImage} alt="Production Image"/> 
        {:else}
          <div class="flex flex-row opacity-70">
            <span class="text-2xl"><PhotoIcon/></span>
            <strong class="">Production Image</strong>
          </div>
        {/if}
      </div>
    </div>
    <hr class="my-4"/>
    <div class="flex flex-row">
      <FileDropzone on:change={(e) => handleFileChange(e, url => $prodInfo.designerStamp = url)} class="w-1/2" name="Designer Stamp" slotLead="object-center" accept="image/*">
        <svelte:fragment slot="lead"><span class="text-4xl"><DownloadIcon/></span></svelte:fragment>
        <svelte:fragment slot="message">Place Designer Stamp</svelte:fragment>
        <svelte:fragment slot="meta">Only PNG, JPEG, JPG allowed</svelte:fragment>
    </FileDropzone>
      <div class="card relative mx-4 w-1/2 flex justify-center items-center variant-ringed-surface">
        {#if $prodInfo.designerStamp}
            <button class="absolute btn btn-icon top-0 right-0" on:click={() => $prodInfo.designerStamp = "" }><CloseIcon/></button>
            <img class="p-6" src={$prodInfo.designerStamp} alt="Stamp"/> 
        {:else}
            <div class="flex flex-row opacity-70">
              <span class="text-2xl "><StampIcon/></span>
              <strong class="">Designer Stamp</strong>
            </div>
        {/if}
      </div>
    </div>
  </section>
</div>
