#![allow(clippy::redundant_closure_call)]
#![allow(clippy::await_holding_refcell_ref)]
#![allow(clippy::drop_non_drop)]
#![allow(non_snake_case)]

pub mod app;
pub mod elements;
pub mod page;
pub mod util;

use cfg_if::cfg_if;
use util::ApiClient;

pub const ROOT_API_URL: &str = "http://127.0.0.1:8070/";

cfg_if! {
    if #[cfg(feature = "console_log")] {
        fn init_log() {
            use log::Level;
            console_log::init_with_level(Level::Trace).expect("error initializing log");
        }
    } else {
        fn init_log() {}
    }
}

fn main() {
    init_log();
    ApiClient::init();
    dioxus_web::launch(app::App)
}

pub mod prelude {
    pub use crate::elements::use_toaster;
    pub use crate::page;
    pub use crate::util::{async_handler, maybe_class, sync_handler, ApiClient};
    pub use dioxus_router::{use_route, use_router};
}
