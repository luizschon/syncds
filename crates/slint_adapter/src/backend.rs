use ctru::prelude::*;
use ctru::services::{
    gfx::{Flush, Screen, Swap},
    gspgpu::FramebufferFormat,
};
use slint::{
    platform::{
        software_renderer::{MinimalSoftwareWindow, RepaintBufferType, TargetPixel},
        Platform, WindowAdapter,
    },
    PhysicalSize,
};
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, PartialEq, Clone, Copy)]
struct Rgba8Pixel(u32);

impl TargetPixel for Rgba8Pixel {
    fn blend(&mut self, _color: slint::platform::software_renderer::PremultipliedRgbaColor) {
        ()
    }

    fn from_rgb(red: u8, green: u8, blue: u8) -> Self {
        Self((red as u32) << 24 | (green as u32) << 16 | (blue as u32) << 8 | 0xFF)
    }

    fn background() -> Self {
        Self(u32::MAX)
    }
}

const DISPLAY_WIDTH: usize = 240;
const DISPLAY_HEIGHT: usize = 400;
static mut FB1: [Rgba8Pixel; DISPLAY_WIDTH * DISPLAY_HEIGHT] =
    [Rgba8Pixel(0); DISPLAY_WIDTH * DISPLAY_HEIGHT];

pub struct GraphicsBackend {
    window: RefCell<Option<Rc<MinimalSoftwareWindow>>>,
    start_time: std::time::Instant,
}

impl Platform for GraphicsBackend {
    fn create_window_adapter(&self) -> Result<Rc<dyn WindowAdapter>, slint::PlatformError> {
        let window = MinimalSoftwareWindow::new(RepaintBufferType::ReusedBuffer);
        self.window.replace(Some(window.clone()));
        Ok(window)
    }

    fn duration_since_start(&self) -> core::time::Duration {
        self.start_time.elapsed()
    }

    fn run_event_loop(&self) -> Result<(), slint::PlatformError> {
        let gfx = Gfx::new().expect("Couldn't obtain GFX Controller");
        let mut hid = Hid::new().expect("Couldn't obtain HID Controller");
        let mut top_screen = gfx.top_screen.borrow_mut();
        top_screen.set_double_buffering(false);
        top_screen.set_framebuffer_format(FramebufferFormat::Rgba8);
        top_screen.swap_buffers();
        let mut console = Console::new(gfx.bottom_screen.borrow_mut());

        let fb = unsafe { &mut *core::ptr::addr_of_mut!(FB1) };

        let mut buffer: &mut [Rgba8Pixel] = fb;

        self.window
            .borrow()
            .as_ref()
            .unwrap()
            .set_size(PhysicalSize::new(
                DISPLAY_WIDTH as u32,
                DISPLAY_HEIGHT as u32,
            ));

        loop {
            slint::platform::update_timers_and_animations();

            if let Some(window) = self.window.borrow().clone() {
                window.draw_if_needed(|renderer| {
                    gfx.wait_for_vblank();
                    renderer.render(&mut buffer, DISPLAY_WIDTH);

                    let framebuffer = top_screen.raw_framebuffer();
                    unsafe {
                        framebuffer.ptr.copy_from(
                            buffer.as_ptr().cast::<u8>(),
                            DISPLAY_WIDTH * DISPLAY_HEIGHT * 4,
                        )
                    }

                    top_screen.flush_buffers();
                });
            }

            hid.scan_input();

            if hid.keys_down().contains(KeyPad::START) {
                console.clear();
                unsafe {
                    println!(
                        "{:?}",
                        std::slice::from_raw_parts(
                            top_screen.raw_framebuffer().ptr,
                            DISPLAY_HEIGHT
                        )
                    );
                }
                console.flush_buffers();
            }
        }
    }
}

pub fn init() {
    slint::platform::set_platform(Box::new(GraphicsBackend {
        window: Default::default(),
        start_time: std::time::Instant::now(),
    }))
    .expect("Slint backend already initialized.");
}
