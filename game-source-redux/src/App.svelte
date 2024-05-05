<script lang="ts">
  import KubeWebSocket from "./lib/websockets";
  import { onMount } from "svelte";
  import { v4 } from "uuid";

  const width = 16;
  const height = 16;
  const cube_size = 100;

  onMount(() => {
    const ws = new KubeWebSocket(v4());

    ws.onError = (err) => {
      console.error(err);
    };

    ws.onClose = () => {
      console.error("Connection closed");
    };
  });
</script>

<main>
  {#each { length: height } as _, y}
    {#each { length: width } as _, x}
      <div
        id={`tile-${y}-${x}`}
        class={`tile ${(x + y) % 2 == 0 ? "tile-even" : "tile-odd"}`}
        style={`width: ${cube_size}px; height: ${cube_size}px; position: absolute; top: ${cube_size * y}px; left: ${cube_size * x}px;`}
      ></div>
    {/each}
  {/each}
</main>

<style>
  .tile {
    border-style: solid;
    border-width: thin;
  }
  .tile-even {
    background-color: magenta;
  }

  .tile-odd {
    background-color: black;
  }
</style>
