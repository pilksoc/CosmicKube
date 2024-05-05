/**
 * An array of length 2 that represents [x, y] rectilinear coordinate.
 */
export type Coordinate = number[];

/**
 * Its a fokin kube mate
 */
export type Kube = {
  id: string;
  name: string;
};

const baseUrl = "https://hack.djpiper28.co.uk/cache";

export function getAssetUrl(kube: Kube): string {
  return `${baseUrl}/kubeImageById/${kube.id}`;
}

export type Player = {
  uuid: string;
  username: string;
};

export type SpaceKind = number;

export const SpaceKinds = {
  EMPTY: 0,
  KUBE: 1,
  PLAYER: 2,
} as const;

/**
 * A space on the board, see the SpaceKinds object for the different kinds of spaces.
 */
export type Action = {
  coordinate: Coordinate;
  kind: SpaceKind;
  kube?: Kube;
  player?: Player;
};

export type Space = {
  coordinate: Coordinate;
  contains: SpaceKind;
};

export type LocalSpace = Space[];

export const ActionTypes = {
  PICKUP: 0,
  PLACE: 1,
};

/**
 * See the ActionTypes object for the different kinds of actions.
 */
export type ActionType = number;

export type PlayerInfo = {
  initialised: boolean;
  player: Player;
  coordinates: Coordinate;
  old_coordinates?: Coordinate;
  action?: Action;
};
