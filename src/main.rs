#![no_main]
#![no_std]

use cortex_m::asm::wfi;
use cortex_m_rt::entry;
use microbit::{
    board::{
        Board,
    },
    display::blocking::Display,
    hal::Timer,
};
use embedded_hal::delay::DelayNs;
use panic_rtt_target as _;
use rtt_target::{rtt_init_print, rprintln};

use microbit2_demolib::{
    button::{
        ButtonStatus
    },
    font5x5::{
        FontGlyph, Font
    },
};


fn mprint(board: Board, string: &str) {

    let chars = string.as_bytes();
    let chars_length = chars.len();

    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let font = Font::new();
    let mut left_button = ButtonStatus::new(board.buttons.button_a.degrade());
    let mut right_button = ButtonStatus::new(board.buttons.button_b.degrade());

    let mut idx = 0usize;

    loop {
        left_button.update();
        right_button.update();

        if left_button.is_pressed {
            if idx > 0 { idx -= 1; }
        } else if right_button.is_pressed {
            if idx < chars_length - 1 { idx += 1; }
        }

        let ch = chars[idx];

        let glyph: &FontGlyph = font.getglyph(ch as char);

        display.show(&mut timer, *glyph, 100);
    }
}

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();

    mprint(board, "Hello, world!");

    loop {
        wfi();
    }
}


