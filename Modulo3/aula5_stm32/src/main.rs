#![no_main]
#![no_std]

use panic_halt as _;

use crate::hal::{pac, prelude::*};
use cortex_m_rt::entry;
use embedded_graphics::{image::Image, image::ImageRaw, pixelcolor::BinaryColor, prelude::*};
use embedded_hal::digital::v2::InputPin;
use micromath::statistics::{Mean, StdDev, Variance};
use micromath::vector::Vector;
use micromath::vector::{Vector2d, Vector3d};
use micromath::F32Ext;
use rotary_encoder_embedded::standard::StandardMode;
use rotary_encoder_embedded::{Direction, RotaryEncoder};
use rtt_target::{rprintln, rtt_init_print};
use scapegoat::SgMap;
use sh1106::{prelude::*, Builder};
use stm32f4xx_hal as hal;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    embed_math();

    scapegoat_example();

    let (Some(dp), Some(cp)) = (pac::Peripherals::take(), cortex_m::peripheral::Peripherals::take()) else {
        panic!("Cannot take peripherals");
    };

    /* Clocks */
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(48.MHz()).freeze();

    /* GPIOs */
    let gpiob = dp.GPIOB.split();
    let scl = gpiob
        .pb8
        .into_alternate()
        .internal_pull_down(true)
        .set_open_drain();
    let sda = gpiob
        .pb9
        .into_alternate()
        .internal_pull_down(true)
        .set_open_drain();

    let gpioe = dp.GPIOE.split();
    let dt = gpioe.pe13.into_pull_up_input();
    let clk = gpioe.pe11.into_pull_up_input();
    let key = gpioe.pe9.into_pull_up_input();

    /* Display */
    let i2c = dp.I2C1.i2c((scl, sda), 400.kHz(), &clocks);
    let mut disp: GraphicsMode<_> = Builder::new().connect_i2c(i2c).into();

    /* Rotary Encoder */
    let mut enc = RotaryEncoder::new(dt, clk).into_standard_mode();

    /* Delay */
    let mut delay = cp.SYST.delay(&clocks);

    /* Images */
    let img_bytes = include_bytes!("../ssd1306-image.data");
    let raw_image: ImageRaw<BinaryColor> = ImageRaw::new(img_bytes, 128);
    let image = Image::new(&raw_image, Point::zero());

    let img_bytes_rev = invert_image(img_bytes);
    let rev_raw_image: ImageRaw<BinaryColor> = ImageRaw::new(&img_bytes_rev, 128);
    let rev_image = Image::new(&rev_raw_image, Point::zero());

    /* General Variables */
    let mut counter = 0_i32;
    let mut state = true;

    /* Start Display and Print First Image */
    disp.init().unwrap();
    disp.flush().unwrap();
    image.draw(&mut disp).unwrap();
    disp.flush().unwrap();

    /* Start Button Helper Variable */
    let mut was_pressed = key.is_low();

    loop {
        let is_pressed = key.is_low();
        if !was_pressed && is_pressed {
            state = !state;
            if state {
                image.draw(&mut disp).unwrap();
            } else {
                rev_image.draw(&mut disp).unwrap();
            }
            disp.flush().unwrap();

            was_pressed = true;
        } else if !is_pressed {
            was_pressed = false;
        }

        update_rotary_encoder(&mut enc, &mut counter);

        delay.delay_ms(10_u32);
    }
}

fn invert_image(img: &[u8]) -> [u8; 128 * 8] {
    let mut output = [0; 128 * 8];

    for (i, byte) in img.iter().enumerate() {
        output[i] = !(*byte);
    }

    output
}

fn update_rotary_encoder<DT, CLK>(enc: &mut RotaryEncoder<StandardMode, DT, CLK>, counter: &mut i32)
where
    DT: InputPin,
    CLK: InputPin,
{
    enc.update();
    match enc.direction() {
        Direction::Clockwise => {
            *counter += 1;
            rprintln!("clockwise: {}", counter);
        }
        Direction::Anticlockwise => {
            *counter -= 1;
            rprintln!("anti clockwise: {}", counter);
        }
        Direction::None => {}
    }
}

