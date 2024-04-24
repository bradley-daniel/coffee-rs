// pub struct HttpResponse {
//     headers: Headers,
//     body: Body,
//
// }
//
// pub struct Headers {
//     method: Method,
//     url: String,
//     version: String,
// }
//
//
// // TODO: Add better implementation of conetent to include jsons
// pub struct Body {
//     content: String,
// }
//
// pub enum Method {
//     GET,
//     POST,
// }

pub struct Request {
    pub method: Method,
    pub endpoint: String,
    pub version: String,
}

impl From<Vec<String>> for Request {
    fn from(value: Vec<String>) -> Self {
        let info: Vec<_> = value[0].split(' ').collect();
        Request {
            method: info[0].into(),
            endpoint: info[1].into(),
            version: info[2].into(),
        }
    }
}
pub enum Method {
    GET,
    POST,
}

impl From<String> for Method {
    fn from(value: String) -> Self {
        value.as_str().into()
    }
}

impl From<&str> for Method {
    fn from(value: &str) -> Self {
        match value {
            "GET" => Self::GET,
            "POST" => Self::POST,
            _ => unreachable!("Unable to parse route type {}", value),
        }
    }
}
