
pub struct Space {
    coordinates: [u64; 2],
    contains: SpaceKind,
} 

pub enum SpaceKind {
    Kube(Kube),
    Player(Player),
    EmptySpace,
}

