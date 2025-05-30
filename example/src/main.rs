use trading_charts_example::example::App;
use leptos::{mount::mount_to_body, view};

fn main() {
    // set up logging
    #[cfg(debug_assertions)]
    console_log::init_with_level(log::Level::Debug).unwrap();
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! {
            <App/>
        }
    })
}
