#![recursion_limit = "1024"]

use std::sync::Arc;
use opentelemetry::{trace::{Tracer, FutureExt, TraceContextExt}, Context};

use log::warn;
use warp::Filter;
use router::*;
mod router;

#[tokio::main]
async fn main() -> project_rust::Result1<()> {
    let _cmd_app = project_rust::init_app_global();

    let user_v1_routes = router::user::user::setup();

    let routes = user_v1_routes
        .with(warp::cors().allow_any_origin());

    warp::serve(routes).run(([127, 0, 0, 1],19000))
        .await;

    Ok(())
}

