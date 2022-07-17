extern crate console_error_panic_hook;

fn main() {
    console_error_panic_hook::set_once();
    csh::start();
}
