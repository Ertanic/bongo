use axum::{
    body::Body,
    http::StatusCode,
    http::{Request, Response},
};
use rune::runtime::SyncFunction;
use std::{
    collections::HashMap,
    sync::Arc,
    task::{Context, Poll},
};
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct RoutesService {
    routes: Arc<Mutex<HashMap<String, SyncFunction>>>,
}

impl RoutesService {
    pub fn new(routes: Arc<Mutex<HashMap<String, SyncFunction>>>) -> Self {
        Self { routes }
    }
}

impl tower::Service<Request<Body>> for RoutesService {
    type Response = Response<Body>;
    type Error = std::convert::Infallible;
    type Future =
        std::pin::Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let path = req.uri().path().to_owned();

        let routes = self.routes.clone();
        Box::pin(async move {
            let lock = routes.lock().await;
            let handler = match lock.get(&path) {
                Some(handler) => handler,
                None => {
                    return Ok(Response::builder()
                        .status(StatusCode::NOT_FOUND)
                        .body(Body::from("Route not found"))
                        .expect("Unable to build response"));
                }
            };

            handler
                .async_send_call::<()>(())
                .await
                .into_result()
                .expect("Unable to convert VmResult into Result");

            Ok(Response::builder()
                .status(StatusCode::OK)
                .body(Body::from(()))
                .unwrap())
        })
    }
}
