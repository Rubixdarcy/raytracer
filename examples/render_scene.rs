use raytracer::prelude::*;
use std::f32::consts::FRAC_PI_2;
use std::f32::consts::FRAC_PI_3;
use std::f32::consts::FRAC_PI_4;

// const WIDTH: usize = 100;
// const HEIGHT: usize = 50;

// const WIDTH: usize = 640;
// const HEIGHT: usize = 480;

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;


const FOV: f32 = FRAC_PI_3;

fn main() {
    
    let floor = Sphere {
        transform: scaling(10.0, 0.01, 10.0),
        material: Material {
            color: color_rgb!(1.0, 0.9, 0.9),
            specular: 0.0,
            ..Material::default()
        },
        ..Sphere::default()
    };
    let left_wall = Sphere {
        transform: translation(0.0, 0.0, 5.0)
                   * rotation_y(-FRAC_PI_4)
                   * rotation_x(FRAC_PI_2)
                   * scaling(10.0, 0.01, 10.0),
        material: floor.material,
        ..Sphere::default()
    };
    let right_wall = Sphere {
        transform: translation(0.0, 0.0, 5.0)
                   * rotation_y(FRAC_PI_4)
                   * rotation_x(FRAC_PI_2)
                   * scaling(7.0, 0.01, 7.0),
        material: floor.material,
        ..Sphere::default()
    };

    let middle = Sphere {
        transform: translation(-0.5, 1.0, 0.5),
        material: Material {
            color: color_rgb!(0.1, 1.0, 0.5),
            diffuse: 0.7,
            specular: 0.3,
            ..Material::default()
        },
        ..Sphere::default()
    };
    let right = Sphere {
        transform: translation(1.5, 0.5, -0.5)
                   * scaling(0.5, 0.5, 0.5),
        material: Material {
            color: color_rgb!(0.5, 1.0, 0.1),
            diffuse: 0.7,
            specular: 0.3,
            ..Material::default()
        },
        ..Sphere::default()
    };
    let left = Sphere {
        transform: translation(-1.5, 0.33, -0.75)
                   * scaling(0.33, 0.33, 0.33),
        material: Material {
            color: color_rgb!(1.0, 0.8, 0.1),
            diffuse: 0.7,
            specular: 0.3,
            ..Material::default()
        },
        ..Sphere::default()
    };

    let light = Light::new(point(-10.0, 10.0, -10.0), Color::WHITE);

    let camera = Camera::new(
        WIDTH,
        HEIGHT,
        FOV,
        view_transform(
            point(0.0, 1.5, -5.0),
            point(0.0, 1.0, 0.0),
            vector(0.0, 1.0, 0.0)
        )
    );

    let world = World::new(
        vec![floor, left_wall, right_wall, left, middle, right],
        vec![light]
    );

    let canvas = camera.render(&world);

    canvas.save("out.ppm").unwrap();
    println!("Saved to out.ppm");
}