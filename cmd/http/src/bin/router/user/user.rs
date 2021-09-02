use warp::{self, Filter, filters::BoxedFilter};
use project_rust::service::user::user::login as login_service;
use project_rust::{wraped_or_tree,wraped_debug_boxed};
use project_rust::model::user::LoginRequest;

fn login() -> BoxedFilter<(impl warp::Reply,)> {
    warp::path!("user" / "login")
        .and(warp::post())
        .and(LoginRequest::parameters())
        .and_then(login_service)
        .boxed()
}


pub fn setup() -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
    let user_login_router = login();


    wraped_or_tree!(
        user_login_router
      )
}