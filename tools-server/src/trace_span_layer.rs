use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use axum::http::{HeaderMap, Request};
use tower::{Layer, Service};
use tracing::Level;

#[derive(Default, Debug, Clone)]
pub struct TraceSpanLayer;

impl<S> Layer<S> for TraceSpanLayer {
    type Service = TraceSpanMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        TraceSpanMiddleware { inner }
    }
}

#[derive(Clone)]
pub struct TraceSpanMiddleware<S> {
    inner: S,
}

impl<S, ReqBody> Service<Request<ReqBody>> for TraceSpanMiddleware<S>
where
    S: Service<Request<ReqBody>> + Send + 'static,
    S::Future: Send + 'static,
    S::Response: Send + 'static,
    S::Error: Send + 'static,
    ReqBody: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        let trace_id = extract_trace_id(req.headers());
        let fut = self.inner.call(req);

        Box::pin(async move {
            let span = tracing::span!(Level::INFO, "TRACE_ID", trace_id = %trace_id);
            let _enter = span.enter();
            let res = fut.await?;
            Ok(res)
        })
    }
}

fn extract_trace_id(headers: &HeaderMap) -> String {
    headers
        .get("trace_id")
        .or_else(|| headers.get("correlation_id"))
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "NONE".to_string())
}