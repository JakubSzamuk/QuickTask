<script lang="ts">
  import { FloppyDisk, Play, Record } from 'phosphor-svelte'
  // import { invoke } from "@tauri-apps/api/tauri"
  // async function greet(){
  //   // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  //   greetMsg = await invoke("greet", { name })
  // }


  import "./app.css";



  let speed_val = 1;
  let repeat_val = 1;
</script>

<main class="flex flex-col items-center gap-2">
  <button class="relative flex justify-center items-center w-20 h-20 bg-primary rounded-md mt-2 active:scale-95 transition-transform">
    <Record size={59} color="#fff" />
    <p class="absolute bottom-1 right-1 text-lg">F12</p>
  </button>
  <button class="relative flex justify-center items-center w-20 h-20 bg-primary rounded-md active:scale-95 transition-transform">
    <Play size={59} color="#fff" />
    <p class="absolute bottom-1 right-1 text-lg">F10</p>
  </button>
  <div class="w-20">
    <div class="flex items-center">
      <p class="text-sm">Speed</p>
      <p class="ml-auto">{speed_val}x</p>
    </div>
    <input min="1" max="20" bind:value={speed_val} type="range" class="w-20" />
  </div>
  <div class="w-20">
    <div class="flex items-center">
      <p class="text-sm">Repeat</p>
      <p class="ml-auto">{repeat_val == 11 ? "∞" : repeat_val}</p>
    </div>
    <input min="1" max="11" bind:value={repeat_val} type="range" class="w-20" />
  </div>

  <button class="relative flex justify-center items-center w-20 h-20 bg-primary rounded-md active:scale-95 transition-transform">
    <FloppyDisk size={59} color="#fff" />
  </button>


</main>

<style>
  
</style>