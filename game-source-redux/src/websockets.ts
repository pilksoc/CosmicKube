const baseUrl = "https://hack.djpiper28.co.uk/ws";

class KubeWebSocket {
  private onError: (err: string) => void;
  private onClose: () => void;
  private ws: WebSocket;

  constructor() {
    this.ws = new WebSocket(baseUrl);
    this.ws.onmessage = this.onMessage.bind(this);
    this.ws.onclose = this.onClose.bind(this);
    this.ws.onerror = this.onError.bind(this);
  }

  private onMessage(event: MessageEvent) {
    console.log(event.data);
  }
}

export default new KubeWebSocket();
