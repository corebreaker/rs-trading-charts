use npm_rs::{NodeEnv, NpmEnv};
use log::info;

fn main() {
    info!("Building JS module for trading-charts...");
    let res = NpmEnv::default()
        .set_path("./bindings")
        .with_node_env(&NodeEnv::Development)
        .init_env()
        .install(None)
        .run("build")
        .exec()
        .unwrap();

    info!("JS module for buils result: {res}.");
}
