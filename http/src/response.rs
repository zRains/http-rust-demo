use std::{collections::HashMap, error::Error, io::Write};

use crate::request::Version;

#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum HttpStatus {
    CONTINUE = 100,
    SWITCHING_PROTOCOLS = 101,
    PROCESSING = 102,
    EARLYHINTS = 103,
    OK = 200,
    CREATED = 201,
    ACCEPTED = 202,
    NON_AUTHORITATIVE_INFORMATION = 203,
    NO_CONTENT = 204,
    RESET_CONTENT = 205,
    PARTIAL_CONTENT = 206,
    AMBIGUOUS = 300,
    MOVED_PERMANENTLY = 301,
    FOUND = 302,
    SEE_OTHER = 303,
    NOT_MODIFIED = 304,
    TEMPORARY_REDIRECT = 307,
    PERMANENT_REDIRECT = 308,
    BAD_REQUEST = 400,
    UNAUTHORIZED = 401,
    PAYMENT_REQUIRED = 402,
    FORBIDDEN = 403,
    NOT_FOUND = 404,
    METHOD_NOT_ALLOWED = 405,
    NOT_ACCEPTABLE = 406,
    PROXY_AUTHENTICATION_REQUIRED = 407,
    REQUEST_TIMEOUT = 408,
    CONFLICT = 409,
    GONE = 410,
    LENGTH_REQUIRED = 411,
    PRECONDITION_FAILED = 412,
    PAYLOAD_TOO_LARGE = 413,
    URI_TOO_LONG = 414,
    UNSUPPORTED_MEDIA_TYPE = 415,
    REQUESTED_RANGE_NOT_SATISFIABLE = 416,
    EXPECTATION_FAILED = 417,
    I_AM_A_TEAPOT = 418,
    MISDIRECTED = 421,
    UNPROCESSABLE_ENTITY = 422,
    FAILED_DEPENDENCY = 424,
    PRECONDITION_REQUIRED = 428,
    TOO_MANY_REQUESTS = 429,
    INTERNAL_SERVER_ERROR = 500,
    NOT_IMPLEMENTED = 501,
    BAD_GATEWAY = 502,
    SERVICE_UNAVAILABLE = 503,
    GATEWAY_TIMEOUT = 504,
    HTTP_VERSION_NOT_SUPPORTED = 505,
}

impl Into<(u16, String)> for &HttpStatus {
    fn into(self) -> (u16, String) {
        match self {
            HttpStatus::CONTINUE => (100, "Continue".into()),
            HttpStatus::SWITCHING_PROTOCOLS => (101, "Switching Protocols".into()),
            HttpStatus::PROCESSING => (102, "Processing".into()),
            HttpStatus::EARLYHINTS => (103, "Earlyhints".into()),
            HttpStatus::OK => (200, String::from("Ok")),
            HttpStatus::CREATED => (201, "Created".into()),
            HttpStatus::ACCEPTED => (202, "Accepted".into()),
            HttpStatus::NON_AUTHORITATIVE_INFORMATION => {
                (203, "Non Authoritative Information".into())
            }
            HttpStatus::NO_CONTENT => (204, "No Content".into()),
            HttpStatus::RESET_CONTENT => (205, "Reset Content".into()),
            HttpStatus::PARTIAL_CONTENT => (206, "Partial Content".into()),
            HttpStatus::AMBIGUOUS => (300, "Ambiguous".into()),
            HttpStatus::MOVED_PERMANENTLY => (301, "Moved Permanently".into()),
            HttpStatus::FOUND => (302, "Found".into()),
            HttpStatus::SEE_OTHER => (303, "See Other".into()),
            HttpStatus::NOT_MODIFIED => (304, "Not Modfifed".into()),
            HttpStatus::TEMPORARY_REDIRECT => (307, "Temporary Redirect".into()),
            HttpStatus::PERMANENT_REDIRECT => (308, "Permanent Redirect".into()),
            HttpStatus::BAD_REQUEST => (400, "Bad Request".into()),
            HttpStatus::UNAUTHORIZED => (401, "Unauthorized".into()),
            HttpStatus::PAYMENT_REQUIRED => (402, "Payment Required".into()),
            HttpStatus::FORBIDDEN => (403, "Forbidden".into()),
            HttpStatus::NOT_FOUND => (404, "Not Found".into()),
            HttpStatus::METHOD_NOT_ALLOWED => (405, "Method Not Allowed".into()),
            HttpStatus::NOT_ACCEPTABLE => (406, "Not Acceptable".into()),
            HttpStatus::PROXY_AUTHENTICATION_REQUIRED => {
                (407, "Proxy Authentication Required".into())
            }
            HttpStatus::REQUEST_TIMEOUT => (408, "Request Timeout".into()),
            HttpStatus::CONFLICT => (409, "Conflict".into()),
            HttpStatus::GONE => (401, "Gone".into()),
            HttpStatus::LENGTH_REQUIRED => (411, "Length Required".into()),
            HttpStatus::PRECONDITION_FAILED => (412, "Precondition Failed".into()),
            HttpStatus::PAYLOAD_TOO_LARGE => (413, "Payload Too Large".into()),
            HttpStatus::URI_TOO_LONG => (414, "URI Too Long".into()),
            HttpStatus::UNSUPPORTED_MEDIA_TYPE => (415, "Unsupported Media Type".into()),
            HttpStatus::REQUESTED_RANGE_NOT_SATISFIABLE => {
                (416, "Required Range Not Satisfiable".into())
            }
            HttpStatus::EXPECTATION_FAILED => (417, "Expectation Failed".into()),
            HttpStatus::I_AM_A_TEAPOT => (418, "I Am A Teapot".into()),
            HttpStatus::MISDIRECTED => (421, "Misdirected".into()),
            HttpStatus::UNPROCESSABLE_ENTITY => (422, "Unprocessable Entity".into()),
            HttpStatus::FAILED_DEPENDENCY => (424, "Failed Dependency".into()),
            HttpStatus::PRECONDITION_REQUIRED => (428, "Precondition Required".into()),
            HttpStatus::TOO_MANY_REQUESTS => (429, "Too Many Requests".into()),
            HttpStatus::INTERNAL_SERVER_ERROR => (500, "Internal Server Error".into()),
            HttpStatus::NOT_IMPLEMENTED => (501, "Not Implemented".into()),
            HttpStatus::BAD_GATEWAY => (502, "Bad Gateway".into()),
            HttpStatus::SERVICE_UNAVAILABLE => (503, "Service Unavailable".into()),
            HttpStatus::GATEWAY_TIMEOUT => (504, "Gateway Timeout".into()),
            HttpStatus::HTTP_VERSION_NOT_SUPPORTED => (505, "Http Version Not Supported".into()),
        }
    }
}

