#[cfg(test)]
mod tests {
    use super::RequestBuilder;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn prints() {
        let req =
            RequestBuilder::get("www.google.ac.kr")
                .query("a","b")
                .header_raw("c","d")
                .header_raw("e","f")
                .content_type_json()
                .accept_json()
                .build();
        println!("{:?}", req);
    }
}

extern crate bincode;
extern crate hyper;
extern crate serde;
extern crate serde_json;

pub use hyper::Request;
pub use hyper::Method;
pub use hyper::Headers;
use std::collections::HashMap;
use serde::Serialize;

pub struct RequestBuilder {
    method: Method,
    uri: String,
    headers: Headers,
    queries: HashMap<String, String>,
}

static HEADER_CONTENT_TYPE: &'static str = "Content-Type";

impl RequestBuilder {
    fn new<S: Into<String>>(method: hyper::Method, uri: S) -> RequestBuilder {
        RequestBuilder {
            method: method,
            uri: uri.into(),
            headers: Headers::new(),
            queries: HashMap::new(),
        }
    }
    pub fn get<S: Into<String>>(uri: S) -> RequestBuilder {
        RequestBuilder::new(Method::Get, uri)
    }
    pub fn post<S: Into<String>>(uri: S) -> RequestBuilder {
        RequestBuilder::new(Method::Post, uri)
    }
    pub fn put<S: Into<String>>(uri: S) -> RequestBuilder {
        RequestBuilder::new(Method::Put, uri)
    }
    pub fn delete<S: Into<String>>(uri: S) -> RequestBuilder {
        RequestBuilder::new(Method::Delete, uri)
    }
    pub fn head<S: Into<String>>(uri: S) -> RequestBuilder {
        RequestBuilder::new(Method::Head, uri)
    }
    pub fn build(self) -> Request {
        let uri = self.uri.parse().unwrap();
        let mut req = hyper::Request::new(self.method, uri);
        *(req.headers_mut()) = self.headers;
        req
    }
}

// query
impl RequestBuilder {
    pub fn clear_query(mut self) -> Self {
        self.queries.clear();
        self
    }

    /// set one query parameter
    pub fn query<S1: Into<String>, S2: Into<String>>(mut self, key: S1, value: S2) -> Self {
        self.queries.insert(key.into(), value.into());
        self
    }

    /// set multiple query parameters with serialize struct
    pub fn query_from_object<T>(mut self, object: T) -> Self where T: Serialize {
        let map: HashMap<String, String> = bincode::deserialize(&bincode::serialize(&object).unwrap()).unwrap();
        self.queries.extend(map.into_iter());
        self
    }

    /// set multiple query parameters with json object
    pub fn query_from_json<T>(mut self, json: serde_json::Value) -> Self where T: Serialize {
        if let serde_json::Value::Object(map) = json {
            for (k, v) in map.into_iter() {
                let v = match v {
                    serde_json::Value::Number(n) => n.to_string(),
                    serde_json::Value::String(s) => s,
                    serde_json::Value::Null => String::new(),
                    _ => panic!("nested json is provided. please flat"),
                };
                self.queries.insert(k, v);
            }
        } else {
            panic!("serde_json");
        }

        self
    }
}

// header
impl RequestBuilder {
    pub fn clear_header(mut self) -> Self {
        self.headers.clear();
        self
    }

    pub fn header<H: hyper::header::Header>(mut self, header: H) -> Self {
        self.headers.set(header);
        self
    }
    pub fn header_raw<S1: Into<String>, S2: Into<String>>(mut self, key: S1, value: S2) -> Self {
        self.headers.set_raw(key.into(), value.into());
        self
    }

    // request types
    // https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types/Complete_list_of_MIME_types

    pub fn content_type<S: Into<String>>(mut self, raw: S) -> Self {
        self.headers.set_raw(HEADER_CONTENT_TYPE, raw.into());
        self
    }

    pub fn content_type_json(mut self) -> Self {
        self.headers.set(hyper::header::ContentType::json());
        self
    }

    pub fn content_type_x_www_form_urlencoded(mut self) -> Self {
        self.headers.set_raw(HEADER_CONTENT_TYPE, "application/x-www-form-urlencoded");
        self
    }

    // accept type
    pub fn accept<S: Into<String>>(mut self, raw: S) -> Self {
        self.headers.set_raw("Accept", raw.into());
        self
    }
    pub fn accept_json(mut self) -> Self {
        self.headers.set_raw("Accept", "application/json");
        self
    }
}
