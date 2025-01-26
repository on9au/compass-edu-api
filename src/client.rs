use std::sync::Arc;

use tokio::sync::RwLock;
use url::Url;

const BASE_URL: &str = "compass.education";

/// The Compass Education API client.
///
/// This struct contains the base URL of the school and a reqwest client.
///
/// You do **not** have to wrap the `Client` in an [`Rc`] or [`Arc`] to **reuse** it,
/// because it already uses an [`Arc`] internally.
#[derive(Debug, Clone)]
pub struct Client {
    pub(crate) inner: Arc<RwLock<CompassEduClientRef>>,
}

impl Client {
    /// Creates a new `Client` with the given school prefix.
    pub fn new<S: Into<String>>(school_prefix: S, cookie: S) -> Self {
        let base_url = format!("https://{}.{}", school_prefix.into(), BASE_URL);

        // Create headers
        let mut headers = reqwest::header::HeaderMap::new();

        headers.insert(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_static("*/*"),
        );

        headers.insert(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"),
        );

        headers.insert(
            reqwest::header::ACCEPT_ENCODING,
            reqwest::header::HeaderValue::from_static("gzip, deflate"),
        );

        headers.insert(
            reqwest::header::USER_AGENT,
            reqwest::header::HeaderValue::from_static(
                "iOS/14_6_0 type/iPhone CompassEducation/6.3.0",
            ), // we are definitely the compass app
        );

        headers.insert(
            reqwest::header::ACCEPT_LANGUAGE,
            reqwest::header::HeaderValue::from_static("en-au"),
        );

        headers.insert(
            reqwest::header::CONNECTION,
            reqwest::header::HeaderValue::from_static("close"),
        );

        // Add session cookie into jar
        let jar = reqwest::cookie::Jar::default();
        let cookie = format!("ASP.NET_SessionId={}", cookie.into());
        let cookie = cookie.as_str();
        jar.add_cookie_str(cookie, &base_url.parse::<Url>().unwrap());

        Self {
            inner: Arc::new(RwLock::new(CompassEduClientRef {
                base_url: format!("{}/Services", base_url),
                user: None,
                client: reqwest::Client::builder()
                    .cookie_store(true)
                    .default_headers(headers)
                    .gzip(true)
                    .build()
                    .unwrap(),
            })),
        }
    }
}

/// The Compass Education API client reference. Contains the base URL of the school and a reqwest client.
/// Inner struct of [`Client`]. Used for thread-safe access and cheap cloning.
#[derive(Debug)]
pub struct CompassEduClientRef {
    /// **WARNING:** This field NEVER ends with a `/`.
    pub(crate) base_url: String,
    /// User
    pub(crate) user: Option<crate::models::Account>,
    /// Reqwest client.
    pub(crate) client: reqwest::Client,
}
