use crate::kube::{Kube, KubeId};
use crate::recipe::Recipe;
use serde::Deserialize;
use thiserror::Error;
use reqwest::Url;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum EnvVarParseError {
    #[error("Error while getting environment variable. Have you included an equals sign?")]
    EnvironmentError(#[from] std::env::VarError),
    #[error("Failed to parse URL, is it valid?")]
    UrlParseError(#[from] url::ParseError)
}

#[derive(Debug, Error)]
pub enum RestError {
    #[error("Error while trying to reach REST server.")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Could not parse JSON.")]
    SerdeError(#[from] serde_json::Error),
}

#[derive(Debug)]
pub struct CacheClient {
    url: Url,
}
impl CacheClient {
    pub fn new() -> Result<CacheClient, EnvVarParseError> {
        let url_string = std::env::var("ENDPOINT")?;
        Ok(CacheClient {
            url: Url::parse(&url_string)?,
        })
    }
    pub fn from_url(url: &str) -> Result<CacheClient, url::ParseError> {
        Ok(CacheClient {
            url: Url::parse(url)?,
        })
    }
    pub async fn get_kubes(&self) -> Result<Vec<Kube>, RestError> {
        let res = reqwest::get(self.url.join("kubes").unwrap()).await?.text().await?;
        Ok(serde_json::from_str(&res)?)
    }
    pub async fn get_recipes(&self) -> Result<Vec<Recipe>, RestError> {
        let res = reqwest::get(self.url.join("kubeRecipes").unwrap()).await?.text().await?;
        Ok(serde_json::from_str(&res)?)
    }
    pub async fn get_kube_by_id(&self, id: KubeId) -> Result<Kube, RestError> {
        let res = reqwest::get(self.url.join(format!("kube/:{}", id.uuid()).as_str()).unwrap()).await?.text().await?;
        Ok(serde_json::from_str(&res)?)
    }
    pub async fn get_recipe_by_id(&self, id1: Uuid, id2: Uuid) -> Result<Recipe, RestError> {
        let res = reqwest::get(self.url.join(format!("kubeRecipeByIds/:{}/:{}", id1, id2).as_str()).unwrap()).await?.text().await?;
        Ok(serde_json::from_str(&res)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn get_kubes() {
        let client = CacheClient::from_url("https://hack.djpiper28.co.uk/cache/").unwrap();
        let kubes = client.get_kubes().await.unwrap();
        let control_kubes_string = String::from(
"[{\"name\":\"hydrogen\",\"id\":\"153227c6-c069-4748-b5aa-aafac8abef00\"},{\"name\":\"oxygen\",\"id\":\"3ffe4c9e-5d35-42c9-a70e-1d80c544bdbb\"},{\"name\":\"nitrogen\",\"id\":\"bb155bf1-7200-45e7-b126-f2882f7aecaa\"},{\"name\":\"calcium\",\"id\":\"5d75a6e7-b45b-4c5f-ae14-042ad17b3156\"},{\"name\":\"iron\",\"id\":\"a0e89ba5-41cb-4a8d-895e-866f0b2004f2\"},{\"name\":\"aluminium\",\"id\":\"f475d09e-75e7-4096-af5b-a00de9ae5e48\"},{\"name\":\"uranium\",\"id\":\"615b14af-d4a8-4e57-8a6c-190110114ad1\"},{\"name\":\"sodium\",\"id\":\"485cbd9d-5608-4b6a-afbe-52829663090d\"},{\"name\":\"chlorine\",\"id\":\"dbe34fef-e969-4f4d-b148-7898d6de699c\"},{\"name\":\"light\",\"id\":\"0652bcac-e87b-4323-9d05-939ddfd1c726\"},{\"name\":\"time\",\"id\":\"efd60b9a-dcc4-4690-a087-c1aba4a7d16e\"},{\"name\":\"silicon\",\"id\":\"c37997c9-e7ba-44d6-b726-97aec89545f8\"},{\"name\":\"water\",\"id\":\"bf1ed678-3be7-42d7-8544-82124b6c6111\"},{\"name\":\"tap water\",\"id\":\"7bc0ee7b-6772-4315-84dc-93887e007bd6\"},{\"name\":\"salt\",\"id\":\"d24a2af2-eed2-40af-a064-001de3545e84\"},{\"name\":\"sea water\",\"id\":\"c622c644-1590-4e4f-b7ae-42fb358ad004\"},{\"name\":\"air\",\"id\":\"ad2ed248-7bc8-469e-b9bb-7b1bc4878cad\"},{\"name\":\"rust\",\"id\":\"1c213d9d-b060-45c5-a8e0-48c45b01c30b\"},{\"name\":\"feldspar\",\"id\":\"9796d796-d166-4a7e-895a-fd9bf65dd3ee\"},{\"name\":\"sand\",\"id\":\"2d727d5b-a71c-49c3-84ef-cffa2fe47f9a\"},{\"name\":\"dirt\",\"id\":\"b4a9dc42-273a-4dc6-9c76-c51a847b743c\"},{\"name\":\"beach\",\"id\":\"e7dcaab2-1c0a-4af2-a5a5-38995f4c0167\"},{\"name\":\"earth\",\"id\":\"39807784-9d57-48fc-8e7a-e877e975eb9c\"},{\"name\":\"life\",\"id\":\"dc383720-1649-4119-ad0a-68f209b237d5\"},{\"name\":\"age\",\"id\":\"181ec200-54aa-4dca-821d-b88efe6b129d\"},{\"name\":\"energy\",\"id\":\"abc9bcf7-47d2-46f0-bf6e-f70a126820b7\"},{\"name\":\"rock\",\"id\":\"0b3ab01f-4fc6-4bc8-8acc-5ae2a7fb640f\"},{\"name\":\"fire\",\"id\":\"bbee222a-4f4a-477a-8be8-2e6f314b5a36\"},{\"name\":\"glass\",\"id\":\"f84b34f7-5406-4795-a56d-9d07c4b3c9d5\"},{\"name\":\"steam\",\"id\":\"a38fd90e-c2ff-4955-9161-462018dfaf52\"},{\"name\":\"radioactive water\",\"id\":\"6bffa307-4114-40ff-bf43-6934945b1753\"},{\"name\":\"nuclear reactor\",\"id\":\"021b0835-e16e-4163-ac66-4f7c7b78ce7e\"}]"
        );
        let control_kubes: Vec<Kube> = serde_json::from_str(&control_kubes_string).unwrap();
        assert_eq!(control_kubes, kubes)
    }
}
