
pub struct Space {
    coordinate: [u64; 2],
    contains: SpaceKind,
} 

pub enum SpaceKind {
    Kube(Kube),
    Player(Player),
    EmptySpace,
}

