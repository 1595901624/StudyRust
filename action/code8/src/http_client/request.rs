use std::collections::HashMap;
use reqwest::blocking::RequestBuilder;

#[derive(Debug)]
pub struct RequestBody {
    pub query: Option<Vec<(String, String)>>,
    pub form: Option<Vec<(String, String)>>,
    pub json: Option<Vec<(String, String)>>,
}

impl RequestBody {
    pub(crate) fn new(query: Option<Vec<(String, String)>>,
                      form: Option<Vec<(String, String)>>,
                      json: Option<Vec<(String, String)>>) -> RequestBody {
        RequestBody {
            query,
            form,
            json,
        }
    }
}

pub trait RequestBodyBuilder {
    /// 添加请求体
    fn add_body(self, request_body: &RequestBody) -> RequestBuilder;
}

impl RequestBodyBuilder for RequestBuilder {
    fn add_body(self, request_body: &RequestBody) -> RequestBuilder {
        let mut request_builder = self;
        if let Some(query) = &request_body.query {
            for (key, value) in query {
                request_builder = request_builder.query(&[(key, value)]);
            }
        }
        if let Some(form) = &request_body.form {
            for (key, value) in form {
                request_builder = request_builder.form(&[(key, value)]);
            }
        }
        if let Some(json) = &request_body.json {
            let mut map = HashMap::with_capacity(json.len());
            for (key, value) in json {
                map.insert(key, value);
            }
            request_builder = request_builder.json(&map);
        }
        request_builder
    }
}