use std::fs::File;
use std::io::prelude::*;
use std::ops;

#[derive(Debug, Copy, Clone)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32
}

impl Vec3 {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }

    fn mag(&self) -> f32 {
        (*self * *self).sqrt()
    }

    fn normalized(&self) -> Self {
        let mag = self.mag();
        return *self * (1. / mag);
    }

    fn reflect_on(&self, n: &Vec3) -> Self {
        *n * 2. * (*self * *n)  - *self
    }

    fn origin() -> Self {
        Vec3::new(0., 0., 0.)
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = f32;
    fn mul(self, v: Vec3) -> f32 {
        self.x * v.x + self.y * v.y + self.z * v.z // Dot product
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, f: f32) -> Self {
        Vec3 { x: f * self.x, y: f * self.y, z: f * self.z }
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, v: Vec3) -> Self {
        Vec3 { x: self.x + v.x, y: self.y + v.y, z: self.z + v.z }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, v: Vec3) -> Self {
        Vec3 { x: self.x - v.x, y: self.y - v.y, z: self.z - v.z }
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Vec3 { x: self.x * -1., y: self.y * -1., z: self.z * -1. }
    }
}

struct Ray {
    origin: Vec3,
    dir: Vec3,
}

struct RaycastHit {
    point: Vec3,
    normal: Vec3,
    mat: Material,
}

struct PointLight {
    origin: Vec3,
    intensity: f32,
}

#[derive(Debug, Copy, Clone)]
struct Material { 
    color: Vec3,
    phong_exp: f32,
    phong_const: f32,
    diffuse_const: f32,
    ambient_const: f32,
    reflectance: f32,
}

impl Material {
    fn blank() -> Self {
        Material { color: Vec3::new(0., 0., 0.), phong_exp: 0., phong_const: 0., diffuse_const: 0., ambient_const: 0., reflectance: 0. }
    }

    fn new(color: Vec3, phong_exp: f32, phong_const: f32, diffuse_const: f32, ambient_const: f32, reflectance: f32) -> Self {
        Material { color, phong_exp, phong_const, diffuse_const, ambient_const, reflectance }
    }
}

struct Sphere {
    center: Vec3,
    radius: f32,
    mat: Material,
}

impl Sphere {
    fn new(center: Vec3, radius: f32, mat: Material) -> Self {
        Sphere { center, radius, mat }
    }

    // Returns intersection distance along ray or None for no intersection
    fn intersects_ray(&self, ray: &Ray) -> Option<f32> {
        let l = self.center - ray.origin;
        let pld = l * ray.dir.normalized();
        let d_2 = l * l - pld * pld;
        if d_2 > self.radius.powi(2) { return None; }
        let td = (self.radius.powi(2) - d_2).sqrt();
        let t_0 = pld - td;
        let t_1 = pld + td;
        if t_0 > 0. { Some(t_0) } else if t_1 > 0. { Some(t_1) } else { None }
    }
}

struct Scene {
    spheres: Vec<Sphere>,
    lights: Vec<PointLight>,
}

// Returns RaycastHit with info or None if there is no intersection
fn scene_hit(ray: &Ray, scene: &Scene, max_dist: f32) -> Option<RaycastHit> {
    let mut min_dist = f32::MAX;
    let mut surface_mat = Material::blank();
    let mut surface_point = Vec3::origin();
    let mut surface_normal = Vec3::origin();

    for sphere in &scene.spheres {
        if let Some(cur_dist) = sphere.intersects_ray(ray) {
            if cur_dist < min_dist {
                min_dist = cur_dist;
                surface_mat = sphere.mat;
                surface_point = ray.origin + ray.dir * cur_dist;
                surface_normal = (surface_point - sphere.center).normalized();
            }
        }
    }
    if min_dist < max_dist { // Max draw distance
        return Some(RaycastHit {
            point: surface_point,
            normal: surface_normal,
            mat: surface_mat,
        });
    }
    return None;
}

