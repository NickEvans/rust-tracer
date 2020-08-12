use std::fs::File;
use std::io::prelude::*;

struct Vector {
    x: f32,
    y: f32,
    z: f32
}

impl Vector {
    fn new(x: f32, y: f32, z: f32) -> Vector {
        Vector { x: x, y: y, z: z }
    }
}

fn main() -> std::io::Result<()> {
    let width = 500;
    let height = 500;

    let mut data = Vec::new();
    for w in 0..width {
        for h in 0..height {
            data.push(Vector::new(w as f32 / width as f32, 
                h as f32 / height as f32, 
                0.));
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
