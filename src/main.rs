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
        Vec3 { x: x, y: y, z: z }
    }

    fn mag(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    fn normalized(&self) -> Vec3 {
        let mag = self.mag();
        Vec3 { x: self.x / mag, y: self.y / mag, z: self.z / mag }
    }

    fn origin() -> Self {
        Vec3 { x: 0., y: 0., z: 0. }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = f32;
    fn mul(self, v: Vec3) -> f32 {
        // Dot product
        self.x * v.x + self.y * v.y + self.z * v.z
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, f: f32) -> Vec3 {
        Vec3 { x: f * self.x, y: f * self.y, z: f * self.z }
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, v: Vec3) -> Vec3 {
        Vec3 { x: self.x + v.x, y: self.y + v.y, z: self.z + v.z }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, v: Vec3) -> Vec3 {
        Vec3 { x: self.x - v.x, y: self.y - v.y, z: self.z - v.z }
    }
}

struct Ray {
    origin: Vec3,
    dir: Vec3
}

struct Sphere {
    center: Vec3,
    radius: f32
}

impl Sphere {
    fn new(c: Vec3, r: f32) -> Sphere {
        Sphere { center: c, radius: r }
    }

    fn intersects_ray(&self, ray: &Ray, t_0: &mut f32 ) -> bool {
        let l = self.center - ray.origin;
        let pld = l * ray.dir.normalized();
        let d_2 = l * l - pld * pld;
        if d_2 > self.radius.powi(2) {
            return false;
        }
        let td = (self.radius.powi(2) - d_2).sqrt();
        *t_0 = pld - td;
        let t_1 = pld + td;
        *t_0 = if *t_0 < 0. { t_1 } else { *t_0 };
        return *t_0 > 0.;
    }
}

fn main() -> std::io::Result<()> {
    let width = 500;
    let height = 500;
    let fov = std::f32::consts::PI / 2.;

    let obj = Sphere::new(Vec3::new(0., 0., -5.), 1.);

    let mut data = Vec::new();
    for j in 0..height {
        for i in 0..width {
            let w = i as f32;
            let h = j as f32;
            let x = (fov / 2.).tan() * (2. * (w + 0.5) / width as f32 - 1.) * (width as f32 / height as f32);
            let y = (fov / 2.).tan() * -(2. * (h + 0.5) / height as f32 - 1.);
            let z = -1.;
            let dir = Vec3::new(x, y, z).normalized();
            data.push(raycast(&Ray { origin: Vec3::origin(), dir: dir }, &obj));
        }
    }

    let mut buffer = String::new();
    buffer.push_str(&format!("P3\n{} {}\n255\n", width, height));
    for d in data {
        buffer.push_str(&format!("{} {} {}\n", 
            ((d.x * 255.) as u8), 
            ((d.y * 255.) as u8), 
            ((d.z * 255.) as u8)));
    }
    let mut file = File::create("render.ppm")?;
    file.write_all(&buffer.as_bytes())?;

    Ok(())
}

// Cast a ray and return the pixel color as a Vec3
fn raycast(ray: &Ray, obj: &Sphere) -> Vec3 {
    let mut min_dist = f32::MAX;
    if obj.intersects_ray(ray, &mut min_dist) {
        return Vec3 { x: 1., y: 1., z: 1. } // White; hit
    }
    return Vec3 { x: 0.2, y: 0.2, z: 0.2 } // Gray; miss
}
