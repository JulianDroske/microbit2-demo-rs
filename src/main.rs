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
    gpio::{
        NUM_COLS, NUM_ROWS,
    },
};
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


struct GlyphTransition {
    is_forwarding: bool,
    offset: isize,
}

impl GlyphTransition {
    const WIDTH: usize = NUM_COLS;
    const HEIGHT: usize = NUM_ROWS;
    pub fn new(is_forwarding: bool) -> Self {
        GlyphTransition {
            is_forwarding,
            offset: if is_forwarding { GlyphTransition::WIDTH as isize - 1 } else { 0 }
        }
    }

    pub fn getglyph(&mut self, curr_glyph: FontGlyph, next_glyph: FontGlyph) -> (FontGlyph, bool) {
        let curr_pos: isize = self.offset;
        let curr_pos: usize = if curr_pos < 0 { 0 }
            else if curr_pos >= Self::WIDTH as isize { Self::WIDTH - 1 }
            else { curr_pos as usize };

        let is_forwarding = self.is_forwarding;

        let [ left_glyph, right_glyph ] = if is_forwarding { [ curr_glyph, next_glyph ] } else { [ next_glyph, curr_glyph ] };

        let mut new_glyph: FontGlyph = [[0; Self::WIDTH]; Self::HEIGHT];

        for row_pos in 0 .. Self::HEIGHT {
            for col_pos in 0 .. curr_pos {
                new_glyph[row_pos][col_pos] = left_glyph[row_pos][Self::WIDTH - curr_pos + col_pos];
            }
            for col_pos in curr_pos + 1 .. Self::WIDTH {
                new_glyph[row_pos][col_pos] = right_glyph[row_pos][col_pos - curr_pos];
            }
        }

        let is_done = if is_forwarding { curr_pos == 0 } else { curr_pos == Self::WIDTH - 1 };

        self.offset += if is_forwarding { -1 } else { 1 };

        return (
            new_glyph,
            is_done
        );
    }
}


fn mprint(board: Board, string: &str) {

    let chars = string.as_bytes();
    let chars_length = chars.len();

    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let font = Font::new();
    let mut left_button = ButtonStatus::new(board.buttons.button_a.degrade());
    let mut right_button = ButtonStatus::new(board.buttons.button_b.degrade());

    let mut idx = 0usize;

    let mut last_idx = 0usize;

    let mut transition = GlyphTransition::new(true);

    loop {
        left_button.update();
        right_button.update();

        if left_button.is_pressed {
            if idx > 0 {
                idx -= 1;
                transition = GlyphTransition::new(false);
            }
        } else if right_button.is_pressed {
            if idx < chars_length - 1 {
                idx += 1;
                transition = GlyphTransition::new(true);
            }
        }

        let mut glyph: FontGlyph = *font.getglyph(chars[idx] as char);

        if last_idx != idx {
            // show the tansition
            let last_glyph = *font.getglyph(chars[last_idx] as char);
            let (new_glyph, is_done) = transition.getglyph(last_glyph, glyph);
            glyph = new_glyph;

            if is_done {
                last_idx = idx;
            }
        }

        display.show(&mut timer, glyph, 100);
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


