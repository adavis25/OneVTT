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