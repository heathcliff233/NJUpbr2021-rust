use crate::{
    random_double,
    perlin::*,
    vec3::{Vec3, dot, random_in_unit_sphere, random_unit_vector, reflect, refract, unit_vector, Color},
    utils::clamp,
};

use image::*;
use std::path::*;

pub trait Texture {
    fn value(&self, u:f64, v:f64, p:&Vec3) -> Color;
}

#[derive(Clone)]
pub enum Surface {
    SolidColor(SolidColor),
    TestTexture(TestTexture),
    ImageTexture(ImageTexture),
    NoiseTexture(NoiseTexture),
}

impl Surface {
    pub fn new_solid_color(c:Color) -> Self {Surface::SolidColor(SolidColor::new(c))}
    pub fn new_test_texture(c:Color) -> Self {Surface::TestTexture(TestTexture::new(c))}
    pub fn new_image_texture(c:&str) -> Self {Surface::ImageTexture(ImageTexture::new_by_pathstr(c))}
    pub fn new_noise_texture(c:f64) -> Self {Surface::NoiseTexture(NoiseTexture::new(c))}
}

impl Texture for Surface {
    fn value(&self, u:f64, v:f64, p:&Vec3) -> Color {
        match self {
            Surface::SolidColor(r) => r.value(u,v,p),
            Surface::TestTexture(r) => r.value(u,v,p),
            Surface::ImageTexture(r) => r.value(u,v,p),
            Surface::NoiseTexture(r) => r.value(u,v,p),
        }
    }
}

#[derive(Clone)]
pub struct SolidColor{
    pub color_value: Color,
}

impl SolidColor{
    pub fn new1()->Self{
        Self{
            color_value:Color::ones(),
        }
    }
    pub fn new(c:Color)->Self{
        Self{
            color_value:c,
        }
    }
    pub fn new2(red:f64,green:f64,blue:f64)->Self{
        Self{
            color_value:Color::new(red,green,blue),
        }
    }
}
impl Texture for SolidColor {
    fn value(&self,u:f64, v:f64,p:&Vec3) -> Vec3{
        self.color_value
    }
}

#[derive(Clone)]
pub struct TestTexture {
    pub color1: Color,
    pub color2: Color,
}

impl TestTexture {
    pub fn new1()->Self{
        Self{
            color1:Color::ones(),
            color2:Color::ones()/2.0,
        }
    }
    pub fn new(c:Color)->Self{
        Self{
            color1:c,
            color2:c/2.0,
        }
    }
}

impl Texture for TestTexture {
    fn value(&self, u:f64, v:f64, p:&Vec3) -> Vec3 {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0 {
            return self.color1;
        } else {
            return self.color2;
        }
    }
}

#[derive(Clone)]

pub struct ImageTexture {
    pub data: ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>>,
}
impl ImageTexture {
    pub fn new_by_pathstr(dir: &str) -> Self {
        return Self {
            data: image::open(&Path::new(dir)).unwrap().to_rgb(),
        };
    }
    pub fn width(&self) -> u32 {
        return self.data.width();
    }
    pub fn height(&self) -> u32 {
        return self.data.height();
    }
}
impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: &Vec3) -> Vec3 {
        let u = clamp(u, 0.0, 1.0);
        let v = 1.0 - clamp(v, 0.0, 1.0);
        let mut i: u32 = (u * self.width() as f64) as u32;
        let mut j: u32 = (v * self.height() as f64) as u32;

        // Clamp integer mapping, since actual coordinates should be less than 1.0
        if i >= self.width() {
            i = self.width() - 1;
        }
        if j >= self.height() {
            j = self.height() - 1;
        }

        const COLOR_SCALE: f64 = 1.0 / 255.0;
        let pixel = self.data.get_pixel(i, j);
        let [red, green, blue] = pixel.0;
        return Vec3::new(
            red as f64 * COLOR_SCALE,
            green as f64 * COLOR_SCALE,
            blue as f64 * COLOR_SCALE,
        );
    }
}

#[derive(Clone)]
pub struct NoiseTexture {
    pub noise: Perlin,
    pub scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        return Self {
            noise: Perlin::new(),
            scale: scale,
        };
    }

    fn turb(&self, p: &Vec3, depth: u32) -> f64 {
        let mut accum = 0.0;
        let mut p = (*p).clone();
        let mut weight = 1.0;
        for _i in 0..depth {
            accum += self.noise.noise(&p) * weight;
            weight *= 0.5;
            p = p * 2.0;
        }
        accum.abs()
    }
}

impl Texture for NoiseTexture {
    fn value(&self, u:f64, v:f64, p:&Vec3) -> Vec3 {
        //return Vec3::ones() * self.noise.noise(p);
        Vec3::new(1.0, 1.0, 1.0) * 0.5 * (1.0 + (self.scale * p.z + 10.0 * self.turb(p, 7)).sin())
    }
}
