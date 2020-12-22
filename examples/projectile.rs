use raytracer::prelude::*;

#[derive(Debug, Copy, Clone)]
struct Projectile {
    pos: T4,
    velocity: T4,
}

#[derive(Debug, Copy, Clone)]
struct Environment {
    gravity: T4,
    wind: T4,
}

impl Environment {
    fn tick(&self, proj: Projectile) -> Projectile {
        let pos = proj.pos + proj.velocity;
        let velocity = proj.velocity + self.gravity + self.wind;
        Projectile { pos, velocity }
    }
}

fn main() {
    let pos = point(0.0, 1.0, 0.0);
    let velocity = vector(1.0, 1.8, 0.0).normalize() * 11.25;
    let gravity = vector(0.0, -0.1, 0.0);
    let wind = vector(-0.01, 0.0, 0.0);
    let trail_color = color(1.0, 0.0, 0.0);

    let e = Environment { gravity, wind };
    let mut c = Canvas::new(900, 550);

    let mut p = Projectile { pos, velocity };
    while p.pos.y > 0.0 && p.pos.x > -10.0 {
        c.write_pixel(p.pos.x as i32, c.get_height() - (p.pos.y as i32), trail_color);
        p = e.tick(p);
    }

    c.save("out.ppm");
    println!("Saved to out.ppm");

}
