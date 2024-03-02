use crate::kube::Kube;use async_trait::async_trait;

#[async_trait]
pub trait LLM {
    async fn query(input: &str) -> String;
    async fn combine(&self, kubes: &[Kube]) -> Kube;
}

pub struct FakeLLM {
}
impl FakeLLM {
    fn new() -> FakeLLM {
        FakeLLM {  }
    }
}

#[async_trait]
impl LLM for FakeLLM {
    async fn query(input: &str) -> String {
        format!("This is a response to: {input}")
    }
    async fn combine(&self, kubes: &[Kube]) -> Kube {
        let mut new_string = String::new();
        for kube in kubes {
            new_string.push_str(kube.name.as_str());
        }
        let uuid = uuid::Uuid::new_v4();
        Kube { name: new_string, uuid }		
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn fake_combine_test() {
        let kubes = vec![
            Kube { name: String::from("water"), uuid: uuid::Uuid::new_v4() },
            Kube { name: String::from("glass"), uuid: uuid::Uuid::new_v4() }
        ];
        let fake_llm = FakeLLM::new();
        let response_kube = fake_llm.combine(&kubes).await;
        assert_eq!(
            String::from("waterglass"),
            response_kube.name,
        );
    }
}
