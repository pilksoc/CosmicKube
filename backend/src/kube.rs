#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Kube {
    pub uuid: uuid::Uuid,
    pub name: String,
}
