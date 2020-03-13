mod rate_limit;
use rate_limit::RateLimit;

use crate::error::{DiscordError, Result};
use isahc::{http::StatusCode, prelude::*, HttpClient as IsachClient};

pub(crate) const DISCORD_URL: &'static str = "https://discordapp.com/api";

pub(crate) struct HttpClient {
    token: String,
    client: IsachClient,
    rate_limit: RateLimit,
}

impl HttpClient {
    /// Creates a new http client
    pub(crate) fn new(token: impl Into<String>) -> HttpClient {
        let client = IsachClient::new().expect("Can't create Http Client");
        HttpClient {
            token: token.into(),
            client,
            rate_limit: RateLimit::default(),
        }
    }

    /// This function makes a GET request, and returns the response.
    /// uri: URL where the client will make a GET request.
    /// rt_key: RateLimit key
    pub(crate) async fn get(&self, uri: String, rt_key: String) -> Result<Response<Body>> {
        // Create request
        let req = Request::builder()
            .method("GET")
            .uri(&uri)
            .header("Authorization", &self.token)
            .body(())
            .unwrap();

        // Check and wait if we reach the limit
        self.rate_limit.check_and_sleep(&rt_key).await;

        // Get response
        let res = self
            .client
            .send_async(req)
            .await
            .map_err(|_| DiscordError::HttpNoResponse)?;

        // Catch http errors and return if there is one
        self.catch_http_errors(&res)?;

        // Update the limit with the response headers
        self.rate_limit.update(rt_key, &res).await;

        // if let Err(e) = self.catch_http_error() {
        //      parse error JSON struct
        // }

        Ok(res)
    }

    /// This function makes a POST request, and returns the response.
    pub(crate) async fn post(
        &self,
        uri: String,
        rt_key: String,
        body: impl Into<Body>,
    ) -> Result<Response<Body>> {
        let req = Request::builder()
            .method("POST")
            .uri(uri)
            .header("Authorization", &self.token)
            .header("Content-Type", "application/json")
            .body(body)
            .unwrap();

        // Check and wait if we reach the limit
        self.rate_limit.check_and_sleep(&rt_key).await;

        let res = self
            .client
            .send_async(req)
            .await
            .map_err(|_| DiscordError::HttpNoResponse)?;

        self.catch_http_errors(&res)?;

        // Update the limit with the response headers
        self.rate_limit.update(rt_key, &res).await;
        // if let Err(e) = self.catch_http_error() {
        //      parse error JSON struct
        // }

        Ok(res)
    }

    /// This function makes a PATCH request, and returns the response.
    pub(crate) async fn patch(
        &self,
        uri: String,
        rt_key: String,
        body: impl Into<Body>,
    ) -> Result<Response<Body>> {
        let req = Request::builder()
            .method("PATCH")
            .uri(uri)
            .header("Authorization", &self.token)
            .header("Content-Type", "application/json")
            .body(body)
            .unwrap();

        // Check and wait if we reach the limit
        self.rate_limit.check_and_sleep(&rt_key).await;

        let res = self
            .client
            .send_async(req)
            .await
            .map_err(|_| DiscordError::HttpNoResponse)?;

        self.catch_http_errors(&res)?;

        // Update the limit with the response headers
        self.rate_limit.update(rt_key, &res).await;
        // if let Err(e) = self.catch_http_error() {
        //      parse error JSON struct
        // }

        Ok(res)
    }

    /// This function makes a PATCH request, and returns the response.
    pub(crate) async fn put(&self, uri: String, rt_key: String) -> Result<Response<Body>> {
        let req = Request::builder()
            .method("PUT")
            .uri(uri)
            .header("Authorization", &self.token)
            .body(())
            .unwrap();

        // Check and wait if we reach the limit
        self.rate_limit.check_and_sleep(&rt_key).await;

        let res = self
            .client
            .send_async(req)
            .await
            .map_err(|_| DiscordError::HttpNoResponse)?;

        self.catch_http_errors(&res)?;

        // Update the limit with the response headers
        self.rate_limit.update(rt_key, &res).await;
        // if let Err(e) = self.catch_http_error() {
        //      parse error JSON struct
        // }

        Ok(res)
    }

    /// This function makes a GET request, and returns the response.
    pub(crate) async fn delete(&self, uri: String, rt_key: String) -> Result<Response<Body>> {
        // Create request
        let req = Request::builder()
            .method("DELETE")
            .uri(uri)
            .header("Authorization", &self.token)
            .body(())
            .unwrap();

        // Check and wait if we reach the limit
        self.rate_limit.check_and_sleep(&rt_key).await;

        // Get response
        let res = self
            .client
            .send_async(req)
            .await
            .map_err(|_| DiscordError::HttpNoResponse)?;

        // Catch http errors and return if there is one
        self.catch_http_errors(&res)?;

        // Update the limit with the response headers
        self.rate_limit.update(rt_key, &res).await;

        // if let Err(e) = self.catch_http_error() {
        //      parse error JSON struct
        // }

        Ok(res)
    }

    // TODO: Rename this
    fn catch_http_errors(&self, res: &Response<Body>) -> Result<()> {
        let err = match res.status() {
            StatusCode::OK
            | StatusCode::CREATED
            | StatusCode::NO_CONTENT
            | StatusCode::NOT_MODIFIED
            | StatusCode::TOO_MANY_REQUESTS => return Ok(()),

            StatusCode::BAD_REQUEST => DiscordError::HttpImproperlyFormatted,
            StatusCode::FORBIDDEN => DiscordError::HttpForbidden, // no autorizado
            StatusCode::NOT_FOUND => DiscordError::HttpInvalidParameters, // not found or bad format
            StatusCode::METHOD_NOT_ALLOWED => DiscordError::HttpNoResponse, // method not allowed
            // HANDLED BY RATELIMIT StatusCode::TOO_MANY_REQUESTS => DiscordError::HttpNoResponse, // too many requests
            StatusCode::BAD_GATEWAY => DiscordError::HttpNoResponse, // gateway unavailable
            _ => DiscordError::HttpNoResponse,
        };

        Err(err)

        //TODO: Remove HttpNoResponse here
    }
}
