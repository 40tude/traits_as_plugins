pub mod rtd_512;

// Called from temperature::register()
pub fn register() {
    rtd_512::register();
}