fn embed_math() {
    /* Float point approximations (F32Ext) */
    rprintln!("***** F32Ext *****");
    rprintln!("sqrt = {}", 2.0.sqrt());

    rprintln!("sin = {}", core::f32::consts::PI.sin());
    rprintln!("cos = {}", core::f32::consts::PI.cos());

    rprintln!("ceil = {}", 2.5.ceil());
    rprintln!("floor = {}", 2.5.floor());
    rprintln!("round(2.5) = {}", 2.5.round());
    rprintln!("round(2.4) = {}", 2.4.round());

    rprintln!("powi = {}", 2.1.powi(3));
    rprintln!("powi = {}", 2.1 * 2.1 * 2.1);

    rprintln!("log2 = {}", 8.0.log2());
    rprintln!("log2 = {}", 8_i32.ilog2());

    rprintln!("exp = {}", 2.0.exp());

    rprintln!("1/x = {}", 2.0.inv());
    rprintln!("1/sqrt(x) = {}", 2.0.invsqrt());

    /* Vector */
    rprintln!("***** Vector2d *****");
    let p1 = Vector2d::from((1.0, 1.0));
    let p2 = Vector2d::from((2.0, 2.0));
    rprintln!("p1.x = {}", p1.x);
    rprintln!("p1.y = {}", p1.y);
    rprintln!("p1 as array = {:?}", p1.to_array());
    rprintln!("p1 + p2 = {:?}", p1 + p2);
    rprintln!("p1 - p2 = {:?}", p1 - p2);
    rprintln!("p1 * f32 = {:?}", p1 * 3.0);
    rprintln!("p1 * p2 = {}", p1.dot(p2));
    rprintln!("length p1 = {}", p1.magnitude());
    rprintln!("distance between p1 and p2 = {}", p1.distance(p2));

    rprintln!("***** Vector3d *****");
    let p1 = Vector3d::from((1.0, 1.0, 1.0));
    let p2 = Vector3d::from((2.0, 2.0, 2.0));
    rprintln!("p1.x = {}", p1.x);
    rprintln!("p1.y = {}", p1.y);
    rprintln!("p1.z = {}", p1.z);
    rprintln!("p1 as array = {:?}", p1.to_array());
    rprintln!("p1 + p2 = {:?}", p1 + p2);
    rprintln!("p1 - p2 = {:?}", p1 - p2);
    rprintln!("p1 * f32 = {:?}", p1 * 3.0);
    rprintln!("p1 * p2 = {}", p1.dot(p2));
    rprintln!("length p1 = {}", p1.magnitude());
    rprintln!("distance between p1 and p2 = {}", p1.distance(p2));

    /* Statistical Analysis */
    let data = [
        0.001752830049,
        0.002239453029,
        0.002832703774,
        0.003547459285,
        0.004398359598,
        0.005399096651,
        0.006561581477,
        0.00789501583,
        0.009404907738,
        0.01109208347,
        0.01295175957,
        0.01497274656,
        0.0171368592,
        0.0194186055,
        0.0217852177,
        0.02419707245,
        0.02660852499,
        0.02896915528,
        0.03122539334,
        0.03332246029,
        0.03520653268,
        0.03682701403,
        0.03813878155,
        0.0391042694,
        0.03969525475,
        0.03989422804,
        0.03969525475,
        0.0391042694,
        0.03813878155,
        0.03682701403,
        0.03520653268,
        0.03332246029,
        0.03122539334,
        0.02896915528,
        0.02660852499,
        0.02419707245,
        0.0217852177,
        0.0194186055,
        0.0171368592,
        0.01497274656,
        0.01295175957,
        0.01109208347,
        0.009404907738,
        0.00789501583,
        0.006561581477,
        0.005399096651,
        0.004398359598,
        0.003547459285,
        0.002832703774,
        0.002239453029,
        0.001752830049,
        0.001358296923,
        0.001042093481,
        0.0007915451583,
        0.000595253242,
        0.0004431848412,
        0.0003266819056,
        0.0002384088201,
        0.0001722568939,
        0.0001232219168,
        0.0000872682695,
    ];

    rprintln!("***** Statistics *****");
    let mean = 0.0163;
    let stddev = 0.0141383216;
    let variance = 0.0001998921375;

    rprintln!(
        "mean = {}/{} [{}]",
        data.iter().cloned().mean(),
        mean,
        data.iter().cloned().mean() / mean
    );
    rprintln!(
        "std dev = {}/{} [{}]",
        data.as_ref().stddev(),
        stddev,
        data.as_ref().stddev() / stddev
    );
    rprintln!(
        "variance = {}/{} [{}]",
        data.as_ref().variance(),
        variance,
        data.as_ref().variance() / variance
    );
}

macro_rules! sg_map {
    ($capacity:expr) => {
        SgMap::<_, _, $capacity>::new()
    };
}

macro_rules! stack_string {
    ($capacity:expr) => {
        heapless::String::<$capacity>::new()
    };
}

fn scapegoat_example() {
    const CAPACITY: usize = 4;

    let mut map = sg_map!(CAPACITY);

    map.try_insert(3, "the").unwrap();
    map.try_insert(2, "don't blame").unwrap();
    map.try_insert(1, "Please").unwrap();
    map.try_insert(4, "borrow checker").unwrap();

    let message = map
        .iter()
        .map(|(_, v)| *v)
        .fold(stack_string![64], |mut msg, item| {
            msg.push_str(item).unwrap();
            msg.push_str(" ").unwrap();
            msg
        });

    rprintln!("Message: {}", message);
    rprintln!("map[2]: {}", map[&2]);
    rprintln!("Invalid insert: {}", map.try_insert(5, "Hello").is_err());
}
