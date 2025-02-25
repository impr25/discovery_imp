#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rtt_init_print,rprintln};
use panic_rtt_target as _;

use lsm303agr::{AccelOutputDataRate,Lsm303agr,AccelScale};
use microbit::{hal::twim, pac::twim0::frequency::FREQUENCY_A};

use microbit::hal::timer::Timer;
use microbit::hal::prelude::*;
use nb::Error;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();

    let i2c = { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100)};

    let mut countdown = Timer::new(board.TIMER0);
    let mut delay = Timer::new(board.TIMER1);

    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor.set_accel_odr(AccelOutputDataRate::Hz10).unwrap();
    sensor.set_accel_scale(AccelScale::G16).unwrap();

    let mut max_g = 0.;
    let mut measuring = false;

    loop {
        while !sensor.accel_status().unwrap().xyz_new_data {}
        let mut g_x = sensor.accel_data().unwrap().x as f32 / 1000.0;
        // rprintln!("Acceleration value is X:{}, Y:{}, Z:{}",data.x, data.y, data.z);

        if measuring {
            match countdown.wait() {
                Err(Error::WouldBlock) => {
                    if g_x >max_g {
                        max_g =g_x;
                    }
                },
                Ok(_) => { 
                    rprintln!("Max acceleration is {}", max_g);
                    max_g = 0.;
                    measuring = false;
                }
            }
        } 
        else {
            if g_x > 0.5 {
                rprintln!("START !");
                measuring =true;
                max_g=g_x;
                countdown.start(1_000_000_u32);
            }   
        }

        
    }
}
