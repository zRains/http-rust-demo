use crate::handler::{Handler, PageNotFoundHandler, StaticPageHandler, WebServiceHandler};
use http::{
    request::{HttpRequest, Method, Rescourse},
    response::HttpResponse,
};
use std::io::Write;

pub struct Router;

impl Router {
    pub fn route(request: HttpRequest, stream: &mut impl Write) {
        match request.mothod {
            Method::Get => match &request.resource {
                Rescourse::Path(s) => {
                    let paths: Vec<&str> = s.split("/").collect();

                    println!("{:?}", paths);

                    match paths[1] {
                        "api" => {
                            let response: HttpResponse = WebServiceHandler::handle(&request);
                            response.send(stream).unwrap();
                        }
                        _ => {
                            let response: HttpResponse = StaticPageHandler::handle(&request);

                            response.send(stream).unwrap();
                        }
                    }
                }
            },
            _ => {
                let response: HttpResponse = PageNotFoundHandler::handle(&request);
                response.send(stream).unwrap();
            }
        }
    }
}
