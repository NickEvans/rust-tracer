# rust-tracer
A simple, in-progress ray-tracing algorithm implemented from scratch in Rust.
![Sample Render](https://raw.githubusercontent.com/NickEvans/rust-tracer/master/render.png)
Implements Phong model for diffuse and specular lighting, shadows, and mirror reflections.

## To Run
```
$ cd rust-tracer
$ cargo run
```
The program will output a rendering to a plain .ppm file.

## Todo
- Refraction
- Depth of field
- More shape primitives