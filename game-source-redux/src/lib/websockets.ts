import { v4 } from "uuid";

const baseUrl = "wss://hack.djpiper28.co.uk/ws/";

export const ERR_VAL = -9999999;

/**
 * This contains the change of position of the player
 */
export type PlayerCoordinateDelta = {
  oldCoord: Coordinate;
  newCoord: Coordinate;
};

export default class KubeWebSocket {
  // Socket Shite
  public onError: (ev: Event) => void = console.error;
  public onClose: () => void = console.error;

  // Game actions
  public onPlayerAction?: (
    coords: PlayerCoordinateDelta,
    player: Player,
  ) => void;
  public onKubeAction?: (coords: PlayerCoordinateDelta, kube: Kube) => void;
  public onEmptyAction?: (coords: PlayerCoordinateDelta) => void;

  private ws: WebSocket;
  private name: string;

  constructor(name: string) {
    this.name = name;

    this.ws = new WebSocket(baseUrl);
    this.ws.onmessage = this.onMessage.bind(this);
    this.ws.onclose = this.onClose.bind(this);
    this.ws.onerror = this.onError.bind(this);
    this.ws.onopen = this.onOpen.bind(this);
  }

  private onMessage(event: MessageEvent) {
    const data: string = event.data;
    console.log(`Received: ${data}`);
    const eventData: PlayerInfo = JSON.parse(data) as PlayerInfo;

    const action = eventData.action;
    const coordDelta: PlayerCoordinateDelta = {
      oldCoord: eventData.old_coordinates ?? [ERR_VAL, ERR_VAL],
      newCoord: eventData.coordinates,
    };
    if (action) {
      switch (action.kind) {
        case SpaceKinds.EMPTY:
          this.onEmptyAction?.(coordDelta);
          break;
        case SpaceKinds.KUBE:
          if (!action.kube) throw new Error("Shit is fucked");
          this.onKubeAction?.(coordDelta, action.kube);
          break;
        case SpaceKinds.PLAYER:
          if (!action.player) throw new Error("Shit is fucked");
          this.onPlayerAction?.(coordDelta, action.player);
          break;
      }
    }
  }

  private send(data: string) {
    console.log(`Sending: ${data}`);
    this.ws.send(data);
  }

  private onOpen() {
    const data: PlayerInfo = {
      initialised: false,
      player: {
        uuid: v4(),
        username: this.name,
      },
      coordinates: [0, 0],
    };

    this.send(JSON.stringify(data));
  }
}
