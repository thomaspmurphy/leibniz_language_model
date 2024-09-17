<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { fade, fly } from 'svelte/transition';
  import { onMount } from 'svelte';

  let quote = "";
  let visible = false;
  let rotating = false;
  let rotationDegree = 0;

    async function generateQuote() {
    visible = false;
    rotationDegree += 360;
    await new Promise(resolve => setTimeout(resolve, 500)); // Wait for fade out
    quote = await invoke("get_leibniz_quote");
    visible = true;
    setTimeout(() => {
        rotating = false;
    }, 500); // Stop rotation after 1 second
    }

  onMount(() => {
    generateQuote();
  });
</script>

<div class="quote-container">
  <img src="/leibniz.jpg" alt="Gottfried Wilhelm Leibniz" class="portrait" style="transform: rotate({rotationDegree}deg);" />
  <div class="quote-wrapper">
    {#if visible}
      <p in:fly="{{ y: 20, duration: 500 }}" out:fade class="quote">"{quote}"</p>
    {/if}
  </div>
  <button on:click={generateQuote}>Generate New Quote</button>
</div>

<style>
  :global(body) {
    background-color: black;
    color: #fee8af;
  }

  .quote-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 2rem;
    min-height: 100vh;
  }

  .portrait {
    width: 200px;
    height: 200px;
    border-radius: 50%;
    margin-bottom: 1rem;
    transition: transform 1s ease-in-out;
  }

  .quote-wrapper {
    height: 150px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .quote {
    font-size: 1.5rem;
    font-style: italic;
    text-align: center;
    margin-bottom: 1rem;
    color: #fcdf96;
  }

  button {
    padding: 0.5rem 1rem;
    font-size: 1rem;
    cursor: pointer;
    background-color: #fee8af;
    color: black;
    border: none;
    border-radius: 4px;
  }

  button:hover {
    background-color: #ffec8b;
  }
</style>