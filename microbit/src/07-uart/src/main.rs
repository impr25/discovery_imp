#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rtt_init_print,rprintln};
use panic_rtt_target as _;
use core::fmt::Write;
use heapless::Vec;

#[cfg(feature = "v1")]
use microbit::{
    hal::prelude::*,
    hal::uart,
    hal::uart::{Baudrate, Parity},
};

#[cfg(feature = "v2")]
use microbit::{
    hal::prelude::*,
    hal::uarte,
    hal::uarte::{Baudrate, Parity},
};

#[cfg(feature = "v2")]
mod serial_setup;
#[cfg(feature = "v2")]
use serial_setup::UartePort;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();

    #[cfg(feature = "v1")]
    let mut serial = {
        uart::Uart::new(
            board.UART0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        )
    };


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

    // nb::block!(serial.write(b'X')).unwrap();
    // nb::block!(serial.flush()).unwrap();

    // let sen: &str = "The quick brown fox jumps over the lazy dog \n";
    // for char in sen.as_bytes(){
    //     nb::block!(serial.write(*char)).unwrap();
    // }
    // nb::block!(serial.flush()).unwrap();

    let mut buffer: Vec<u8,32> = Vec::new();

    // My solution
    // loop {
    //     // let byte: u8 = nb::block!(serial.read()).unwrap();
        
    //     // rprintln!("{}",byte);
    //     // nb::block!(serial.write(byte)).unwrap();
    //     // nb::block!(serial.flush()).unwrap();

    //     let byte: u8 = nb::block!(serial.read()).unwrap();
    //     // rprintln!("{}",byte);
    //     if byte == 13u8 {
    //         buffer.reverse();
    //         for char in &buffer {
    //             nb::block!(serial.write(*char)).unwrap();
    //             nb::block!(serial.flush()).unwrap();
    //         }
    //         buffer.clear();
    //     }
    //     else {
    //         buffer.push(byte);
    //     }
    // }

    loop {
        buffer.clear();

        loop {
            let byte: u8 = nb::block!(serial.read()).unwrap();
            if buffer.push(byte).is_err() {
                write!(serial,"Buffer is full").unwrap();
                break;
            }

            if byte == 13 {
                for char in buffer.iter().rev().chain(&[b'\n', b'\r']) {
                    nb::block!(serial.write(*char)).unwrap();
                }
                break;
            } 
        }
    }
}
