pub fn init_logger() {
    std::env::set_var("RUST_LOG", "actix_web=debug, actix_server=debug");
    env_logger::init();
}