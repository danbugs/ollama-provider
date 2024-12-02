use wasmcloud_component::http;

wit_bindgen::generate!({ generate_all });

use thomastaylor312::ollama::generate::{generate, Request};

struct Component;

http::export!(Component);

impl http::Server for Component {
    fn handle(
        _request: http::IncomingRequest,
    ) -> http::Result<http::Response<impl http::OutgoingBody>> {
        let prompt = "Once upon a time".to_string();
        let resp = generate(&Request {
            prompt,
            images: None,
        }).unwrap();

        Ok(http::Response::new(format!("{:#?}!\n", resp.response)))
    }
}
