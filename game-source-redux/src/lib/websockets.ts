import { v4 } from "uuid";

const baseUrl = "https://hack.djpiper28.co.uk/ws/";

export const ERR_VAL = -9999999;

/**
 * This contains the change of position of the player
 */
export type PlayerCoordinateDelta = {
  oldCoord: Coordinate;
  newCoord: Coordinate;
};

class KubeWebSocket {
  // Socket Shite
  public onError: (ev: Event) => void = () => { };
  public onClose: () => void = () => { };

  // Game actions
  public onPlayerAction?: (
    coords: PlayerCoordinateDelta,
    player: Player,
  ) => void;
  public onKubeAction?: (coords: PlayerCoordinateDelta, kube: Kube) => void;
  public onEmptyAction?: (coords: PlayerCoordinateDelta) => void;

  private ws: WebSocket;

  constructor() {
    this.ws = new WebSocket(baseUrl);
    this.ws.onmessage = this.onMessage.bind(this);
    this.ws.onclose = this.onClose.bind(this);
    this.ws.onerror = this.onError.bind(this);
  }

  private onMessage(event: MessageEvent) {
    console.log(event.data);
    const data: string = event.data;
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

  public connectAs(name: string) {
    const data: PlayerInfo = {
      initialised: false,
      player: {
        uuid: v4(),
        username: name,
      },
      coordinates: [0, 0],
    };

    this.send(JSON.stringify(data));
  }

  private send(data: string) {
    console.log(`Sending: ${data}`);
    this.ws.send(data);
  }
}

export default new KubeWebSocket();
