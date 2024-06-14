<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";

  let cvs = document.querySelector("canvas");
  let ctx = cvs?.getContext("2d");
  if (ctx) {
    ctx.fillStyle = "blue"
    ctx.fillRect(40, 40, 300, 500);
  }
  
  type cell = {id: number}

  let cellHeight = 0;

  // const tablesize = 256;
  const tablesize = 256;

  const makeCells = (_:any, i:number): cell => {return {id: i}};
  let cells: cell[] = Array.from({length: tablesize}, makeCells);

  const setChild = async (child: HTMLElement, position: string) => {
      child.style.marginTop = position;
  }

  const logPos = async (e: MouseEvent) => {
    if (e.buttons == 1) {
      let el = e.currentTarget as HTMLElement;
      let child = el.children[0] as HTMLElement;
      let rect = el.getBoundingClientRect();
      let y = (e.clientY - rect.y); 
      setChild(child, y + "px");
      let id = el.textContent;
      if (id) {
        let val = 1 - (y / cellHeight);
        // table[parseInt(id)] = y / cellHeight;
        // invoke("sendTable", {size: tablesize});
        invoke("send_table", {value: val, index: parseInt(id)});
      }
    }
  }
</script>

<div id="wavetray">
  <h2>WAVETABLE</h2>
  <div id="table">
  {#each cells as c, i}
    <div 
      class="cell" 
      id={"cell"+c.id} 
      on:mousemove={logPos} 
      bind:clientHeight={cellHeight}> 
      <div class="head">{i}</div>
    </div>
  {/each}
  </div>
</div>
<!-- <canvas height="400" width="600"> -->
<!-- </canvas> -->


<style>
  #wavetray h2 {
    color: white;
  }

#table {
  display: flex;
  flex-direction: row;
  justify-content: space-evenly;
  width: 100%;
  overflow: hidden;
}

#table .cell {
  background-color: lightgray;
  max-width: 3px;
  min-width: 3px;
  height: 590px;
  color: transparent;
}

#table .head {
  background-color: lightgray;
  border-top: solid;
  border-color: black;
  border-width: 2px;
  width: 7px;
  height: 590px;
  margin-top: calc(590px / 2);
}

canvas {
  background-color: white;
}
</style>

