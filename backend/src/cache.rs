use crate::kube::{KubeId, Kube};
use std::vec::Vec;

pub struct Recipe {
    items: Vec<KubeId>,
}

impl Recipe {
    pub fn new(items: Vec<KubeId>) -> Self {
        let mut items = items;
        items.sort();
        Recipe { items }
    }
    pub fn hash(&self) -> u64 {
        let big_key = self.items.iter().fold(0, |acc, x| acc ^ x.as_u128());
        (big_key >> 64 & big_key) as u64
    }
}

pub struct PsqlCache {

}
