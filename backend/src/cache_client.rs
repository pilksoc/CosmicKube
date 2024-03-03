use crate::kube::{Kube};
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
    pub async fn get_kube_by_id(&self, id: Uuid) -> Result<Kube, RestError> {
        let res = reqwest::get(self.url.join(format!("kubeById/{}", id).as_str()).unwrap()).await?.text().await?;
        Ok(serde_json::from_str(&res)?)
    }
    pub async fn get_recipe_by_id(&self, id1: Uuid, id2: Uuid) -> Result<Recipe, RestError> {
        let res = reqwest::get(self.url.join(format!("kubeRecipeByIds/{}/{}", id1, id2).as_str()).unwrap()).await?.text().await?;
        Ok(serde_json::from_str(&res)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn try_get_kubes() {
        let client = CacheClient::from_url("https://hack.djpiper28.co.uk/cache/").unwrap();
        let mut kubes = client.get_kubes().await.unwrap();
        let control_kubes_string = String::from(
"[{\"name\":\"hydrogen\",\"id\":\"cdede93f-d0d7-4b4a-9fde-2a909444c58d\"},{\"name\":\"oxygen\",\"id\":\"8ddcf7ad-61f6-47ff-a49c-4abcec21d6a1\"},{\"name\":\"nitrogen\",\"id\":\"59a64f5b-bcf4-4d2d-bb7f-bc4eceaf41e5\"},{\"name\":\"calcium\",\"id\":\"2b006956-063d-4ca2-b75d-f6e5d67455c9\"},{\"name\":\"iron\",\"id\":\"5e930e14-4597-49b3-95fa-3e6dcc40b1ae\"},{\"name\":\"aluminium\",\"id\":\"d076033a-c583-4d38-8094-249a7fe2972b\"},{\"name\":\"uranium\",\"id\":\"82ac0ed4-62e3-4c5e-af3e-024c38285227\"},{\"name\":\"sodium\",\"id\":\"1c7fda1b-af90-411d-8162-8fd04c4890d3\"},{\"name\":\"chlorine\",\"id\":\"061f6efd-0067-4d71-92b8-a0b58562b906\"},{\"name\":\"light\",\"id\":\"e38ba705-58c1-469d-8eff-7cc01b94fd46\"},{\"name\":\"time\",\"id\":\"6991989a-f347-48eb-8c67-ade4cdc010d0\"},{\"name\":\"silicon\",\"id\":\"72650f96-011b-404d-aba0-2c8aa2f17aeb\"},{\"name\":\"water\",\"id\":\"8cf89e77-bf8b-4cf4-9941-46b853df4480\"},{\"name\":\"tap water\",\"id\":\"5ca230bc-135e-4be8-8be3-7c7c1b3e5484\"},{\"name\":\"salt\",\"id\":\"540710d4-5d7d-42f6-b9b7-aad181affbcf\"},{\"name\":\"sea water\",\"id\":\"74102256-00b5-42aa-9665-1bc81f13c18b\"},{\"name\":\"air\",\"id\":\"88bb179c-0d91-4322-a4e5-d3862bf83a31\"},{\"name\":\"rust\",\"id\":\"ad92587b-1643-469e-8357-1d6ec5ab6380\"},{\"name\":\"feldspar\",\"id\":\"2adfa430-faa4-4ecd-95e1-c6cc8c85f3b5\"},{\"name\":\"sand\",\"id\":\"1db355de-1404-4e7c-bf8b-a6b3355b9dc4\"},{\"name\":\"dirt\",\"id\":\"649e8325-530b-4abb-8fe0-5b79b395e84f\"},{\"name\":\"beach\",\"id\":\"03f9f164-be03-4de3-b5b4-c7549c1ef9e4\"},{\"name\":\"earth\",\"id\":\"66744f80-ccec-4c8b-9025-9edbb75df0a6\"},{\"name\":\"life\",\"id\":\"abd2f9a5-34cd-4b3f-941f-8cf934f6b967\"},{\"name\":\"age\",\"id\":\"bda3c6c5-3b79-418d-8d24-cc533a509065\"},{\"name\":\"energy\",\"id\":\"b4eba917-2179-4cd4-a64e-316fe005f11e\"},{\"name\":\"rock\",\"id\":\"3c545935-1382-4c3d-9771-1fa8492f1b77\"},{\"name\":\"fire\",\"id\":\"1e2e14df-f36b-43a4-9a2a-5112f84abb52\"},{\"name\":\"glass\",\"id\":\"12ec3f8d-0986-42d7-acc2-e6af8ba1842a\"}]"
        );
        let mut control_kubes: Vec<Kube> = serde_json::from_str(&control_kubes_string).unwrap();
        kubes.sort();
        control_kubes.sort();
        assert_eq!(control_kubes, kubes)
    }
    #[tokio::test]
    async fn try_kube_by_id() {
        let client = CacheClient::from_url("https://hack.djpiper28.co.uk/cache/").unwrap();
        let kube = client.get_kube_by_id(Uuid::parse_str("5e930e14-4597-49b3-95fa-3e6dcc40b1ae").unwrap()).await.unwrap();
        let expected_string = String::from("{\"name\":\"iron\",\"id\":\"5e930e14-4597-49b3-95fa-3e6dcc40b1ae\"}");
        let expected_kube: Kube = serde_json::from_str(&expected_string).unwrap();
        assert_eq!(expected_kube, kube);
    }
}
