use crate::{
    hittable::HitRecord,
    random_double,
    ray::Ray,
    texture::Surface,
    vec3::{dot, random_in_unit_sphere, random_unit_vector, reflect, refract, unit_vector, Color, Vec3},
    pdf::PDF,
};
use crate::texture::Texture;
use crate::pdf::{CosinePDF,ZeroPDF};
use std::f64::consts::PI;
use itertools::Diff;

pub trait Scatter {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        srec: &mut ScatterRecord,
    ) -> bool;

    fn scatter_pdf(
        &self,
        ray: &Ray,
        rec: &HitRecord,
        scattered: &Ray
    ) -> f64;

    fn emit(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        u: f64,
        v: f64,
        p: Color,
    ) -> Color;
}

#[derive(Clone)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
    Diffuse(Diffuse),
}

impl Material {
    pub fn new_lambertian(albedo: Color) -> Self {
        Material::Lambertian(Lambertian::new(albedo))
    }

    pub fn new_noise_lamb(c: f64) -> Self {
        Material::Lambertian(Lambertian::new1(c))
    }

    pub fn new_image_tex(c: &str) -> Self { Material::Lambertian(Lambertian::new_img(c))}

    pub fn new_metal(albedo: Color, fuzz: f64) -> Self {
        Material::Metal(Metal::new(albedo, fuzz))
    }

    pub fn new_dielectric(ref_idx: f64) -> Self {
        Material::Dielectric(Dielectric::new(ref_idx))
    }

    pub fn new_diffuse(c: Vec3) -> Self {
        Material::Diffuse(Diffuse::new1(c))
    }
}

impl Scatter for Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        srec: &mut ScatterRecord,
    ) -> bool {
        match self {
            Material::Lambertian(m) => m.scatter(r_in, rec, srec),
            Material::Metal(m) => m.scatter(r_in, rec, srec),
            Material::Dielectric(m) => m.scatter(r_in, rec, srec),
            Material::Diffuse(m) => m.scatter(r_in, rec, srec),
        }
    }

    fn scatter_pdf(
        &self,
        ray: &Ray,
        rec: &HitRecord,
        scattered: &Ray
    ) -> f64 {
        match self {
            Material::Lambertian(m) => m.scatter_pdf(ray, rec, scattered),
            Material::Metal(m) => m.scatter_pdf(ray, rec, scattered),
            Material::Dielectric(m) => m.scatter_pdf(ray, rec, scattered),
            Material::Diffuse(m) => m.scatter_pdf(ray, rec, scattered),
        }
    }

    fn emit(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        u: f64,
        v: f64,
        p: Color,
    ) -> Color {
        match self {
            Material::Diffuse(m) => m.emit(ray, hit_record, u, v, p),
            _ => Color::zero(),
        }
    }

}

#[derive(Clone)]
pub struct Lambertian {
    albedo: Surface,
}

impl Lambertian {
    fn new(albedo: Color) -> Self {
        Self { albedo: Surface::new_solid_color(albedo) }
    }
    fn new1(c:f64) -> Self { Self { albedo: Surface::new_noise_texture(c)} }
    fn new_img(c:&str) -> Self { Self { albedo: Surface::new_image_texture(c)}}
}

impl Scatter for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        srec: &mut ScatterRecord
    ) -> bool {
        srec.material_type = MaterialType::Diffuse;
        srec.specular_ray = Ray::init();
        srec.attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
        srec.pdf_ptr = CosinePDF::new(rec.normal);
        true
    }

    fn scatter_pdf(&self, ray: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        let cosine = dot(&rec.normal, &unit_vector(scattered.direction));
        if cosine < 0.0 { 0.0 } else { cosine / PI }
    }

    fn emit(&self, ray: &Ray, rec: &HitRecord, u:f64, v:f64, p:Vec3) -> Color {
        Color::zero()
    }

}


#[derive(Clone)]
pub struct Metal {
    albedo: Surface,
    fuzz: f64,
}

impl Metal {
    fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo: Surface::new_test_texture(albedo),
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Scatter for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        srec: &mut ScatterRecord,
    ) -> bool {
        let reflected = reflect(unit_vector(r_in.direction), rec.normal);

        srec.specular_ray = Ray::new(rec.p, reflected + self.fuzz * random_in_unit_sphere(), 0.0);
        srec.attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
        srec.material_type = MaterialType::Specular;
        srec.pdf_ptr = ZeroPDF::new();
        dot(&srec.specular_ray.direction, &rec.normal) > 0.0
    }

    fn scatter_pdf(&self, ray: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        0.0
    }

    fn emit(&self, ray: &Ray, rec: &HitRecord, u:f64, v:f64, p:Vec3) -> Color {
        Color::zero()
    }

}

#[derive(Clone)]
pub struct Diffuse {
    shine: Surface,
}
impl Diffuse {
    fn new(shine: Surface) -> Self {
        Self { shine }
    }
    fn new1(col: Color) -> Self {
        Self {
            shine: Surface::new_solid_color(col),
        }
    }
}

impl Scatter for Diffuse {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        srec: &mut ScatterRecord,
    ) -> bool { false }

    fn scatter_pdf(
        &self,
        ray: &Ray,
        rec: &HitRecord,
        scattered: &Ray
    ) -> f64 { 0.0 }

    fn emit(&self, ray: &Ray, rec: &HitRecord, u:f64, v:f64, p:Vec3) -> Color {
        if rec.front_face {
            self.shine.value(u, v, &p)
        } else {
            Color::zero()
        }
    }
}

#[derive(Clone)]
pub struct Dielectric {
    ref_idx: f64,
}

impl Dielectric {
    fn new(ref_idx: f64) -> Self {
        Self { ref_idx }
    }
}

impl Scatter for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        srec: &mut ScatterRecord
    ) -> bool {
        srec.attenuation = Color::ones();
        let etai_over_etat = if rec.front_face {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };
        let unit_direction = unit_vector(r_in.direction);
        let cos_theta = f64::min(dot(&-unit_direction, &rec.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta.powi(2));
        srec.specular_ray = if etai_over_etat * sin_theta > 1.0
            || random_double!() < schlick(cos_theta, etai_over_etat)
        {
            let reflected = reflect(unit_direction, rec.normal);
            Ray::new(rec.p, reflected, 0.0)
        } else {
            let refracted = refract(unit_direction, rec.normal, etai_over_etat);
            Ray::new(rec.p, refracted, 0.0)
        };
        srec.material_type = MaterialType::Specular;
        srec.pdf_ptr = ZeroPDF::new();
        true
    }

    fn scatter_pdf(&self, ray: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        0.0
    }

    fn emit(&self, ray: &Ray, rec: &HitRecord, u:f64, v:f64, p:Vec3) -> Color {
        Color::zero()
    }
}


pub enum MaterialType {
    Diffuse,
    Specular,
}

pub struct ScatterRecord<'a> {
    pub material_type: MaterialType,
    pub specular_ray: Ray,
    pub attenuation: Color,
    pub pdf_ptr: PDF<'a>,
}

fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
