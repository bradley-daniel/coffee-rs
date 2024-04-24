use std::fmt::Display;

#[derive(Debug, Copy, Clone)]
pub enum StatusCode {
    // 200-299
    Ok = 200,

    //400-499
    Forbiden = 403,
    NotFound = 404,
}

impl StatusCode {
    pub fn is_error(&self) -> bool {
        matches!(self, StatusCode::Forbiden | StatusCode::NotFound)
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error_string = match self {
            StatusCode::Ok => "OK",
            StatusCode::Forbiden => "Forbidden",
            StatusCode::NotFound => "Not Found",
        };
        write!(f, "{code} {text}", code = *self as u32, text = error_string)
    }
}
