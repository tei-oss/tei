use std::future::Future;

use tokio::runtime;

pub fn execute_blocking<F: Future>(f: F) {
    runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(f);
}
