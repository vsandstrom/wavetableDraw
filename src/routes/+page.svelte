<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import WaveTable from "../lib/WaveTable.svelte";
    import Controls from "$lib/Controls.svelte";

  let name = "";
  let greetMsg = "";

  // async function trigger() {
  //   await invoke("trigger", {value: true});
  // }


</script>

<!-- <button on:click={trigger}>TRIGGER ME</button> -->
<Controls />
<div class="container">
<WaveTable />
  
</div>

<style>
  :root {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;

    color: #0f0f0f;
    background-color: #f6f6f6;

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
  }

  .container {
    margin: 0;
    padding-top: 1.8vh;
    display: flex;
    flex-direction: column;
    justify-content: center;
    text-align: center;
  }

  @media (prefers-color-scheme: dark) {
    :root {
      color: #f6f6f6;
      background-color: #2f2f2f;
    }
  }
</style>
