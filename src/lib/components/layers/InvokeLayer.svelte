<script lang="ts">
	import { listen } from "@tauri-apps/api/event";
	import { onMount } from "svelte";
  import { loadProject, meta, project } from "$lib/stores/project"
	import { invoke } from "@tauri-apps/api/tauri";
	import type { Project } from "$lib/@types/project";
	import { preferences } from "$lib/stores/user";
  import { toastStore } from "@skeletonlabs/skeleton" 
	import { goto } from "$app/navigation";
 

  onMount(async () => {

    setInterval(async () => {
      if ($meta === undefined) {
        return 
      }

      invoke("save", { path: $meta.currentFilePath, object: $project })
        toastStore.trigger({ message: "Saved Project [Auto]", classes: "p-2", timeout: 1000 * 10})
    }, $preferences.general.autoSaveTimer * 60 * 1000) 


    listen('load-project', ({ payload: [path, project] }) => {
      meta.set({currentFilePath: path}) 
      loadProject(project as Project)
      toastStore.trigger({ message: `Project: ${path} Loaded`, classes: "p-2", timeout: 1000 * 10})
    }) 

    listen("save-project-as", ({ payload: path }) => {
      meta.set({currentFilePath: path}) 
      invoke("save", { path , object: $project })
    })

    listen("goto-preferences", () => {
      goto("/preferences")
    })

  })
</script>

