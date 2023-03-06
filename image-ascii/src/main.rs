use image::{GenericImageView, Rgba};

struct PixelGroup {
    top_left: Pixel,
    top_right: Pixel,
    bottom_left: Pixel,
    bottom_right: Pixel,
}

struct Pixel {
    intensity: u8,
}

impl PixelGroup {
    fn new(
        top_left: Pixel,
        top_right: Pixel,
        bottom_left: Pixel,
        bottom_right: Pixel,
    ) -> PixelGroup {
        PixelGroup {
            top_left,
            top_right,
            bottom_left,
            bottom_right,
        }
    }
    fn to_char(&self) -> char {
        match (
            self.top_left.is_dark(),
            self.top_right.is_dark(),
            self.bottom_left.is_dark(),
            self.bottom_right.is_dark(),
        ) {
            (true, true, true, true) => 'H',
            (true, false, true, true) => 'L',
            (true, true, false, true) => 'P',
            (true, true, true, false) => '7',
            (true, false, false, true) => '/',
            (true, false, false, false) => 'd',
            (true, true, false, false) => '.',
            (true, false, true, false) => '.',
            (false, true, true, true) => '.',
            (false, false, true, true) => '_',
            (false, true, false, true) => '.',
            (false, true, true, false) => '.',
            (false, false, false, true) => '.',
            (false, false, false, false) => ' ',
            (false, true, false, false) => '.',
            (false, false, true, false) => 'd',
        }
    }
}
impl Pixel {
    fn new(rbga: Rgba<u8>) -> Pixel {
        Pixel {
            intensity: if rbga[3] == 0 {
                0
            } else {
                rbga[0] / 3 + rbga[1] / 3 + rbga[2] / 3
            },
        }
    }
    fn is_dark(&self) -> bool {
        self.intensity > 100
    }
}
fn main() {
    println!("Hello, world!");
    let img = image::open("capoo.png").unwrap();
    let (width, height) = img.dimensions();
    println!("width {:?} height {:?}", width, height);
    for y in (0..height).step_by(4) {
        for x in (0..width).step_by(4) {
            let pg: PixelGroup = PixelGroup::new(
                Pixel::new(img.get_pixel(x, y)),
                Pixel::new(img.get_pixel(x, y + 1)),
                Pixel::new(img.get_pixel(x + 1, y)),
                Pixel::new(img.get_pixel(x + 1, y + 1)),
            );
            print!("{}", pg.to_char());
        }
        println!("");
    }
}
