use ctru::prelude::*;
use slint_adapter::backend as backend_3ds;

slint::include_modules!();

fn main() {
    let apt = Apt::new().expect("Couldn't obtain APT controller");
    backend_3ds::init();
    let test = MainWindow::new().unwrap();

    while apt.main_loop() {
        test.run().unwrap();
    }
}
