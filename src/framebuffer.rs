use crate::Vector;
use num_traits::real::Real;
extern crate image;

type Pixel = Vector<u8, 3>;

pub struct FrameBuffer {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Vec<Pixel>>,
}

#[allow(dead_code)]
impl FrameBuffer {
    pub fn new(width: usize, height: usize) -> FrameBuffer {
        let mut pixels = Vec::new();
        for _ in 0..height {
            let mut row = Vec::new();
            for _ in 0..width {
                row.push(Vector{ v : [0, 0, 0] });
            }
            pixels.push(row);
        }

        FrameBuffer {
            width,
            height,
            pixels,
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, pixel: Pixel) {
        self.pixels[y][x] = pixel;
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Pixel {
        self.pixels[y][x]
    }

    pub fn clear(&mut self) {
        for row in self.pixels.iter_mut() {
            for pixel in row.iter_mut() {
                *pixel = Vector{ v : [0, 0, 0] };
            }
        }
    }

    pub fn write_image(&self, filename: &str) {
        let mut imgbuf = image::ImageBuffer::new(self.width as u32, self.height as u32);
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let p = self.get_pixel(x as usize, y as usize);
            *pixel = image::Rgb([p.v[0], p.v[1], p.v[2]]);
        }
        imgbuf.save(filename).unwrap();
    }
}


pub fn lerp<T: Real>(a: Pixel, b: Pixel, t: T) -> Pixel where u8: From<T>{
    let mut ret: Pixel = Default::default();
    ret = a + (b - a) * t;
    return ret;
}