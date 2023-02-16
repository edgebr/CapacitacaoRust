mod rectangles {
    pub struct Rectangle {
        x: u8,
        y: u8,
        width: u8,
        height: u8,
        lines: [char; 6],
    }

    pub enum BorderWidth {
        Light,
        Heavy,
        Double,
    }

    enum Position {
        DownRight,
        DownLeft,
        UpRight,
        UpLeft,
        Horizontal,
        Vertical,
    }

    impl Rectangle {
        pub fn builder() -> RectangleBuilder {
            RectangleBuilder(0, 0, 3, 3, BorderWidth::Light)
        }
        pub fn new(x: u8, y: u8, width: u8, height: u8, border: BorderWidth) -> Self {
            let lines = match border {
                BorderWidth::Light => ['┌', '┐', '└', '┘', '─', '│'],
                BorderWidth::Heavy => ['┏', '┓', '┗', '┛', '━', '┃'],
                BorderWidth::Double => ['╔', '╗', '╚', '╝', '═', '║'],
            };

            if width < 3 || height < 3 {
                panic!("Invalid rectangle fields");
            }

            Rectangle {
                x,
                y,
                width,
                height,
                lines,
            }
        }

        pub fn new_light(x: u8, y: u8, width: u8, height: u8) -> Rectangle {
            Rectangle::new(x, y, width, height, BorderWidth::Light)
        }
        pub fn new_heavy(x: u8, y: u8, width: u8, height: u8) -> Rectangle {
            Rectangle::new(x, y, width, height, BorderWidth::Heavy)
        }
        pub fn new_double(x: u8, y: u8, width: u8, height: u8) -> Rectangle {
            Rectangle::new(x, y, width, height, BorderWidth::Double)
        }

        fn down_right(&self) -> char {
            self.lines[Position::DownRight as usize]
        }
        fn down_left(&self) -> char {
            self.lines[Position::DownLeft as usize]
        }
        fn up_right(&self) -> char {
            self.lines[Position::UpRight as usize]
        }
        fn up_left(&self) -> char {
            self.lines[Position::UpLeft as usize]
        }
        fn horizontal(&self) -> char {
            self.lines[Position::Horizontal as usize]
        }
        fn vertical(&self) -> char {
            self.lines[Position::Vertical as usize]
        }

        fn draw(&self) {
            print!("{}", "\n".repeat(self.y as usize));

            for line in 0..self.height {
                match line {
                    0 => {
                        println!(
                            "{}{}{}{}",
                            " ".repeat(self.x as usize),
                            self.down_right(),
                            self.horizontal()
                                .to_string()
                                .repeat((self.width - 2) as usize),
                            self.down_left()
                        );
                    }
                    l if l == (self.height - 1) => {
                        println!(
                            "{}{}{}{}",
                            " ".repeat(self.x as usize),
                            self.up_right(),
                            self.horizontal()
                                .to_string()
                                .repeat((self.width - 2) as usize),
                            self.up_left()
                        );
                    }
                    _ => {
                        println!(
                            "{}{}{}{}",
                            " ".repeat(self.x as usize),
                            self.vertical(),
                            " ".repeat((self.width - 2) as usize),
                            self.vertical()
                        );
                    }
                }
            }
        }
    }

    pub struct RectangleBuilder(u8, u8, u8, u8, BorderWidth);

    impl RectangleBuilder {
        pub fn x(mut self, x: u8) -> Self {
            self.0 = x;
            self
        }
        pub fn y(mut self, y: u8) -> Self {
            self.1 = y;
            self
        }
        pub fn width(mut self, width: u8) -> Self {
            assert!(width > 3, "Invalid width");

            self.2 = width;
            self
        }
        pub fn height(mut self, height: u8) -> Self {
            assert!(height > 3, "Invalid height");

            self.3 = height;
            self
        }
        pub fn border_width(mut self, border: BorderWidth) -> Self {
            self.4 = border;
            self
        }
        pub fn build(self) -> Rectangle {
            Rectangle::new(self.0, self.1, self.2, self.3, self.4)
        }
    }

    pub fn print(rectangles: &[Rectangle]) {
        for rect in rectangles {
            rect.draw();
        }
    }
}

use crate::rectangles::BorderWidth;
use rectangles::Rectangle;

fn main() {
    let rects = [
        Rectangle::new(0, 0, 3, 3, BorderWidth::Light),
        Rectangle::new_light(10, 0, 5, 5),
        Rectangle::new_light(2, 2, 20, 7),
        Rectangle::new_heavy(6, 2, 20, 7),
        Rectangle::new_double(10, 2, 20, 7),
        Rectangle::builder().x(15).width(20).build(),
        Rectangle::builder()
            .y(1)
            .width(20)
            .height(9)
            .border_width(BorderWidth::Double)
            .build(),
    ];
    rectangles::print(&rects);
}
