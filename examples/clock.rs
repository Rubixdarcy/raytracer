use raytracer::prelude::*;

const CANVAS_SIZE: usize = 200;
const CLOCK_CANVAS_RATIO: f32 = 0.8;
const CLOCK_COLOR: Color = color_rgb!(0.0, 1.0, 0.0);

fn main() {
    let mut c = Canvas::new(CANVAS_SIZE, CANVAS_SIZE);

    let zero_oclock = point(0.0, 1.0, 0.0);

    let clock_scale: f32 = CANVAS_SIZE as f32 * CLOCK_CANVAS_RATIO / 2.0;
    let clock_scale = scaling(clock_scale, clock_scale, clock_scale);
    let clock_translation = translation(CANVAS_SIZE as f32 / 2.0, CANVAS_SIZE as f32 / 2.0, 0.0);
    let clock_transform = clock_translation * clock_scale;

    for i in 0..12i32 {
        let hour_transform = rotation_z((i as f32) * std::f32::consts::PI / 6.0);
        let pos = clock_transform * hour_transform * zero_oclock;
        c.write_pixel(pos.x as i32, CANVAS_SIZE as i32 - pos.y as i32, CLOCK_COLOR);
    }

    c.save("out.ppm").unwrap();
    println!("Saved to out.ppm");
}
