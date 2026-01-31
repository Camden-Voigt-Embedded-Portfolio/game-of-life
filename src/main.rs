#![no_main]
#![no_std]

mod life;
use life::*;

use microbit::hal;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use cortex_m_rt::entry;
use microbit::board::Board;
use microbit::display::blocking::Display;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let _board = Board::take().unwrap();
    let mut display = Display::new(_board.display_pins);
    let mut timer = hal::Timer::new(_board.TIMER0);
    let mut life_grid = [
        [0, 1, 0, 0, 0],
        [1, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 1, 1, 0],
        [0, 1, 0, 0, 0],
    ];

    loop {
        display.show(&mut timer, life_grid, 1000);
        life(&mut life_grid);
        core::hint::spin_loop();
    }
}
