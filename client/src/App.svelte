<script lang="ts">
  import { onMount } from 'svelte';

  let messages: string[] = [];
  let input = '';
  let ws: WebSocket;

  onMount(() => {
    ws = new WebSocket('ws://localhost:3000/ws');

    ws.onopen = () => {
      console.log('Connected to server');
      messages = [...messages, 'Connected to server'];
    };

    ws.onmessage = (event) => {
      messages = [...messages, event.data];
    };

    ws.onclose = () => {
      messages = [...messages, 'Disconnected'];
    };

    return () => ws.close();
  });

  function send() {
    if (input.trim() && ws.readyState === WebSocket.OPEN) {
      ws.send(input);
      input = '';
    }
  }
</script>

<main>
  <h1>VTT Test</h1>

  <div class="messages">
    {#each messages as msg}
      <p>{msg}</p>
    {/each}
  </div>

  <input bind:value={input} placeholder="Send a message..." />
  <button on:click={send}>Send</button>
</main>