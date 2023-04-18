///
/// Procedural macros are a way for you to extend the Rust compiler
/// Content used to implement this code:
///  - https://developerlife.com/2022/03/30/rust-proc-macro
///  - https://doc.rust-lang.org/reference/procedural-macros.html
///  - https://veykril.github.io/tlborm/introduction.html
///
///
extern crate getters;
use getters::Getters;

extern crate my_proc_macros_lib;
use my_proc_macros_lib::{make_func, show_streams, Greetings};

use greetings::Greetings;

#[show_streams]
fn ordinary_function() -> u32 {
    30
}

#[show_streams(pre_fn, post_fn)]
fn ordinary_function2() -> u32 {
    30
}

#[show_streams [attr => value, attr2 => value]]
fn ordinary_function3() -> u32 {
    30
}

#[derive(Greetings)]
struct Human;

#[derive(Greetings)]
struct Robot;

make_func!();

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Getters)]
struct Rectangle {
    width: u32,
    height: u32,
    color: Color,
}

// #[derive(Getters)]
// struct Circle(u32);

// #[derive(Getters)]
// enum ScreenType {}

fn main() {
    ordinary_function();
    ordinary_function2();
    ordinary_function3();
    generated_func();
    Human::hello();
    Robot::hello();

    let rect = Rectangle::new(10, 100, Color::Red);
    println!(
        "Rectangle {{ width: {}, height: {}, color: {:?} }}",
        rect.width(),
        rect.height(),
        rect.color(),
    );
}
