use reqwest;
use http::{HeaderMap, Method};
use reqwest::{Response, Url, Client};
use reqwest::Body;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/39.0.2171.71 Safari/537.36";


#[derive(Debug)]
pub struct GithubHttpClient {
    pub client: Client,
}

impl GithubHttpClient {
    pub fn new() -> GithubHttpClient {
        let async_client = reqwest::Client::builder()
            .use_native_tls()
            .user_agent(USER_AGENT)
            .build().unwrap();
        GithubHttpClient {
            client: async_client
        }
    }
}

impl Default for GithubHttpClient {
    fn default() -> Self {
        let async_client = reqwest::Client::builder()
            .use_native_tls()
            .user_agent(USER_AGENT)
            .build().unwrap();
        GithubHttpClient {
            client: async_client
        }
    }
}

impl GithubHttpClient {
    pub async fn fetch_url(&self, url: &String, method: &str, body: Body, header: &HeaderMap) -> Result<Response> {
        let mut req = reqwest::Request::new(
            Method::from_bytes(method.as_bytes())?,
            Url::parse(url)?);
        let _ = req.body_mut().insert(body);
        for (k, v) in header.iter() {
            req.headers_mut().insert(k.clone(), v.clone());
        }
        let res = self.client.execute(req).await?;

        Ok(res)
    }
}


#[cfg(test)]
mod test {
    use http::HeaderMap;
    use reqwest::Body;
    use tokio_test;
    use super::*;

    #[test]
    fn test_fetch_uri() {
        let client = GithubHttpClient::default();
        let token: &str = "*****";
        let owner: &str = "smartcat999";
        let repo: &str = "issue_auto_report";
        let url: String = format!("https://api.github.com/repos/{}/{}/issues", owner, repo);

        let body = r#"{"title":"CVE-2023-25173","body":"","assignees":["smartcat999"],"milestone": null,"labels":["bug"]}"#;
        let body = Body::from(body);
        let mut header = HeaderMap::new();
        header.insert(http::header::ACCEPT, http::header::HeaderValue::from_str("application/vnd.github+json").unwrap());
        header.insert(http::header::AUTHORIZATION, http::header::HeaderValue::from_str(format!("Bearer {}", token).as_str()).unwrap());
        header.insert("X-GitHub-Api-Version", http::header::HeaderValue::from_str("2022-11-28").unwrap());
        let result = client.fetch_url(&url, "POST", body, &header);
        tokio_test::block_on(result).expect("fetch error");
    }
}