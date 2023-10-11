<script lang="ts">
  import { cableList } from "$lib/stores/equipment";
  import { project } from '$lib/stores/project';
	import { onMount } from 'svelte';
	import { invoke } from "@tauri-apps/api/tauri";

  onMount(async () => {
    invoke("resource")
  })
</script>

<div class="justify-center items-center h-full w-full flex">
  <div class="">
  <h1 class="h1">Welcome</h1>  
  <h1 class="h3 font-bold">This page serves no purpose</h1>  
  <span class="h3">Tip:</span><h1 class="h3 italic"> Hit <kbd>Ctrl</kbd> + <kbd>k</kbd> For a quick menu</h1>  
  </div>

</div>