pub struct HttpResponse {
    version: Version,
    status_code: HttpStatus,
    headers: Option<HashMap<String, String>>,
    body: Option<String>,
}

impl Default for HttpResponse {
    fn default() -> HttpResponse {
        HttpResponse {
            version: Version::V1_1,
            status_code: HttpStatus::OK,
            headers: None,
            body: None,
        }
    }
}

impl From<&HttpResponse> for String {
    fn from(response: &HttpResponse) -> String {
        let status: (u16, String) = response.status_code().into();
        let version: String = response.version().into();
        let headers = response.headers();
        let body = response.body();

        format!(
            "{} {} {}\r\n{}Content-Length: {}\r\n\r\n{}",
            version,
            status.0,
            status.1,
            headers,
            body.len(),
            body
        )
    }
}

impl HttpResponse {
    pub fn new(
        version: Version,
        status_code: HttpStatus,
        headers: Option<HashMap<String, String>>,
        body: Option<String>,
    ) -> HttpResponse {
        let mut response = HttpResponse::default();

        response.version = version;
        response.status_code = status_code;
        response.body = body;
        response.headers = match headers {
            Some(h) => Some(h),
            None => {
                let mut h = HashMap::new();

                h.insert("Content-Type".into(), "text/html".into());

                Some(h)
            }
        };

        response
    }

    pub fn send(&self, write_stream: &mut impl Write) -> Result<(), Box<dyn Error>> {
        let response_str: String = self.into();

        write!(write_stream, "{}", response_str).unwrap();

        Ok(())
    }

    fn version(&self) -> &Version {
        &self.version
    }

    fn status_code(&self) -> &HttpStatus {
        &self.status_code
    }

    fn headers(&self) -> String {
        let mut map = self.headers.clone().unwrap();
        let mut header_str = "".into();

        map.insert("Server".into(), "nginx/1.14.1".into());

        for (k, v) in map.iter() {
            header_str = format!("{}{}: {}\r\n", header_str, k, v);
        }

        header_str
    }

    fn body(&self) -> &str {
        if let Some(b) = &self.body {
            b
        } else {
            ""
        }
    }
}

#[cfg(test)]
mod response_test {
    use super::*;

    #[test]
    fn test_http_status_resolve() {
        let status: (u16, String) = (&HttpStatus::OK).into();
        assert_eq!(status, (200, "Ok".into()));

        let status: (u16, String) = (&HttpStatus::I_AM_A_TEAPOT).into();
        assert_eq!(status, (418, "I Am A Teapot".into()));
    }

    #[test]
    fn test_response_parse() {
        let response_with_no_headers: String =
            (&HttpResponse::new(Version::V1_1, HttpStatus::OK, None, None)).into();

        assert_eq!(
            String::from("HTTP/1.1 200 Ok\r\nContent-Type: text/html\r\nContent-Length: 0\r\n\r\n"),
            response_with_no_headers
        );

        let response_with_body: String = (&HttpResponse::new(
            Version::V1_1,
            HttpStatus::OK,
            None,
            Some("Hello I Am zRain".into()),
        ))
            .into();

        assert_eq!(String::from("HTTP/1.1 200 Ok\r\nContent-Type: text/html\r\nContent-Length: 16\r\n\r\nHello I Am zRain"), response_with_body);

        let mut test_header_map: HashMap<String, String> = HashMap::new();
        let mut header_str: String = "".into();

        test_header_map.insert("Server".into(), "nginx/1.14.1".into());
        test_header_map.insert("Date".into(), "Sat, 24 Sep 2022 03:15:53 GMT".into());
        test_header_map.insert("Connection".into(), "keep-alive".into());

        for (k, v) in test_header_map.iter() {
            header_str = format!("{}{}: {}\r\n", header_str, k, v);
        }

        let response_width_headers: String =
            (&HttpResponse::new(Version::V1_1, HttpStatus::OK, Some(test_header_map), None)).into();

        assert_eq!(
            format!("HTTP/1.1 200 Ok\r\n{}Content-Length: 0\r\n\r\n", header_str),
            response_width_headers
        );
    }
}
