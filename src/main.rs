mod components;
mod backend;

fn main() {
    #[cfg(feature = "server")]
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(backend::launch_server());
    #[cfg(not(feature = "server"))]
    dioxus::launch(components::App);
}