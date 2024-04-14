// Original code from rust-osdev/bootloader crate https://github.com/rust-osdev/bootloader

use bootloader_api::info::{FrameBuffer, FrameBufferInfo, PixelFormat};
use core::{fmt, ptr};
use kernel::RacyCell;
use noto_sans_mono_bitmap::RasterHeight::Size16;
use noto_sans_mono_bitmap::{get_raster, FontWeight, RasterizedChar};

static WRITER: RacyCell<Option<ScreenWriter>> = RacyCell::new(None);

pub fn screenwriter() -> &'static mut ScreenWriter {
    let writer = unsafe { WRITER.get_mut() }.as_mut().unwrap();
    writer
}

pub fn init(buffer: &'static mut FrameBuffer) {
    let info = buffer.info();
    let framebuffer = buffer.buffer_mut();
    let writer = ScreenWriter::new(framebuffer, info);
    *unsafe { WRITER.get_mut() } = Some(writer);
}

/// Additional vertical space between lines
const LINE_SPACING: usize = 0;

pub struct ScreenWriter {
    framebuffer: &'static mut [u8],
    info: FrameBufferInfo,
    x_pos: usize,
    y_pos: usize,
}

impl ScreenWriter {
    pub fn new(framebuffer: &'static mut [u8], info: FrameBufferInfo) -> Self {
        let mut logger = Self {
            framebuffer,
            info,
            x_pos: 0,
            y_pos: 0,
        };
        logger.clear();
        logger
    }

    fn newline(&mut self) {
        self.y_pos += Size16 as usize + LINE_SPACING;
        self.carriage_return()
    }

    fn carriage_return(&mut self) {
        self.x_pos = 0;
    }

    pub fn set_cursor(&mut self, x: usize, y: usize) {
        self.x_pos = x;
        self.y_pos = y;
    }

    /// Erases all text on the screen.
    pub fn clear(&mut self) {
        self.x_pos = 0;
        self.y_pos = 0;
        self.framebuffer.fill(0);
    }

    fn width(&self) -> usize {
        self.info.width
    }

    fn height(&self) -> usize {
        self.info.height
    }

    fn number_to_chars(&self, n: &u32) -> [char; 3] {
        let mut chars = ['\0'; 3];
        let mut n = *n;
        for i in (0..3).rev() {
            chars[i] = ((n % 10) as u8 + b'0') as char;
            n /= 10;
        }
        chars
    }

    pub fn write_str(&mut self, text: &str) {
        for c in text.chars() {
            self.write_char(c);
        }
    }

    pub fn write_number(&mut self, number: &u32) {
        let string_number = self.number_to_chars(number);
        for c in string_number.iter() {
            if *c != '\0' {
                self.write_char(*c);
            }
        }
    }

    pub fn write_char(&mut self, c: char) {
        match c {
            '\n' => self.newline(),
            '\r' => self.carriage_return(),
            c => {
                if let Some(bitmap_char) = get_raster(c, FontWeight::Regular, Size16) {
                    if self.x_pos + bitmap_char.width() > self.width() {
                        self.newline();
                    }
                    if self.y_pos + bitmap_char.height() > self.height() {
                        self.clear();
                    }
                    self.write_rendered_char(bitmap_char);
                }
            }
        }
    }

    fn write_rendered_char(&mut self, rendered_char: RasterizedChar) {
        for (y, row) in rendered_char.raster().iter().enumerate() {
            for (x, byte) in row.iter().enumerate() {
                self.write_pixel(self.x_pos + x, self.y_pos + y, *byte);
            }
        }
        self.x_pos += rendered_char.width();
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, intensity: u8) {
        let pixel_offset = y * self.info.stride + x;
        let color = match self.info.pixel_format {
            PixelFormat::Rgb => [intensity / 4, intensity, intensity / 2, 0],
            PixelFormat::Bgr => [intensity / 2, intensity, intensity / 4, 0],
            other => {
                // set a supported (but invalid) pixel format before panicking to avoid a double
                // panic; it might not be readable though
                self.info.pixel_format = PixelFormat::Rgb;
                panic!("pixel format {:?} not supported in logger", other)
            }
        };
        let bytes_per_pixel = self.info.bytes_per_pixel;
        let byte_offset = pixel_offset * bytes_per_pixel;
        self.framebuffer[byte_offset..(byte_offset + bytes_per_pixel)]
            .copy_from_slice(&color[..bytes_per_pixel]);
        let _ = unsafe { ptr::read_volatile(&self.framebuffer[byte_offset]) };
    }

    pub fn draw_pixel(&mut self, x: usize, y: usize, r: u8, g: u8, b: u8) {
        let pixel_offset = y * self.info.stride + x;
        let color = match self.info.pixel_format {
            PixelFormat::Rgb => [r, g, b, 0],
            PixelFormat::Bgr => [b, g, r, 0],
            other => {
                // set a supported (but invalid) pixel format before panicking to avoid a double
                // panic; it might not be readable though
                self.info.pixel_format = PixelFormat::Rgb;
                panic!("pixel format {:?} not supported in logger", other)
            }
        };
        let bytes_per_pixel = self.info.bytes_per_pixel;
        let byte_offset = pixel_offset * bytes_per_pixel;
        self.framebuffer[byte_offset..(byte_offset + bytes_per_pixel)]
            .copy_from_slice(&color[..bytes_per_pixel]);
        let _ = unsafe { ptr::read_volatile(&self.framebuffer[byte_offset]) };
    }
}

unsafe impl Send for ScreenWriter {}
unsafe impl Sync for ScreenWriter {}

impl fmt::Write for ScreenWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.write_char(c);
        }
        Ok(())
    }
}
