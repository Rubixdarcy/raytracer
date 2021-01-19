use raytracer::prelude::*;

const CANVAS_WIDTH: usize = 100;
const CANVAS_HEIGHT: usize = 100;

// Original values:
// const CANVAS_Z: f64 = 10.0;
// const LIGHT_Z: f64  = -5.0;
const CANVAS_Z: f64 = 60.0;
const LIGHT_Z: f64  = -2.0;

const SPHERE_COLOR: Color = color_rgb!(1.0, 0.2, 1.0);

fn main() {
    let mut canvas = Canvas::new(CANVAS_WIDTH, CANVAS_HEIGHT);
    let screen_transform = scaling(1.0, -1.0, 0.0)
        * translation(-(CANVAS_WIDTH as f64) / 2.0, -(CANVAS_HEIGHT as f64) / 2.0, 0.0);

    let light_pos = point(0.0, 0.0, LIGHT_Z);
    let canvas_transform = translation(0.0, 0.0, CANVAS_Z);
    
    let pixel_transform = canvas_transform * screen_transform;

    //let sphere = Sphere::default();
    let sphere = Sphere::from_transform(
        rotation_z(std::f64::consts::FRAC_PI_4) * scaling(1.3, 1.0, 1.0));

    let mut xs = Intersections::empty();
    for row in 0..CANVAS_HEIGHT {
        for col in 0..CANVAS_WIDTH {
            xs.clear();
            let pixel_pos = pixel_transform * point(col as f64, row as f64, 0.0);
            let ray = Ray::new(light_pos, pixel_pos - light_pos);

            sphere.intersect(ray, &mut xs);
            if xs.len() > 0 {
                canvas.write_pixel(row as i32, col as i32, SPHERE_COLOR);
            }
        }
    }

    canvas.save("out.ppm").unwrap();
    println!("Saved to out.ppm");
}
