<script lang="ts">
  import svelteLogo from "./assets/svelte.svg";
  import viteLogo from "/vite.svg";
  import Counter from "./lib/Counter.svelte";
  import KubeWebSocket from "./lib/websockets";
	import { onMount } from 'svelte';

  const width = 16;
  const height = 16;
  const cube_size = 100;

  onMount(() => {
    const ws = new KubeWebSocket("Danny");

    ws.onError = (err) => {
      console.error(err);
    };

    ws.onClose = () => {
      console.error("Connection closed");
    };
  });
</script>

<main>
  {#each { length: height } as _, j}
    <div
      style={`width: ${cube_size}px; height: ${cube_size}px; position: absolute; top: ${cube_size * j}px;`}
    >
      {#each { length: width } as _, i}
        <div
          style={`width: ${cube_size}px; height: ${cube_size}px; position: absolute; left: ${cube_size * i}px;`}
        ></div>
      {/each}
    </div>
  {/each}
</main>

<style>
</style>
