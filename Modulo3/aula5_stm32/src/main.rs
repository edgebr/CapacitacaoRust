#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m_rt::{entry};
use stm32f4xx_hal as hal;
use embedded_graphics::{image::Image, image::ImageRaw, pixelcolor::BinaryColor, prelude::*};
use embedded_hal::digital::v2::InputPin;
use sh1106::{prelude::*, Builder};
use crate::hal::{pac, prelude::*};
use rotary_encoder_embedded::{RotaryEncoder, Direction};
use rotary_encoder_embedded::standard::StandardMode;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let (Some(dp), Some(cp)) = (pac::Peripherals::take(), cortex_m::peripheral::Peripherals::take()) else {
        panic!("Cannot take peripherals");
    };

    /* Clocks */
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(48.MHz()).freeze();

    /* GPIOs */
    let gpiob = dp.GPIOB.split();
    let scl = gpiob.pb8.into_alternate().internal_pull_down(true).set_open_drain();
    let sda = gpiob.pb9.into_alternate().internal_pull_down(true).set_open_drain();

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
    let mut counter  = 0_i32;
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

fn update_rotary_encoder<DT, CLK>(enc: &mut RotaryEncoder<StandardMode, DT, CLK>, counter: &mut i32) where DT: InputPin, CLK: InputPin {
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
