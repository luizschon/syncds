mod backend;
mod pixel_format;

use backend::SlintBackend3DS;

pub fn init() {
    slint::platform::set_platform(Box::new(SlintBackend3DS::new()))
        .expect("Slint 3DS backend already initialized.");
}
