use ctru::prelude::*;

slint::include_modules!();

fn main() {
    let apt = Apt::new().expect("Couldn't obtain APT controller");
    let test = Test::new().unwrap();
    let mut quit = false;

    test.on_quit(move || {
        slint::quit_event_loop().unwrap();
        quit = true;
    });

    while apt.main_loop() {
        test.run().unwrap();
        if quit {
            break;
        }
    }
}
