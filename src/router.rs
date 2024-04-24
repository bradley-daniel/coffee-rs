use std::collections::HashMap;

use crate::reponse::{IntoResponse, Response};

trait Handler {
    fn call(&self) -> Response;
}

impl<H> Handler for H
where
    H: Fn() -> Response,
{
    fn call(&self) -> Response {
        self()
    }
}

pub struct Router {
    routes: HashMap<&'static str, Box<dyn Handler>>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    pub fn route<F, R>(mut self, path: &'static str, handler: F) -> Self
    where
        F: Fn() -> R + 'static,
        R: IntoResponse,
    {
        self.routes
            .insert(path, Box::new(move || handler().into_resonse()));
        self
    }

    pub fn call(&self, endpoint: &str) -> Option<Response> {
        self.routes.get(endpoint).map(|handler| handler.call())
    }
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}
