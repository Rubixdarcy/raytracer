use crate::color::*;

#[derive(Clone, Debug)]
pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self::new_with_color(width, height, Color::default())
    }

    pub fn new_with_color(width: usize, height: usize, c: Color) -> Self {
        let pixels: Vec<Color> = vec![c; width * height];
        Canvas { width, height, pixels }
    }

    pub fn get_width(&self) -> i32 {
        self.width as i32
    }

    pub fn get_height(&self) -> i32 {
        self.height as i32
    }

    pub fn pixel_at(&self, x: i32, y: i32) -> Color {
        self.pixels[y as usize * self.width + x as usize]
    }

    pub fn write_pixel(&mut self, x: i32, y: i32, c: Color) {
        let i = y as usize * self.width + x as usize;
        if i >= self.pixels.len() { return; }
        self.pixels[i] = c;
    }

    pub fn to_ppm(&self) -> String {
        let mut s = WrappingStringBuilder::new(70);

        s.append_line("P3");
        s.append_line(&format!("{} {}", self.width, self.height));
        s.append_line("255");

        for (i, p) in self.pixels.iter().copied().enumerate() {
            let r = to_ppm_value(p.red);
            let g = to_ppm_value(p.green);
            let b = to_ppm_value(p.blue);

            s.append(&r.to_string());
            s.append(&g.to_string());
            s.append(&b.to_string());

            if (i + 1) % self.width == 0 {
                s.newline();
            }
        }

        return s.into_string();
    }

    pub fn save(&self, file: &str) -> std::io::Result<()> {
        std::fs::write(file, self.to_ppm())
    }
}

fn to_ppm_value(x: f64) -> i32 {
    ((x * 255.0).round() as i32).max(0).min(255)
}

#[derive(Debug, Clone)]
struct WrappingStringBuilder {
    max_width: usize,
    curr_width: usize,
    s: String,
}

impl WrappingStringBuilder {
    fn new(max_width: usize) -> Self {
        WrappingStringBuilder {
            max_width,
            curr_width: 0,
            s: String::new(),
        }
    }

    fn newline(&mut self) {
        self.s.push('\n');
        self.curr_width = 0;
    }

    fn append(&mut self, s: &str) {
        let w0 = self.curr_width;
        let ws = s.len();

        if w0 == 0 {
            self.s.push_str(s);
            self.curr_width = s.len();
            return;
        }

        if w0 + ws + 1  > self.max_width {
            self.newline();
            self.append(s);
            return;
        }

        self.s.push(' ');
        self.s.push_str(s);
        self.curr_width += ws + 1;
    }
    
    fn append_line(&mut self, s: &str) {
        self.append(s);
        self.newline();
    }

    fn into_string(self) -> String {
        self.s
    }
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_canvas() {
        let c = Canvas::new(10, 20);

        for x in 0..10i32 {
            for y in 0..20i32 {
                assert_eq!(c.pixel_at(x, y), color(0.0, 0.0, 0.0));
            }
        }
    }

    #[test]
    fn write_single_pixel() {
        let mut c = Canvas::new(10, 20);
        let red = color(1.0, 0.0, 0.0);

        c.write_pixel(2, 3, red);
        assert_eq!(c.pixel_at(2, 3), red);
    }

    #[test]
    fn canvas_ppm() {
        let mut c = Canvas::new(5, 3);
        c.write_pixel(0, 0, color(1.5, 0.0, 0.0));
        c.write_pixel(2, 1, color(0.0, 0.5, 0.0));
        c.write_pixel(4, 2, color(-0.5, 0.0, 1.0));
        let s = c.to_ppm();
        assert_eq!(s,
"P3
5 3
255
255 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 128 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 255
");
    }

    #[test]
    fn canvas_ppm_max_width() {
        let s = Canvas::new_with_color(10, 2, color(1.0, 0.8, 0.6)).to_ppm();
        assert_eq!(s,
"P3
10 2
255
255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204
153 255 204 153 255 204 153 255 204 153 255 204 153
255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204
153 255 204 153 255 204 153 255 204 153 255 204 153
");
    }
}
