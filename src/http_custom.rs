
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE
}

pub fn http_method_resolve(method: HttpMethod) -> &'static [u8] {
    return match method {
        HttpMethod::GET => b"GET / HTTP/1.1",
        HttpMethod::POST => b"POST / HTTP/1.1",
        HttpMethod::PUT => b"PUT / HTTP/1.1",
        HttpMethod::DELETE => b"DELETE / HTTP1.1",
    };
}

pub enum HttpStatusCode {
    RP_200, 
    RP_201,
    RP_400,
    RP_401,
    RP_404,
    RP_500
}

pub fn http_status_code_resolve(status_code: HttpStatusCode) -> String {
    return match status_code {
        HttpStatusCode::RP_200 => String::from("200 OK"),
        HttpStatusCode::RP_201 => String::from("201 Created"),
        HttpStatusCode::RP_400 => String::from("400 Bad Request"),
        HttpStatusCode::RP_401 => String::from("401 Unauthorized"),
        HttpStatusCode::RP_404 => String::from("404 Not Found"),
        HttpStatusCode::RP_500 => String::from("500 Internal Server Error"),
    };
}
