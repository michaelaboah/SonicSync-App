<script lang="ts">
	import { RadioItem, RadioGroup, setModeCurrent, modeOsPrefers, modeCurrent } from "@skeletonlabs/skeleton";
  import { tauri_store } from "$lib/stores/user"
  import SunIcon from "~icons/ri/sun-line"
  import MoonIcon from "~icons/ri/moon-clear-fill"
  import FolderIcon from "~icons/ri/folder-5-line"
	import { invoke } from "@tauri-apps/api/tauri";
  import { appLocalDataDir  } from "@tauri-apps/api/path"
  import { confirm, message } from "@tauri-apps/api/dialog"
  let value = $modeCurrent ? 0 : 1 

  $: if (value === 0) {
    setModeCurrent(true)
  } else if (value === 1) {
    setModeCurrent(false)
  } //else if (value === 2) {
  //   setModeCurrent($modeOsPrefers)
  // }
 

  async function clearDatabase() {

    confirm("This will delete all contents and items in the database.", "Are You Sure?").then((answer) => {
      if (answer) {
        invoke("delete_all")
        message("All items have been deleted", "Status") 
      }    
    })
    
  }
  
  async function resetPrefs() {

    confirm("This will reset all preferences to its factory settings.", "Are You Sure?").then((answer) => {
      if (answer) {
        tauri_store.reset()
        message("Preferences have been reset", "Status") 
      }    
    })
    
  }
</script>

<div class="p-4 space-y-8">

  <section>
    <h2 class="h3 font-bold">User</h2>
    <hr class="w-5/6 mt-4 mb-2"/>

    <!-- Implement "Are You Sure?" -->
     
    <div class="flex flex-row ml-4">

      <button class="btn btn-md py-1.5 px-2 variant-filled-error" on:click={resetPrefs}>Reset Preferences</button>

    </div>
  </section>

  <section>

    <h2 class="h3 font-bold">Appearance</h2>
    <hr class="w-5/6 mt-4 mb-2"/>

    <div class="flex flex-row ml-4 items-center">

      <p class="h5 font-bold mr-2">Theme: </p>
      <RadioGroup rounded="rounded-token" active="variant-filled-primary" class="scale-90" >
        <RadioItem bind:group={value} name="justify" value={0}>Light</RadioItem>
        <RadioItem bind:group={value} name="justify" value={1}>Dark</RadioItem>
        <!-- <RadioItem bind:group={value} name="justify" title="Sync With System" value={2}>System</RadioItem> -->
      </RadioGroup>

    </div>

  </section>

  <section>
    <h2 class="h3 font-bold">Database</h2>
    <hr class="w-5/6 mt-4 mb-2"/>
  
    <div class="flex flex-row ml-4 space-x-4 items-center">
      <p class="h5 font-bold">Location:</p>
      {#await appLocalDataDir() then path }
        {#key $modeCurrent}
          <input type="text" class="input w-1/3 italic" disabled value={path + "database"}>
        {/key}
      {/await}
      <button class="btn btn-md py-1.5 px-2 variant-filled-tertiary" on:click={() => invoke("open_database_folder")}><span class="mr-2"><FolderIcon/></span>Go to Database</button>
      <button class="btn btn-md py-1.5 px-2 variant-filled-error" on:click={clearDatabase}>Clear Database</button> 
    </div>

  </section>

</div>
