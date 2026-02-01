use std::future::Future;
use std::sync::OnceLock;
use tokio::runtime::Runtime;

static RUNTIME: OnceLock<Runtime> = OnceLock::new();

#[doc(hidden)]
pub fn init() {
    RUNTIME.get_or_init(|| Runtime::new().expect("Failed to create Tokio runtime"));
}

pub fn spawn<F>(future: F)
where
    F: Future<Output = ()> + Send + 'static,
{
    if let Some(rt) = RUNTIME.get() {
        rt.spawn(future);
    } else {
        eprintln!("Tokio runtime not initialized!");
    }
}
