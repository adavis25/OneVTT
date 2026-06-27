let messages = $state<string[]>([]);
let connected = $state(false);
let ws: WebSocket;

function connect() {
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
};

function send(text: string) {
    if (text.trim() && ws.readyState === WebSocket.OPEN) {
      ws.send(text);
    }
}

export function getConnection() {
    return {
        get messages() { return messages; },
        get connected() { return connected; },
        connect,
        send
    };
}