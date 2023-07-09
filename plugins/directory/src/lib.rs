use std::panic;
use starship_plugin::*;

mod tracing_subscriber;

#[fp_export_impl(starship_plugin)]
fn version() -> [u8; 3] {
    [0, 1, 0]
}

#[fp_export_impl(starship_plugin)]
fn metadata() -> PluginMetadata {
    PluginMetadata {
        name: "directory".to_string(),
        description: "The current working directory".to_string(),
    }
}

#[fp_export_impl(starship_plugin)]
async fn output() -> String {
    current_dir()
}

fn init_panic_hook() {
    use std::sync::Once;
    static SET_HOOK: Once = Once::new();
    SET_HOOK.call_once(|| {
        panic::set_hook(Box::new(|info| log(info.to_string())));
    });
}

#[fp_export_impl(starship_plugin)]
fn init() {
    init_panic_hook();
    // tracing_subscriber::init();
    // tracing::info!("directory plugin initialized");
}
