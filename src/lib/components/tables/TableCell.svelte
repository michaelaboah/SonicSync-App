<script lang="ts">
  export let cell;
  export let rowIndex;
  export let cellIndex;
  export let tableData;
  export let selected; 
  let isEditable = false;

  function toggleEditable() {
    isEditable = !isEditable;
  }

  // Custom binding for contenteditable
  function editable(node, cell) {
    node.textContent = cell;

    function update() {
      $tableData[rowIndex][cellIndex] = node.textContent;
    }

    node.addEventListener('input', update);

    return {
      destroy() {
        node.removeEventListener('input', update);
      }
    };
  }
</script>

<style>
  .selected {
    border: 2px solid blue;
  }
</style>

<td
  draggable="true"
  on:dblclick="{toggleEditable}"
  contenteditable="{isEditable}"
  use:editable="{cell}"
  class:selected={selected}
></td>
