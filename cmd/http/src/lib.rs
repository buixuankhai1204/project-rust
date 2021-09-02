#![recursion_limit = "1024"]
#[allow(unused_imports)]
#[allow(unused_macros)]

#[macro_use]
extern crate lazy_static;

#[cfg(feature = "with-serde")]
extern crate serde;
#[cfg(feature = "with-serde")]
#[macro_use]
extern crate serde_derive;
#[cfg(feature = "with-serde")]
extern crate serde_json;

#[macro_use]
extern crate log;

use std::net::IpAddr;

lazy_static! {
    static ref FDCMDAPP: CmdApp = futures::executor::block_on(init_app());
}

pub struct CmdApp {
    pub id: i32,
    pub trusted_proxies: Vec<IpAddr>
}

impl CmdApp {
    pub fn get() -> &'static CmdApp {
        &*FDCMDAPP
    }
}

pub fn init_app_global() -> &'static CmdApp {
    return CmdApp::get();
}

pub async fn init_app() -> CmdApp {
    let trusted_proxies = [127, 10, 0, 1].into();
    return CmdApp {
        id: 1001,
        trusted_proxies: vec![trusted_proxies],
    };
}

use std::convert::Infallible;
use warp::{reject, Rejection};
pub type Result<T> = std::result::Result<T, Rejection>;
pub type RejectionResult<T> = std::result::Result<T, Infallible>;

#[derive(Debug)]
pub struct ResultEmpty;
impl reject::Reject for ResultEmpty {}

pub mod model;
pub mod service;


#[macro_use]
pub mod macros;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result1<T, E = Error> = std::result::Result<T, E>;