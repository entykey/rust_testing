use std::fs::File;
use std::io::Write;
use std::cmp;

const SCALING_FACTOR: f64 = 1.4 as f64;

#[derive(Clone)]
struct RGB {
    red: u8,
    green: u8,
    blue: u8
}

fn serialize_rgb(pixels: &Vec<RGB>, size: usize) -> Vec<u8> {
    // for saving to a file. Is there any way we could do this
    // without constructing a new array? Would be much faster
    let mut output: Vec<u8> = Vec::with_capacity(size * 3);
    for pix in pixels {
        output.push(pix.red);
        output.push(pix.green);
        output.push(pix.blue);
    }
    output
}

struct Canvas {
    // Using 1D array so the bytes are together in memory, should be more efficient than Vec<Vec>
    // since that would store pointers to vectors?
    pixels: Vec<RGB>,
    width: i32,
    height: i32
}

impl Canvas {
    fn set_colour(&mut self, x: i32, y: i32, colour: &RGB) {
        // make this more natural? In C++ you can overload () to get a functor
        if x > 0 && y > 0 &&  x < self.width && y <  self.height {
            self.pixels[(self.width * y + x) as usize] = colour.clone();
        }
    }

    fn write_to_file(&mut self, filename: &str) {
        let mut file = init_ppm(filename, self.width, self.height);
        file.write_all(&serialize_rgb(&self.pixels, (self.width * self.height) as usize)).expect("error");
        /* slow
        for pixel in &self.pixels {
            file.write_all(&[pixel.red, pixel.green, pixel.blue]).expect("error writing to a file");
        }*/
    }

    fn new(width: i32, height: i32) -> Canvas {
        Canvas {
            width,
            height,
            pixels: vec![RGB{red:0, green:0, blue:0}; (width * height) as usize]
        }
    }

    fn draw_square(&mut self, center: &Point, width: i32, colour: &RGB) {
        for y in cmp::max(0, center.y - width) .. cmp::min(self.height, center.y + width) {
            for x in cmp::max(0, center.x - width) .. cmp::min(self.width, center.x + width) {
                self.set_colour(x ,y, &colour);
            }
        }
    }

    fn draw_line(&mut self, from: &Point, to: &Point, width: i32, colour: &RGB) {
        // function that connects two points on the grid with a line
        if from.x == to.x { // vertical lines
            let startx = cmp::max(from.x - width, 0);
            let endx = cmp::min(from.x + width, self.width);
            let endy = cmp::max(from.y, to.y) + 1;
            let starty = cmp::min(from.y, to.y);
            for y in starty .. endy {
                for x in  startx .. endx {
                    self.set_colour(x, y, colour);
                }

            }
        }
        else {
            let k = (to.y - from.y) as f64 / (to.x - from.x) as f64;
            let n = to.y as f64 - k * to.x as f64;
            let lower = cmp::min(from.x, to.x);
            let upper = cmp::max(from.x, to.x) + 1;
            for x in lower .. upper {
                // We colour y's as a function of x's
                self.draw_square(
                    &Point {x: x, y: (k * x as f64 + n) as i32},
                    width,
                    colour
                );
            }
            if k.abs() > 1.0 {
                // for steep lines, we also have to consider x as a function of y to get good results
                let lower = cmp::min(from.y, to.y);
                let upper = cmp::max(from.y, to.y) + 1;
                for y in lower .. upper {
                    self.draw_square(
                        &Point {x: ((y as f64 - n) / k) as i32, y: y},
                        width,
                        colour
                    );
                }
            }
        }
    }

}

fn rotate_point(center: &Point, point: &Point, angle: f64) -> Point {
    // also scales down a bit
    let (sin, cos) = angle.sin_cos();
    let translated = Point {x: ((point.x - center.x) as f64 / SCALING_FACTOR) as i32,
                            y: ((point.y - center.y) as f64 / SCALING_FACTOR) as i32};
    let rotated = Point {x: (translated.x as f64 * cos - translated.y as f64 * sin) as i32,
                         y: (translated.x as f64 * sin + translated.y as f64 * cos) as i32
    };
    Point {x: rotated.x + center.x, y: rotated.y + center.y}
}

fn init_ppm(filename: &str, width: i32, height: i32) -> File {
    let mut file = File::create(format!("{}.ppm",filename)).expect("couldn't create");
    file.write_all(format!("P6 {} {} 255 ", width, height).as_bytes()).expect("error writing to a file");
    file
}

struct Point {
    x: i32,
    y: i32
}

fn main() {
    const WIDTH: i32 = 1500;
    const HEIGHT: i32 = 1500;
    let mut picture = Canvas::new(WIDTH, HEIGHT);
    draw_tree(&mut picture,
              &Point {x: WIDTH/2, y: HEIGHT},
              &Point {x: WIDTH/2, y: 3*HEIGHT/4},
              15,
              &RGB {red: 255, blue: 255, green: 255},
              0.6,
              2);
    picture.write_to_file("test");
}


fn draw_tree(mut canvas: &mut Canvas, prev: &Point, next: &Point, iter: i32, colour: &RGB, angle: f64, branches: i32) {
    // recursively generates branches.
    if iter == 0 {
        return;
    }
    canvas.draw_line(prev, next, 1, colour);
    let prev = Point {x: 2 * next.x - prev.x, y: 2 * next.y - prev.y};

    if branches % 2 == 1 {
        draw_tree(&mut canvas, &next, &rotate_point(next, &prev, 0.0), iter - 1, colour, angle, branches);
    }
    for i in 1 .. branches / 2 + 1 {
        // colours are hardcoded currently but that's not really important
        let rot_left = rotate_point(next, &prev, i as f64 * angle);
        draw_tree(&mut canvas, &next, &rot_left, iter - 1, &RGB{red: 247, green: 97, blue: 74}, angle, branches);

        let rot_right = rotate_point(next, &prev, - i as f64 * angle);
        draw_tree(&mut canvas, &next, &rot_right, iter - 1, &RGB{red: 26, green: 121, blue: 244}, angle, branches);
    }
}