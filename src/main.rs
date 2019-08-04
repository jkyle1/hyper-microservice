use futures::{future, Future};
use hyper::{Body, Error, Method, Request, Response, Server, StatusCode};
use hyper::service::service_fn;
use slab::Slab;
use std::sync::{Arc, Mutex};
use std::fmt;
extern crate futures;
extern crate hyper;


type UserId = u64;
struct UserData;

//Arc= atomic reference counter provides multiple references to single instance of data - mutex over slab of user data
type UserDb = Arc<Mutex<Slab<UserData>>>;


fn main() {

    let addr = ([127, 0, 0, 1], 8080).into();
    let builder = Server::bind(&addr);
    let user_db = Arc::new(Mutex::new(Slab::new()));
    let server = builder.serve(move || {
        let user_db = user_db.clone();
        service_fn(move |req| microservice_handler(req, &user_db))
    });
    let server = server.map_err(drop);
    hyper::rt::run(server);
}

    fn microservice_handler(req: Request<Body>, user_db: &UserDb) -> impl Future<Item=Response<Body>, Error=Error>
    {
            match(req.method(), req.uri().path()){
                (&Method::GET, "/") => {
                    future::ok(Response::new(INDEX.into()))
                },
                _ =>{
                let response = Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(Body::empty())
                    .unwrap();
                    future::ok(response)
                },
        }
    }

    fn response_with_code(status_code: StatusCode) -> Response<Body> {
        Response::builder()
            .status(status_code)
            .body(Body::empty())
            .unwrap()
    }
    
const INDEX: &'static str = r#"
<!doctype html>
<html>
    <head>
        <title>Rust Microservice</title>
    </head>
    <body>
        <h3>Rust Microservice</h3>
    </body>
</html>
"#;