// Cast a ray and return the pixel color as a Vec3
fn raycast(ray: &Ray, scene: &Scene, depth: i32) -> Vec3 {
    if depth < 4 {
        if let Some(hit_info) = scene_hit(ray, scene, 1000.) {
            let surface_mat = hit_info.mat;
            let surface_point = hit_info.point;
            let surface_normal = hit_info.normal;
            let mut diffuse_intensity = 0.;
            let mut specular_intensity = 0.;

            // Reflect
            let reflect_dir = -ray.dir.reflect_on(&surface_normal).normalized();
            let reflect_point = surface_point + surface_normal * 0.001;
            let reflect_color = raycast(&Ray { origin: reflect_point, dir: reflect_dir }, &scene, depth + 1);  

            for light in &scene.lights {
                let light_vec = light.origin - surface_point;
                let light_dir = light_vec.normalized();
                let light_dist = light_vec.mag();

                // Check if point is in shadow
                let shadow_point = surface_point + surface_normal * 0.001;
                if let Some(_hit) = scene_hit(&Ray { origin: shadow_point, dir: light_dir }, scene, light_dist) {
                    continue;
                }

                diffuse_intensity += (light_dir * surface_normal).max(0.) * light.intensity;
                specular_intensity += (light_dir.reflect_on(&surface_normal) * ray.dir).min(0.).powf(surface_mat.phong_exp) * light.intensity;
            }

            let diffuse_color = surface_mat.color * diffuse_intensity * surface_mat.diffuse_const;
            let specular_color = Vec3::new(1., 1., 1.) * specular_intensity * surface_mat.phong_const;
            let ambient_color = surface_mat.color * surface_mat.ambient_const;
            let reflection = reflect_color * surface_mat.reflectance;
            return diffuse_color + specular_color + ambient_color + reflection;
        }
    }

    let height = ray.dir.y;
    return Vec3::new(height, height, height); // Background color
}

fn main() -> std::io::Result<()> {
    let width = 500;
    let height = 500;
    let fov = std::f32::consts::PI / 3.;

    // Scene construction
    let m_blue = Material::new(Vec3::new(0.1, 0.1, 0.4), 40., 0.4, 1., 0.1, 0.0);
    let m_red = Material::new(Vec3::new(0.7, 0.02, 0.05), 250., 1., 1.2, 0.2, 0.);
    let m_mirror = Material::new(Vec3::new(1., 1., 1.), 1500., 1., 0., 0., 0.75);
    let m_ground = Material::new(Vec3::new(0.2, 0.2, 0.2), 0., 0., 1., 0.1, 0.17);
    let mut spheres = Vec::new();
    spheres.push(Sphere::new(Vec3::new(0., -1.25, -5.), 1., m_red));
    spheres.push(Sphere::new(Vec3::new(-2., -0.75, -7.), 1.2, m_mirror));
    spheres.push(Sphere::new(Vec3::new(0.8, 0.45, -4.), 1., m_blue));
    spheres.push(Sphere::new(Vec3::new(0., -7_002.25, 0.), 7_000., m_ground));
    let mut lights = Vec::new();
    lights.push(PointLight { origin: Vec3::new(8., 8., 10.), intensity: 0.8 });
    lights.push(PointLight { origin: Vec3::new(-3., 4., 5.), intensity: 0.65 });
    let scene = Scene { spheres: spheres, lights: lights };

    let mut data = Vec::new();
    for j in 0..height {
        for i in 0..width {
            let w = i as f32;
            let h = j as f32;
            let x = (fov / 2.).tan() * (2. * (w + 0.5) / width as f32 - 1.) * (width as f32 / height as f32);
            let y = (fov / 2.).tan() * -(2. * (h + 0.5) / height as f32 - 1.);
            let z = -1.;
            let dir = Vec3::new(x, y, z).normalized();
            data.push(raycast(&Ray { origin: Vec3::origin(), dir: dir }, &scene, 0));
        }
    }

    let mut buffer = String::new();
    buffer.push_str(&format!("P3\n{} {}\n255\n", width, height));
    for d in data {
        buffer.push_str(&format!("{} {} {}\n", 
            ((d.x.min(1.) * 255.) as u8), 
            ((d.y.min(1.) * 255.) as u8), 
            ((d.z.min(1.) * 255.) as u8)));
    }
    let mut file = File::create("render.ppm")?;
    file.write_all(&buffer.as_bytes())?;

    Ok(())
}
