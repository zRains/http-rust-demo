use regex::Regex;
use std::collections::HashMap;
use std::process;

/// Http methods
///
/// include
/// - GET
/// - POST
#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Uninitialized,
}

impl From<&str> for Method {
    fn from(str: &str) -> Method {
        match str {
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => Method::Uninitialized,
        }
    }
}

/// Http versions
///
/// include
/// - HTTP/1.0
/// - HTTP/1.1
/// - HTTP/2.0
#[derive(Debug, PartialEq)]
pub enum Version {
    V1_0,
    V1_1,
    V2_0,
    Uninitialized,
}

impl From<&str> for Version {
    fn from(str: &str) -> Version {
        match str {
            "HTTP/1.0" => Version::V1_0,
            "HTTP/1.1" => Version::V1_1,
            "HTTP/2.0" => Version::V2_0,
            _ => Version::Uninitialized,
        }
    }
}

impl From<&Version> for String {
    fn from(version: &Version) -> String {
        match version {
            Version::V1_0 => "HTTP/1.0".into(),
            Version::V1_1 => "HTTP/1.1".into(),
            Version::V2_0 => "HTTP/2.0".into(),
            _ => "HTTP/Uninitialized".into(),
        }
    }
}

/// Http rescourses
#[derive(Debug, PartialEq)]
pub enum Rescourse {
    Path(String),
}

#[derive(Debug)]
pub struct HttpRequest {
    pub mothod: Method,
    pub version: Version,
    pub resource: Rescourse,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

impl From<String> for HttpRequest {
    fn from(str: String) -> HttpRequest {
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_recourse = Rescourse::Path("".into());
        let mut parsed_version = Version::Uninitialized;
        let mut parsed_headers = HashMap::new();
        let mut parsed_body = None;

        println!("{:#?}", str);

        for line in str.lines() {
            match line {
                l if Regex::new(r"(GET|POST)\s/[0-9a-zA-z/.]*\sHTTP/(1\.1|1\.2|2\.0)")
                    .unwrap()
                    .is_match(l) =>
                {
                    let (method, recourse, version) = process_request_line(l);

                    parsed_method = method;
                    parsed_recourse = recourse;
                    parsed_version = version;
                }
                l if Regex::new(r"[a-zA-Z-]*:\s[\d\D]+").unwrap().is_match(l) => {
                    let dot_idx = l.find(':').unwrap();
                    let header_key = l[..dot_idx].trim().to_string();
                    let header_value = l[dot_idx + 1..].trim().to_string();

                    parsed_headers.entry(header_key).or_insert(header_value);
                }

                l => {
                    parsed_body = if l.len() > 0 {
                        Some(l.to_string())
                    } else {
                        None
                    };
                }
            }
        }

        HttpRequest {
            mothod: parsed_method,
            version: parsed_version,
            resource: parsed_recourse,
            headers: parsed_headers,
            body: parsed_body,
        }
    }
}

fn process_request_line(line: &str) -> (Method, Rescourse, Version) {
    let request_line: Vec<&str> = line.split(" ").collect();

    if request_line.len() != 3 {
        eprintln!("Process request line error: {:?}", request_line);
        process::exit(1);
    }

    (
        request_line[0].into(),
        Rescourse::Path(request_line[1].to_string()),
        request_line[2].into(),
    )
}

#[cfg(test)]
mod request_test {
    use super::*;

    #[test]
    fn test_method_into() {
        let method_get: Method = "GET".into();
        let method_post: Method = "POST".into();

        assert_eq!(method_get, Method::Get);
        assert_eq!(method_post, Method::Post);
    }

    #[test]
    fn test_version_into() {
        let version_1_0: Version = "HTTP/1.0".into();
        let version_1_1: Version = "HTTP/1.1".into();
        let version_2_0: Version = "HTTP/2.0".into();

        assert_eq!(version_1_0, Version::V1_0);
        assert_eq!(version_1_1, Version::V1_1);
        assert_eq!(version_2_0, Version::V2_0);
    }

    #[test]
    fn test_parse_request_line() {
        let (method, resource, version) = process_request_line("GET / HTTP/1.0");

        assert_eq!(
            (method, resource, version),
            (Method::Get, Rescourse::Path("/".into()), Version::V1_0)
        );

        let (method, resource, version) = process_request_line("POST /foo HTTP/1.1");

        assert_eq!(
            (method, resource, version),
            (Method::Post, Rescourse::Path("/foo".into()), Version::V1_1)
        );
    }

    #[test]
    fn test_parse_request() {
        let request: String = r#"GET / HTTP/1.1
        Accept-Encoding: gzip, deflate, br
        Accept-Language: zh-CN,zh;q=0.9
        Cache-Control: max-age=0"#
            .into();
        let parsed_request: HttpRequest = request.into();
        let mut test_headers = HashMap::new();

        test_headers.insert("Accept-Encoding".into(), "gzip, deflate, br".into());
        test_headers.insert("Accept-Language".into(), "zh-CN,zh;q=0.9".into());
        test_headers.insert("Cache-Control".into(), "max-age=0".into());

        println!("{:#?}", parsed_request);

        assert_eq!(parsed_request.mothod, Method::Get);
        assert_eq!(parsed_request.resource, Rescourse::Path("/".into()));
        assert_eq!(parsed_request.version, Version::V1_1);
        assert_eq!(parsed_request.headers, test_headers);
        assert_eq!(parsed_request.body, None);
    }
}
