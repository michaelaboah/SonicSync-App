<script lang="ts">
  import type { Cable } from "$lib/@types/equipment";
  export let label: Cable

</script>

<ul class="w-[1.75in] max-w-[1.75in] rounded max-h-[0.5in] h-[0.5in] bg-red-100">
  <span class="text-lg font-bold">{label.name}</span>
</ul>
