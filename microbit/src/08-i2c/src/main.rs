// #![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;
use core::str;
use core::fmt::Write;
use heapless::Vec;

#[cfg(feature = "v1")]
use microbit::{
    hal::twi,
    pac::twi0::frequency::FREQUENCY_A,
};

#[cfg(feature = "v2")]
use microbit::{
    hal::twim,
    hal::prelude::*,
    hal::uarte,
    hal::uarte::{Baudrate, Parity},
    pac::twim0::frequency::FREQUENCY_A,
};

#[cfg(feature= "v2")]
mod serial_setup;
#[cfg(feature = "v2")]
use serial_setup::UartePort;

use lsm303agr::{
    MagOutputDataRate, AccelOutputDataRate, Lsm303agr,
};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();

    #[cfg(feature = "v2")]
    let mut serial = {
        let serial = uarte::Uarte::new(
            board.UARTE0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );
        UartePort::new(serial)
    };


    #[cfg(feature = "v1")]
    let i2c = { twi::Twi::new(board.TWI0, board.i2c.into(), FREQUENCY_A::K100) };

    #[cfg(feature = "v2")]
    let i2c = { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100) };

    // Code from documentation
    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor.set_accel_odr(AccelOutputDataRate::Hz10).unwrap();
    sensor.set_mag_odr(MagOutputDataRate::Hz10).unwrap();
    let mut sensor = sensor.into_mag_continuous().ok().unwrap();

    let mut buffer:Vec<u8,32> = Vec::new();
    let mut sensor_mode:u8 = 0;

    loop {
        buffer.clear();

        loop {

            let byte: u8 = nb::block!(serial.read()).unwrap();
            rprintln!("{}",byte as char);
            nb::block!(serial.write(byte)).unwrap();
            nb::block!(serial.flush()).unwrap();
            if buffer.push(byte).is_err() {
                write!(serial,"buffer is full").unwrap();
                break;
            }

            if byte == 13 {
                let comand:&str = str::from_utf8(&buffer).unwrap().trim();
                if comand == "magnetometer" {
                    sensor_mode = 1;
                    buffer.clear();
                }
                else if comand == "accelerometer" {
                    sensor_mode  = 2;
                    buffer.clear();
                }
                else {
                    rprintln!("{}",comand);
                    write!(serial,"\r\nwrong comand : {} \n\r",comand).unwrap();
                    nb::block!(serial.flush()).unwrap();
                    // sensor_mode = 0;
                    buffer.clear();
                }
            }
            
            if sensor_mode == 1 {
                if sensor.mag_status().unwrap().xyz_new_data {
                    let data = sensor.mag_data().unwrap();
                    // rprintln!("Mag : x {} y {} z {}", data.x, data.y, data.z);
                    write!(serial,"Mag reading x:{}, y:{}, z:{}.\r\n",data.x, data.y, data.z).unwrap();
                    // nb::block!(serial.flush()).unwrap();
                }
            }
            else if sensor_mode == 2 {
                if sensor.accel_status().unwrap().xyz_new_data {
                    let data = sensor.accel_data().unwrap();
                    // rprintln!("Acceleration: x {} y {} z {}", data.x, data.y, data.z);
                    write!(serial,"Accel reading x:{}, y:{}, z:{}.\r\n",data.x, data.y, data.z).unwrap();
                    // nb::block!(serial.flush()).unwrap();
                }
            }
            else {
                // rprintln!("{}",byte as char);
                // nb::block!(serial.write(byte)).unwrap();
                // nb::block!(serial.flush()).unwrap();
                continue;
            }
        }
    }
}

