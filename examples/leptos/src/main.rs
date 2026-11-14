use leptos::mount::mount_to_body;
use trading_charts_example_leptos::app::App;

fn main() {
    #[cfg(debug_assertions)]
    console_log::init_with_level(log::Level::Debug).unwrap();
    console_error_panic_hook::set_once();

    mount_to_body(App)
}
