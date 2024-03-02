use crate::kube::Kube;use async_trait::async_trait;

/// Trait for interacting with an LLM.
#[async_trait]
pub trait LLM {
    /// Send a query to the LLM and get a [`std::string::String`] response.
    async fn query(input: &str) -> String;
    /// Ask the LLM to combine the given Kubes and return a new Kube.
    async fn combine(&self, kubes: &[Kube]) -> Kube;
}

/// A fake LLM that functions very basically, not processing the input in any meaningful way. This is most useful for testing functionality of other features which use LLMs.
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
        let uuid = uuid::Uuid::new_v5(&uuid::Uuid::NAMESPACE_DNS, b"poo");
        Kube { name: new_string, uuid }		
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn fake_combine_test() {
        let kubes = vec![
            Kube { name: String::from("water"), uuid: uuid::Uuid::new_v5(&uuid::Uuid::NAMESPACE_DNS, b"poo") },
            Kube { name: String::from("glass"), uuid: uuid::Uuid::new_v5(&uuid::Uuid::NAMESPACE_DNS, b"poo") }
        ];
        let fake_llm = FakeLLM::new();
        let response_kube = fake_llm.combine(&kubes).await;
        assert_eq!(
            String::from("waterglass"),
            response_kube.name,
        );
    }
}
