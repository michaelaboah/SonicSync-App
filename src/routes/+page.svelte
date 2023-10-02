<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri'
  import { project } from '$lib/stores/project';
	import { onMount } from 'svelte';
	import printJS from 'print-js';
  // import 
 

  onMount(async () => {

    let labels = ["Guitar", "Oboe", "Violin 1", "Violin 2"]
    const base64 = await invoke<string>("print_4x20_labels", { labels } );
    printJS({printable: base64, type: "pdf", base64: true})
  })


</script>

<div class="justify-center items-center h-full w-full flex">
  <div class="">
  <h1 class="h1">Welcome</h1>  
  <h1 class="h3 font-bold">This page serves no purpose</h1>  
  <span class="h3">Tip:</span><h1 class="h3 italic"> Hit <kbd>Ctrl</kbd> + <kbd>k</kbd> For a quick menu</h1>  
  </div>
</div>
