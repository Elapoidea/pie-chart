use image::{RgbImage, Rgb, Rgba, RgbaImage};

const WINDOW_X: u32 = 1000;
const WINDOW_Y: u32 = 1000;
const PI: f32 = 3.14;
const COLOURS: [Rgba<u8>; 9] = [
    Rgba([85, 172, 238, 255]), // Blue
    Rgba([221, 46, 68, 255]), // Red
    Rgba([119, 178, 86, 255]), // Green
    Rgba([253, 203, 88, 255]), // Yellow
    Rgba([170, 141, 215, 255]), // Purple
    Rgba([244, 144, 12, 255]), // Orange
    Rgba([227, 229, 231, 255]), // White
    Rgba([49, 55, 61, 255]), // Black
    Rgba([193, 105, 79, 255]), // Brown
];

struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn angle(&self) -> f32 {
        if self.x < 0 && self.y == 0 { 1.0 }
        else { (self.y as f32).atan2(self.x as f32) }
    }

    fn within_angle_range(&self, size: f32, offset: f32) -> bool {
        let angle = self.angle() / 2.0 / PI + 0.5;

        angle < size + offset && angle > offset
    }

    fn in_circle(&self, d: u32) -> bool {
        self.x*self.x + self.y*self.y < (d*d) as i32
    }
}

fn main() {
    let mut img = RgbaImage::new(WINDOW_X, WINDOW_Y);

    let items = vec![0.05, 0.05, 0.05, 0.05, 0.1, 0.1, 0.2, 0.4];

    let center_x = (WINDOW_X / 2) as i32;
    let center_y = (WINDOW_Y / 2) as i32;

    let mut sum_offset = 0.0;
    
    for a in 0..items.len() {
        for i in 0..WINDOW_X {
            for j in 0..WINDOW_Y {
    
                let cartesian = Point { 
                    x: i as i32 - center_x, 
                    y: center_y - j as i32 
                };
                
                if cartesian.in_circle(WINDOW_X / 2) && !cartesian.in_circle(WINDOW_X / 3) {
                    if cartesian.within_angle_range(items[a], sum_offset) {
                        img.put_pixel(i, j, COLOURS[a]);  
                    }  
                }
            }
        }

        sum_offset += items[a];
    }

    img.save("empty.png");
}



fn per_to_rad(p: f64) -> f64 {
    p * 2.0 * 3.141
}
