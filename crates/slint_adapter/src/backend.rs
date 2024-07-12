use crate::pixel_format::Rgba8Pixel;
use ctru::prelude::*;
use ctru::services::{
    gfx::{Flush, Screen, Swap},
    gspgpu::FramebufferFormat,
};
use slint::{
    platform::{
        software_renderer::{MinimalSoftwareWindow, RenderingRotation, RepaintBufferType},
        Platform, WindowAdapter,
    },
    PhysicalSize,
};
use std::{cell::RefCell, rc::Rc};

// Top screen size in 2D mode
const TOP_DISPLAY_WIDTH: usize = 400;
const TOP_DISPLAY_HEIGHT: usize = 240;

// Bottom screen size
const BOTTOM_DISPLAY_WIDTH: usize = 320;
const _BOTTOM_DISPLAY_HEIGHT: usize = 240;

const DISPLAY_WIDTH: usize = TOP_DISPLAY_WIDTH + BOTTOM_DISPLAY_WIDTH;
const DISPLAY_HEIGHT: usize = TOP_DISPLAY_HEIGHT;

static mut FB1: [Rgba8Pixel; DISPLAY_WIDTH * DISPLAY_HEIGHT] = [Rgba8Pixel {
    red: 0,
    green: 0,
    blue: 0,
    alpha: 0,
}; DISPLAY_WIDTH * DISPLAY_HEIGHT];

pub(crate) struct SlintBackend3DS {
    window: RefCell<Option<Rc<MinimalSoftwareWindow>>>,
    start_time: std::time::Instant,
}

impl SlintBackend3DS {
    pub fn new() -> Self {
        Self {
            window: Default::default(),
            start_time: std::time::Instant::now(),
        }
    }
}

impl Platform for SlintBackend3DS {
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
                    renderer.set_rendering_rotation(RenderingRotation::Rotate90);
                    renderer.render(&mut buffer, DISPLAY_HEIGHT);

                    let framebuffer = top_screen.raw_framebuffer();
                    unsafe {
                        framebuffer.ptr.copy_from(
                            buffer.as_ptr().cast::<u8>(),
                            TOP_DISPLAY_WIDTH * TOP_DISPLAY_HEIGHT * 4,
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
