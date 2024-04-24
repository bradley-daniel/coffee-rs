use crate::status_code::StatusCode;

pub struct Response {
    pub status: StatusCode,
    pub version: String,
    pub content: String,
}
impl From<Response> for String {
    fn from(value: Response) -> Self {
        let status_line = format!("{} {}", value.version, value.status);
        format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status_line,
            value.content.len(),
            value.content,
        )
    }
}

pub trait IntoResponse {
    fn into_resonse(self) -> Response;
}

impl IntoResponse for () {
    fn into_resonse(self) -> Response {
        Response {
            status: StatusCode::Ok,
            version: "HTTP/1.1 200".into(),
            content: "".into(),
        }
    }
}

impl IntoResponse for &str {
    fn into_resonse(self) -> Response {
        Response {
            status: StatusCode::Ok,
            version: "HTTP/1.1 200".into(),
            content: self.to_string(),
        }
    }
}

impl IntoResponse for String {
    fn into_resonse(self) -> Response {
        Response {
            status: StatusCode::Ok,
            version: "HTTP/1.1 200".into(),
            content: self,
        }
    }
}
