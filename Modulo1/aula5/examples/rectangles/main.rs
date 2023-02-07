extern crate core;

fn main() {
    println!("Printing squares");
    let square1 = Rectangle {
        origin: Point { x: 0, y: 0 },
        dimensions: Dimension {
            width: 3,
            height: 3,
        },
        border: BorderWidth::Light,
    };
    square1.draw();
    let square1 = Rectangle {
        origin: Point { x: 10, y: 0 },
        dimensions: Dimension {
            width: 5,
            height: 5,
        },
        border: BorderWidth::Light,
    };
    square1.draw();
    let square1 = Rectangle {
        origin: Point { x: 5, y: 5 },
        dimensions: Dimension {
            width: 20,
            height: 7,
        },
        border: BorderWidth::Light,
    };
    square1.draw();
    let square1 = Rectangle {
        origin: Point { x: 5, y: 5 },
        dimensions: Dimension {
            width: 20,
            height: 7,
        },
        border: BorderWidth::Double,
    };
    square1.draw();
    let square1 = Rectangle {
        origin: Point { x: 5, y: 5 },
        dimensions: Dimension {
            width: 20,
            height: 7,
        },
        border: BorderWidth::Heavy,
    };
    square1.draw();
}

struct Point {
    x: u8,
    y: u8,
}

struct Dimension {
    pub width: u8,
    pub height: u8,
}

impl Dimension {
    fn is_valid(&self) -> bool {
        self.width >= 3 && self.height >= 3
    }
}

enum BorderWidth {
    Light,
    Heavy,
    Double,
}

struct Rectangle {
    pub origin: Point,
    pub dimensions: Dimension,
    pub border: BorderWidth,
}

impl Rectangle {
    fn draw(&self) {
        if !self.dimensions.is_valid() {
            panic!("Invalid dimensions!");
        }

        print!("{}", "\n".repeat(self.origin.y as usize));

        for line in 0..self.dimensions.height {
            match line {
                0 => {
                    println!(
                        "{}{}{}{}",
                        " ".repeat(self.origin.x as usize),
                        self.down_right(),
                        self.horizontal()
                            .repeat((self.dimensions.width - 2) as usize),
                        self.down_left()
                    );
                }
                l if l == (self.dimensions.height - 1) => {
                    println!(
                        "{}{}{}{}",
                        " ".repeat(self.origin.x as usize),
                        self.up_right(),
                        self.horizontal()
                            .repeat((self.dimensions.width - 2) as usize),
                        self.up_left()
                    );
                }
                _ => {
                    println!(
                        "{}{}{}{}",
                        " ".repeat(self.origin.x as usize),
                        self.vertical(),
                        " ".repeat((self.dimensions.width - 2) as usize),
                        self.vertical()
                    );
                }
            }
        }
    }
    fn down_right(&self) -> &str {
        match self.border {
            BorderWidth::Light => "┌",
            BorderWidth::Heavy => "┏",
            BorderWidth::Double => "╔",
        }
    }
    fn down_left(&self) -> &str {
        match self.border {
            BorderWidth::Light => "┐",
            BorderWidth::Heavy => "┓",
            BorderWidth::Double => "╗",
        }
    }
    fn up_right(&self) -> &str {
        match self.border {
            BorderWidth::Light => "└",
            BorderWidth::Heavy => "┗",
            BorderWidth::Double => "╚",
        }
    }
    fn up_left(&self) -> &str {
        match self.border {
            BorderWidth::Light => "┘",
            BorderWidth::Heavy => "┛",
            BorderWidth::Double => "╝",
        }
    }
    fn horizontal(&self) -> &str {
        match self.border {
            BorderWidth::Light => "─",
            BorderWidth::Heavy => "━",
            BorderWidth::Double => "═",
        }
    }
    fn vertical(&self) -> &str {
        match self.border {
            BorderWidth::Light => "│",
            BorderWidth::Heavy => "┃",
            BorderWidth::Double => "║",
        }
    }
}
