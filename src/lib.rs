extern crate hyper;
extern crate gotham;
extern crate mime;

use hyper::server::Http;
use hyper::{Request, Response, StatusCode};

use gotham::http::response::create_response;
use gotham::state::State;
use gotham::handler::NewHandlerService;


pub fn say_hello(state: State, _req: Request) -> (State, Response) {
    let res = create_response(
        &state,
        StatusCode::Ok,
        Some((
            String::from("Hello World!").into_bytes(),
            mime::TEXT_PLAIN,
        )),
    );

    (state, res)
}

pub fn start() {
    let addr = "0.0.0.0:8000".parse().unwrap();

    let server = Http::new()
        .bind(&addr, NewHandlerService::new(|| Ok(say_hello)))
        .unwrap();

    println!("Listening on http://{}", server.local_addr().unwrap());
    server.run().unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
