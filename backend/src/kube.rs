use uuid::Uuid;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct KubeId {
    uuid: Uuid,
}

impl KubeId {
    pub fn new(name: &str) -> Self {
        let mut name = name.to_string();
        name.push_str("kube");
        KubeId {
            uuid: Uuid::new_v5(&Uuid::NAMESPACE_DNS, name.as_bytes()),
        }
    }

    pub fn as_u128(&self) -> u128 {
        self.uuid.as_u128()
    }
}

#[derive(PartialEq, Debug)]
pub struct Kube {
    id: KubeId,
    name: String,
}
