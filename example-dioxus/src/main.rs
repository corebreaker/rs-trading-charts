use trading_charts_dioxus_example::example::app;

fn main() {
    #[cfg(debug_assertions)]
    console_log::init_with_level(log::Level::Debug).unwrap();
    console_error_panic_hook::set_once();

    dioxus::launch(app);
}
