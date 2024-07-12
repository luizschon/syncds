use slint::platform::software_renderer::{PremultipliedRgbaColor, TargetPixel};

/// Pixel format represented in 32-bit, 8-bits per color channel plus 8-bit alpha
/// channel.
///
/// Note that the framebuffer data is interpreted as little-endian bytewise, meaning
/// that the byte ordering is A-B-G-R.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Rgba8Pixel {
    pub alpha: u8,
    pub blue: u8,
    pub green: u8,
    pub red: u8,
}

impl TargetPixel for Rgba8Pixel {
    fn blend(&mut self, color: PremultipliedRgbaColor) {
        let a = (255 - color.alpha) as u16;
        self.red = (self.red as u16 * a / 255) as u8 + color.red;
        self.green = (self.green as u16 * a / 255) as u8 + color.green;
        self.blue = (self.blue as u16 * a / 255) as u8 + color.blue;
        self.alpha = (self.alpha as u16 + color.alpha as u16
            - (self.alpha as u16 * color.alpha as u16) / 255) as u8;
    }

    fn from_rgb(red: u8, green: u8, blue: u8) -> Self {
        Self {
            red,
            green,
            blue,
            alpha: 255,
        }
    }

    fn background() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
            alpha: 0,
        }
    }
}

/// Pixel format represented in 24-bits, 8-bits per color channel in reverse order
/// (blue, green and red).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Bgr8Pixel {
    pub blue: u8,
    pub green: u8,
    pub red: u8,
}

impl TargetPixel for Bgr8Pixel {
    fn blend(&mut self, color: PremultipliedRgbaColor) {
        let a = (255 - color.alpha) as u16;
        self.red = (self.red as u16 * a / 255) as u8 + color.red;
        self.green = (self.green as u16 * a / 255) as u8 + color.green;
        self.blue = (self.blue as u16 * a / 255) as u8 + color.blue;
    }

    fn from_rgb(red: u8, green: u8, blue: u8) -> Self {
        Self { blue, green, red }
    }
}

/// Pixel format represented in 16-bits, 5-bits per color channel plus 1-bit
/// for the alpha channel.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Rgb5A1Pixel(pub u16);

impl Rgb5A1Pixel {
    const RED_MASK: u16 = 0b1111100000000000;
    const GREEN_MASK: u16 = 0b0000011111000000;
    const BLUE_MASK: u16 = 0b0000000000111110;

    /// Return red component as avalue between 0 and 255.
    fn red(&self) -> u8 {
        (self.0 & Self::RED_MASK >> 8) as u8
    }

    /// Return blue component as a value between 0 and 255.
    fn blue(&self) -> u8 {
        (self.0 & Self::BLUE_MASK >> 8) as u8
    }

    /// Return green component as a value between 0 and 255.
    fn green(&self) -> u8 {
        (self.0 & Self::GREEN_MASK << 2) as u8
    }

    /// Return alpha component as a value between 0 and 255.
    ///
    /// Since the alpha channel id represented as 1-bit, the only two possible
    /// values are 0 and 255.
    fn alpha(&self) -> u8 {
        (self.0 & 0b1 << 7) as u8
    }
}

impl TargetPixel for Rgb5A1Pixel {
    fn blend(&mut self, color: PremultipliedRgbaColor) {
        unimplemented!()
    }

    fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        // Grab most significant bits from each channel, pack them inside a u16
        // and set alpha channel bit to 1.
        Self(r as u16 & 0b11111000 << 8 | g as u16 & 0b11111000 << 3 | b as u16 >> 2 | 0b1)
    }

    fn background() -> Self {
        Self(0)
    }
}

/// Pixel format represented in 16-bits, 4-bits per color channel and alpha
/// channel.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Rgba4Pixel(pub u16);

impl TargetPixel for Rgba4Pixel {
    fn blend(&mut self, color: PremultipliedRgbaColor) {
        unimplemented!()
    }

    fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        unimplemented!()
    }

    fn background() -> Self {
        unimplemented!()
    }
}
