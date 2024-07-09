use ctru::prelude::*;
use ctru::services::gfx::Flush;
use ctru::services::{
    gfx::{Flush, Screen, Swap},
    gspgpu::FramebufferFormat,
};
use slint::platform::software_renderer::{self, Rgb565Pixel, SoftwareRenderer, TargetPixel};
use slint::{
    platform::{
        software_renderer::{MinimalSoftwareWindow, RepaintBufferType},
        Platform, WindowAdapter,
    },
    PhysicalSize,
};
use std::intrinsics::size_of;
use std::{cell::RefCell, rc::Rc, time::Instant};

const DISPLAY_WIDTH: usize = 320;
const DISPLAY_HEIGHT: usize = 240;
static mut FB1: [TargetPixel; DISPLAY_WIDTH * DISPLAY_HEIGHT] =
    [software_renderer::Rgb565Pixel(0); DISPLAY_WIDTH * DISPLAY_HEIGHT];

pub struct GraphicsBackend {
    window: RefCell<Option<Rc<MinimalSoftwareWindow>>>,
    start_time: Instant,
}

impl Platform for GraphicsBackend {
    fn create_window_adapter(&self) -> Result<Rc<dyn WindowAdapter>, slint::PlatformError> {
        let window = MinimalSoftwareWindow::new(RepaintBufferType::SwappedBuffers);
        self.window.replace(Some(window.clone()));
        Ok(window)
    }

    fn duration_since_start(&self) -> core::time::Duration {
        core::time::Duration::from_micros(self.start_time.elapsed().as_micros() as u64)
    }

    fn run_event_loop(&self) -> Result<(), slint::PlatformError> {
        let gfx = Gfx::new().expect("Couldn't obtain GFX Controller");
        let mut hid = Hid::new().expect("Couldn't obtain HID Controller");
        let mut top_screen = gfx.top_screen.borrow_mut();
        top_screen.set_double_buffering(false);
        top_screen.set_framebuffer_format(FramebufferFormat::Rgb565);
        top_screen.swap_buffers();
        let (width, height) = (framebuffer.width, framebuffer.height);

        let _console = Console::new(gfx.bottom_screen.borrow_mut());
        println!("\x1b[21;4HPress A to flip the image.");
        println!("\x1b[29;16HPress Start to exit");

        let fb = unsafe { &mut *core::ptr::addr_of_mut!(FB1) };

        let mut buffer: &mut [TargetPixel] = fb;

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
                    renderer.render(buffer, DISPLAY_WIDTH);

                    let framebuffer = top_screen.raw_framebuffer();
                    unsafe {
                        framebuffer.ptr.copy_from(
                            buffer.as_ptr(),
                            DISPLAY_WIDTH
                                * DISPLAY_HEIGHT
                                * size_of::<software_renderer::Rgb565Pixel>(),
                        )
                    }

                    top_screen.flush_buffers();
                });
            }

            hid.scan_input();

            if hid.keys_down().contains(KeyPad::START) {
                println!("\x1b[21;8HQuero sairrr");
            }
        }
    }
}