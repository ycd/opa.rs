use std::collections::HashMap;

type Headers = HashMap<String, String>;

pub struct Opa {
    host: String,
    port: i16,
    version: String,
    ssl: bool,
    headers: Option<Headers>,
}

impl Opa {
    pub fn new() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 8181,
            version: "v1".to_string(),
            ssl: false,
            headers: None,
        }
    }

    ///  Get the host
    ///
    ///  Default: localhost
    pub fn host(&self) -> String {
        self.host.clone()
    }

    /// Get the port
    ///
    /// Default: 8181
    pub fn port(&self) -> i16 {
        self.port
    }

    /// Get the version
    ///
    /// Default: v1
    pub fn version(&self) -> String {
        self.version.clone()
    }

    pub fn ssl(&self) -> bool {
        self.ssl
    }

    pub fn headers(&self) -> Option<Headers> {
        self.headers.clone()
    }

    pub(crate) fn ip(&self) -> String {
        format!("http://{}:{}", self.host, self.port)
    }
}
