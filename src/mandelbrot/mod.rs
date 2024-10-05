pub extern crate image;
// pub extern crate num;

use std::time::Instant;

//use druid::image::{DynamicImage, ImageBuffer, RgbImage};
use num::Complex;

use rayon::prelude::*;

pub struct Pos {
    pub x: f64,
    pub y: f64,
}
#[derive(Debug, Clone)]
pub struct Coordinates {
    pub re_min: f64,
    pub re_max: f64,
    pub im_min: f64,
    pub im_max: f64,
}

impl Coordinates {
    pub fn calc_zoom_on_pos(&mut self, pos: Pos, canvas_size: Pos, zoom_in: bool) {
         
        let mut m_pos =  Pos {x: 0.0, y: 0.0 };
        m_pos.x = map(pos.x,  0.0, canvas_size.x, self.re_min, self.re_max);
        m_pos.y = map(pos.y,  0.0, canvas_size.y, self.im_min, self.im_max);
        let dx = self.re_max - self.re_min;
        let dy = self.im_max - self.im_min;

        let new: Coordinates;
        if zoom_in {
            new = Coordinates {
                re_min: m_pos.x - dx / 4.0,
                re_max: m_pos.x + dx / 4.0,
                im_min: m_pos.y - dy / 4.0,
                im_max: m_pos.y + dy / 4.0,

            };
            
        } else {
            new = Coordinates {
                re_min: m_pos.x - dx,
                re_max: m_pos.x + dx,
                im_min: m_pos.y - dy,
                im_max: m_pos.y + dy,

            };
        }

        *self = new;
    }  
}
impl Default for Coordinates {
    fn default() -> Self {
        Self {
            re_min: -2.0,
            re_max: 1.0,
            im_min: -1.0,
            im_max: 1.0,

        }
    }

    
}

#[derive(Debug, Clone)]
pub struct Mandelbrot {
    pub max_iterations: usize,
    pub image_size: (usize, usize),
    pub threshold: Option<f64>,
    pub color_map: image::DynamicImage,
}

impl Default for Mandelbrot {
    fn default() -> Self {
        let d = include_bytes!("../../assets/spectrum_visible_short.png");
        let b = image::load_from_memory(d).unwrap();

        println!("map size: {}/{}", b.width(), b.height());

        Self {
            max_iterations: 50,
            image_size: (0, 0),
            threshold: None,
            color_map: b.clone(),
        }
    }
}
impl Mandelbrot {
    pub fn calculate(&self, coordinates: &Coordinates) -> Vec<usize> {

        let start = Instant::now();
        let (width, height) = self.image_size;

        let mut buffer : Vec<_> = vec![0; width*height];
        for row in 0..height {
            for col in 0..width {

                let x_percent = col as f64 / width as f64;
                let y_percent = row as f64 / height as f64;

                let cx = coordinates.re_min + (coordinates.re_max - coordinates.re_min) * x_percent;
                let cy = coordinates.im_min + (coordinates.im_max - coordinates.im_min) * y_percent;

                let escaped_at = self.mandelbrot_at_point(cx, cy, 2.0);

                let index = row*width+col;

                buffer[index] = escaped_at;
            }
        }

        let ellapsed = start.elapsed();
        println!("calculate: {:?}", ellapsed.as_millis());
        buffer
    }

    pub fn calculate2(&self, coordinates: &Coordinates) -> Vec<usize> {

        let start = Instant::now();
        let (width, height) = self.image_size;

        let escape = match self.threshold {
            Some(value) => value,
            None => 2.0
        };

        println!("calc img size: {}/{}", width, height);

        let buffer = (0..width*height).into_par_iter().map(|index| {
            let res = (index / width, index % width);
            let row = res.0;
            let col = res.1;
            let x_percent = col as f64 / width as f64;
            let y_percent = row as f64 / height as f64;

            let cx = coordinates.re_min + (coordinates.re_max - coordinates.re_min) * x_percent;
            let cy = coordinates.im_min + (coordinates.im_max - coordinates.im_min) * y_percent;

            let escaped_at = self.mandelbrot_at_point(cx, cy, escape);

            escaped_at
        }).collect();
        

        let ellapsed = start.elapsed();
        println!("calculate2: {:?}", ellapsed.as_millis());
        buffer
    }

    fn mandelbrot_at_point(&self, cx: f64, cy: f64, escape: f64) -> usize {
    

        
    
        let mut z = Complex::<f64>{re: 0.0, im: 0.0};
        let c = Complex::new(cx, cy);
    
        for i in 0..=self.max_iterations {
            if z.norm() > escape {
                return i;
            }
            z = z * z + c;
        }
    
        0
    }

    pub fn render_mandelbrot_image(&self, buffer: &[usize]) -> image::RgbImage {

        let start = Instant::now();
        let (width, height) = (self.image_size.0 as u32, self.image_size.1 as u32);
        println!("render img size: {}/{}", width, height);
        let mut imgbuf = image::ImageBuffer::new(width, height);
    
        let map_buffer = self.color_map.as_rgba8().unwrap();

        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let index: usize = y as usize * width as usize + x as usize;
            let value = buffer[index];
            let n = self.normalize_value(value, 0, self.max_iterations);
            let x_pos = n*100.0 - 1.0;
            let map_pixel = map_buffer.get_pixel(x_pos as u32, 0);
            
            //let gray = n as u8;
            //*pixel = image::Rgb::<u8>([gray, gray, gray]);
            *pixel = image::Rgb::<u8>([map_pixel[0], map_pixel[1], map_pixel[2]]);
            
            // if value == 0 {
            //     *pixel = image::Rgb::<u8>([0, 0, 0]);
            // } else {
            //     *pixel = image::Rgb::<u8>([255, 255, 255]);
            // }
            
        }
        let img_data = image::RgbImage::from_vec(width, height, imgbuf.into_vec()).unwrap();

        let ellapsed = start.elapsed();
        println!("render image: {:?}", ellapsed.as_millis());
        img_data
    }
    fn normalize_value(&self, value: usize, min: usize, max: usize) -> f32 {
        let n = (value as f32 - min as f32) / (max as f32 - min as f32);
        n
    }
}

fn map(value: f64, in_min: f64, in_max: f64, out_min: f64, out_max: f64) -> f64 {
    (value - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}

