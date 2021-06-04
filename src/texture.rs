use crate::{
    random_double,
    perlin::*,
    vec3::{Vec3, dot, random_in_unit_sphere, random_unit_vector, reflect, refract, unit_vector, Color},
};

pub trait Texture {
    fn value(&self, u:f64, v:f64, p:&Vec3) -> Color;
}

#[derive(Clone)]
pub enum Surface {
    SolidColor(SolidColor),
    TestTexture(TestTexture),
    NoiseTexture(NoiseTexture),
}

impl Surface {
    pub fn new_solid_color(c:Color) -> Self {Surface::SolidColor(SolidColor::new(c))}
    pub fn new_test_texture(c:Color) -> Self {Surface::TestTexture(TestTexture::new(c))}
    pub fn new_noise_texture(c:f64) -> Self {Surface::NoiseTexture(NoiseTexture::new(c))}
}

impl Texture for Surface {
    fn value(&self, u:f64, v:f64, p:&Vec3) -> Color {
        match self {
            Surface::SolidColor(r) => r.value(u,v,p),
            Surface::TestTexture(r) => r.value(u,v,p),
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
