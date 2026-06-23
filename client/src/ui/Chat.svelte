<script lang="ts">
  import { getConnection } from '../state/connection.svelte';

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
    {#each conn.messages as msg}
      <p>{msg}</p>
    {/each}
  </div>

  <input bind:value={input} placeholder="Send a message..." />
  <button onclick={handleSend}>Send</button>
</div>