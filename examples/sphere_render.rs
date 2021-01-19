use raytracer::prelude::*;

const SCREEN_WIDTH: usize = 100;
const SCREEN_HEIGHT: usize = 100;

const CANVAS_WIDTH: usize = 800;
const CANVAS_HEIGHT: usize = 800;


// Original values:
// const CANVAS_Z: f64 = 10.0;
// const RAY_ORIGIN_Z: f64  = -5.0;
const SCREEN_Z: f64 = 60.0;
const RAY_ORIGIN_Z: f64  = -2.0;

const SPHERE_COLOR: Color = color_rgb!(1.0, 0.2, 1.0);

fn main() {
    let mut canvas = Canvas::new(CANVAS_WIDTH, CANVAS_HEIGHT);

    let ray_origin = point(0.0, 0.0, RAY_ORIGIN_Z);
    let screen_transform = translation(0.0, 0.0, SCREEN_Z);
    let canvas_transform = get_canvas_transform();
    
    let pixel_transform = screen_transform * canvas_transform;

    let sphere = Sphere::new(
        rotation_z(std::f64::consts::FRAC_PI_4) * scaling(1.3, 1.0, 1.0),
        Material { color: SPHERE_COLOR, ..Material::default() },
    );
    let light = Light::new(point(-10.0, 10.0, -10.0), Color::WHITE);

    let mut xs = Intersections::empty();
    for row in 0..CANVAS_HEIGHT {
        for col in 0..CANVAS_WIDTH {
            xs.clear();
            let pixel_pos = pixel_transform * point(col as f64, row as f64, 0.0);
            let ray = Ray::new(ray_origin,
                               (pixel_pos - ray_origin).normalize());

            sphere.intersect(ray, &mut xs);
            if let Some(intersection) = xs.hit() {
                let hit_pos = ray.at(intersection.t);
                let eyev = -ray.direction;
                let normalv = sphere.normal_at(hit_pos);
                
                let color = intersection.object.material.lighting(
                    light, hit_pos, eyev, normalv, false
                );
                canvas.write_pixel(row as i32, col as i32, color);
            }
        }
    }

    canvas.save("out.ppm").unwrap();
    println!("Saved to out.ppm");
}

fn get_canvas_transform() -> M4 {
    let width_ratio = SCREEN_WIDTH as f64 / CANVAS_WIDTH as f64;
    let height_ratio = SCREEN_HEIGHT as f64 / CANVAS_HEIGHT as f64;
    scaling(width_ratio, -height_ratio, 0.0)
        * translation(-(CANVAS_WIDTH as f64) / 2.0, -(CANVAS_HEIGHT as f64) / 2.0, 0.0)
}
