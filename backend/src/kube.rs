use std::str::FromStr;

use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Kube {
    pub id: Uuid,
    pub name: String,
}
impl Kube {
    pub fn new(name: String) -> Kube {
        let mut name_uuid = name.clone();
        name_uuid.push_str("kube");
        Kube {
            id: Uuid::new_v4(),
            name,
        }
    }
    pub fn from_name_uuid(name: &str, uuid: &str) -> Result<Kube, <Uuid as FromStr>::Err> {
        Ok(Kube {
            id: Uuid::from_str(uuid)?,
            name: name.to_string()
        })
    }
}

// we should have a placeholder ''loading'' cube we can send over if api is slow

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_kubes() {
        let input_json = String::from(
"[{\"name\":\"hydrogen\",\"id\":\"153227c6-c069-4748-b5aa-aafac8abef00\"},{\"name\":\"oxygen\",\"id\":\"3ffe4c9e-5d35-42c9-a70e-1d80c544bdbb\"},{\"name\":\"nitrogen\",\"id\":\"bb155bf1-7200-45e7-b126-f2882f7aecaa\"}]"
        );
        let mut expected: Vec<Kube> = vec![
            Kube::from_name_uuid("hydrogen", "153227c6-c069-4748-b5aa-aafac8abef00").unwrap(),
            Kube::from_name_uuid("oxygen", "3ffe4c9e-5d35-42c9-a70e-1d80c544bdbb").unwrap(),
            Kube::from_name_uuid("nitrogen", "bb155bf1-7200-45e7-b126-f2882f7aecaa").unwrap(),
        ];
        let mut kubes: Vec<Kube> = serde_json::from_str(&input_json).unwrap();
        expected.sort();
        kubes.sort();
        assert_eq!(expected, kubes);
    }
}
