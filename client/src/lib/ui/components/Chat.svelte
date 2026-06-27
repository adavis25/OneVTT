<script lang="ts">
  import { getConnection } from '$lib/state/connection.svelte';

  const conn = getConnection();
  let input = $state('');

  function handleSend() {
    conn.send(input);
    input = '';
  }
</script>

<div class="chat">
  <h2>Chat {conn.connected ? '(connected)' : '(disconnected)'}</h2>

  <div class="messages">
  
  </div>

  <input bind:value={input} placeholder="Send a message..." />
  <button onclick={handleSend}>Send</button>
</div>

<style>
  .chat {
    background: rgba(15, 15, 20, 0.85);
    backdrop-filter: blur(4px);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 6px;
    padding: 0.75rem;
    color: #e0e0e0;
  }
</style>