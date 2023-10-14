<script lang="ts">
  import { onMount } from "svelte";
  import { createEventDispatcher } from "svelte";
  import TableCell from './TableCell.svelte';
	import { writable } from "svelte/store";

  export let tableData = [];
  export let rowHeaders = []
  export let colHeaders = []
  export let rows = 8;
  export let cols = 8;

  let selectedRange = null;
  let state = [];
  let isDrag = false;

  // Create event dispatcher to communicate between components
  const dispatch = createEventDispatcher();

  function createTableData() {
    return Array.from({ length: rows }, () =>
      Array.from({ length: cols }, () => {
        state.push(false);
        return Math.floor(Math.random() * 100);
      })
    );
  }

  $tableData = createTableData();

  const beginDrag = () => {
    isDrag = true;
  };

  const endDrag = () => {
    isDrag = false;
  };

  const toggle = (r, c) => {
    state[r * cols + c] = !state[r * cols + c];
  };

  const mouseHandler = (r, c) => (e) => {
    if (isDrag || e.type === "mousedown") {
      toggle(r, c);
    }
  };
</script>

<style>
  table {
    border-collapse: collapse;
    width: 100%;
    height: 100%;
    background-color: #f8f8f8;
  }

  th, td {
    border: 1px solid #ddd;
    padding: 8px;
    text-align: left;
  }

  th {
    background-color: #4CAF50;
    color: white;
  }

  tr:nth-child(even) {
    background-color: #f2f2f2;
  }

  .selected {
    background-color: blue;
  }
</style>

<svelte:window on:mousedown={beginDrag} on:mouseup={endDrag} />

<table>
  {#if $tableData}
    <tr>
      <th></th>
      {#each $colHeaders as col}
        <th>{col}</th>
      {/each}
    </tr>
    {#each $tableData as row, rowIndex}
      <tr>
        <th>{$rowHeaders[rowIndex]}</th>
        {#each row as cell, cellIndex}
          <TableCell
            {cell}
            rowIndex={rowIndex}
            cellIndex={cellIndex}
            tableData={tableData}
            on:mousedown={mouseHandler(rowIndex, cellIndex)}
            on:mouseenter={mouseHandler(rowIndex, cellIndex)}
            selected="{state[rowIndex * cols + cellIndex]}"
          />
        {/each}
      </tr>
    {/each}
  {/if}
</table>
