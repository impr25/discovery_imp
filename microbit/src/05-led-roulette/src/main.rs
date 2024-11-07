#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;
// use panic_halt as _;
use microbit::{
    board::Board,
    hal::{prelude::*, Timer},
    display::blocking::Display,
};

const LIGHTUP: [(usize,usize);16] = [
    (0,0),(1,0),(2,0),(3,0),(4,0),(4,1),(4,2),(4,3),(4,4),(3,4),(2,4),(1,4),(0,4),(0,3),(0,2),(0,1)
];

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();

    let mut timer = Timer::new(board.TIMER0);

    let mut display = Display::new(board.display_pins);

    let mut led = [
        [0,0,0,0,0],
        [0,0,0,0,0],
        [0,0,0,0,0],
        [0,0,0,0,0],
        [0,0,0,0,0],
    ];

    let mut last_led = (0,0);
    // board.display_pins.col1.set_low().unwrap();
    // let mut row1 = board.display_pins.row1;

    // board.display_pins.col1.set_low().unwrap();
    // board.display_pins.row1.set_high().unwrap();
    // board.display_pins.row2.set_high().unwrap();
    // board.display_pins.row5.set_high().unwrap();
    // infinite loop; just so we don't leave this stack frame
    loop {
        // row1.set_low().unwrap();
        // rprintln!("Dark");
        // timer.delay_ms(1_000_u16);
        // row1.set_high().unwrap();
        // rprintln!("Light");
        // timer.delay_ms(1_000_u16);

        // display.show(&mut timer, ligh_it_all, 1000);

        // display.clear();
        // timer.delay_ms(1000_u16);
        for curr_led in LIGHTUP.iter() {
            led[last_led.0][last_led.1] = 0;
            led[curr_led.0][curr_led.1] = 1;
            display.show(&mut timer, led, 50);
            last_led = *curr_led;
        } 
    }
}