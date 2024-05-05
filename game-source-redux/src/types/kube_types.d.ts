/**
 * An array of length 2 that represents [x, y] rectilinear coordinate.
 */
type Coordinate = number[];

/**
 * Its a fokin kube mate
 */
type Kube = {
  id: string;
  name: string;
};

type Player = {
  uuid: string;
  username: string;
};

type SpaceKind = number;

const SpaceKinds = {
  EMPTY: 0,
  KUBE: 1,
  PLAYER: 2,
} as const;

/**
 * A space on the board, see the SpaceKinds object for the different kinds of spaces.
 */
type Action = {
  coordinate: Coordinate;
  kind: SpaceKind;
  kube?: Kube;
  player?: Player;
};

type Space = {
  coordinate: Coordinate;
  contains: SpaceKind;
};

type LocalSpace = Space[];

const ActionTypes = {
  PICKUP: 0,
  PLACE: 1,
};

/**
 * See the ActionTypes object for the different kinds of actions.
 */
type ActionType = number;

type PlayerInfo = {
  initialised: boolean;
  player: Player;
  coordinates: Coordinate;
  old_coordinates?: Coordinate;
  action?: Action;
};
