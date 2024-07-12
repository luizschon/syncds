use ctru::prelude::*;

slint::include_modules!();

fn main() {
    let apt = Apt::new().expect("Couldn't obtain APT controller");
    slint_adapter::init();
    let test = MainWindow::new().unwrap();

    while apt.main_loop() {
        test.run().unwrap();
    }
}
