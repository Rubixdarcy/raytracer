use raytracer::prelude::*;

const SCREEN_WIDTH: usize = 100;
const SCREEN_HEIGHT: usize = 100;

const CANVAS_WIDTH: usize = 800;
const CANVAS_HEIGHT: usize = 800;


// Original values:
// const CANVAS_Z: f32 = 10.0;
// const RAY_ORIGIN_Z: f32  = -5.0;
const SCREEN_Z: f32 = 60.0;
const RAY_ORIGIN_Z: f32  = -2.0;

const SPHERE_COLOR: Color = color_rgb!(1.0, 0.2, 1.0);

fn main() {
    let mut canvas = Canvas::new(CANVAS_WIDTH, CANVAS_HEIGHT);

    let ray_origin = point(0.0, 0.0, RAY_ORIGIN_Z);
    let screen_transform = translation(0.0, 0.0, SCREEN_Z);
    let canvas_transform = get_canvas_transform();
    
    let pixel_transform = screen_transform * canvas_transform;

    let sphere = Sphere::new(
        rotation_z(std::f32::consts::FRAC_PI_4) * scaling(1.3, 1.0, 1.0),
        Material { color: SPHERE_COLOR, ..Material::default() },
    );
    let light = Light::new(point(-10.0, 10.0, -10.0), Color::WHITE);

    for row in 0..CANVAS_HEIGHT {
        for col in 0..CANVAS_WIDTH {
            let pixel_pos = pixel_transform * point(col as f32, row as f32, 0.0);
            let ray = Ray::new(ray_origin,
                               (pixel_pos - ray_origin).normalize());

            if let Some(intersection) = sphere.intersect(ray).hit() {
                let hit_pos = ray.at(intersection.t);
                let eyev = -ray.direction;
                let normalv = sphere.normal_at(hit_pos);
                
                let color = intersection.object.material.lighting(
                    light, hit_pos, eyev, normalv,
                );
                canvas.write_pixel(row as i32, col as i32, color);
            }
        }
    }

    canvas.save("out.ppm").unwrap();
    println!("Saved to out.ppm");
}

fn get_canvas_transform() -> M4 {
    let width_ratio = SCREEN_WIDTH as f32 / CANVAS_WIDTH as f32;
    let height_ratio = SCREEN_HEIGHT as f32 / CANVAS_HEIGHT as f32;
    scaling(width_ratio, -height_ratio, 0.0)
        * translation(-(CANVAS_WIDTH as f32) / 2.0, -(CANVAS_HEIGHT as f32) / 2.0, 0.0)
}
