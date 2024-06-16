<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let vol = 0.0;
  let freq = 200.0;

  const volume = async () => {
    invoke("set_volume", {value: vol});
  }

  const frequency = () => {
    invoke("set_frequency", {value: freq});
  }

const interpolation = (e: MouseEvent & { currentTarget: HTMLSelectElement }) => {
  invoke("set_interpolation", {value: parseInt(e.currentTarget.value)})
}

  async function trigger() {
    await invoke("trigger", {value: 1.0});
  }

</script>

<div id="controls">
  <button on:click={trigger}>TRIGGER ME</button>
  <input 
    bind:value={vol}  
    on:input={volume} 
    id="volume"
    type="range"
    min="0.0"
    max="0.5"
    step="any"
  >
  <label for="volume">VOLUME</label>
  <input 
    bind:value={freq} 
    on:change={frequency}
    id="frequency"
    type="number" 
    min="18.0"
    max="400.0"
    step="any"
  >
  <label for="frequency">FREQ</label>
  <select name="interpolation" id="lerp" on:input={interpolation}>
    <option value={0}>None</option>
    <option value={1} selected>Linear</option>
    <option value={2}>Cubic</option>
    <option value={3}>Hermetic</option>
  </select>
</div>

<style>
  button { 
    color: #ffffff;
    background-color: #0f0f0f98;
    height: 30px;
  }
  button:active {
    background-color: #0f0f0f69;
  }
</style>
