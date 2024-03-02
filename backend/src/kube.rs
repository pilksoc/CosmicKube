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

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }
}

#[derive(PartialEq, Debug)]
pub struct Kube {
    pub id: KubeId,
    pub name: String,
}
impl Kube {
    pub fn new(name: String) -> Kube {
        Kube {
            id: KubeId::new(name.as_str()),
            name,
        }
    }
}

// we should have a placeholder ''loading'' cube we can send over if api is slow
