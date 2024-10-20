use microbit::{
    board::{
        Board,
    },
    hal::{
        gpio::{
            Input, Pin,
            // p0::{ P0_14, P0_23 }
        }
    }
};
use embedded_hal::digital::InputPin;
use nrf52833_hal::gpio::Floating;


// type ButtonInput = Input<Floating>;

// type ButtonA = P0_14<ButtonInput>;
// type ButtonB = P0_23<ButtonInput>;

type ButtonPin = Pin<Input<Floating>>;

pub struct ButtonStatus {
    pin: ButtonPin,
    pub is_pressing: bool,
    pub is_pressed: bool,
    pub is_released: bool,
}

impl ButtonStatus {
    pub fn new(pin: ButtonPin) -> Self {
        ButtonStatus {
            pin,
            is_pressing: false,
            is_pressed: false, is_released: false
        }
    }

    pub fn update(&mut self) {
        let is_pressing = self.pin.is_low().expect("failed to get button status");

        self.is_pressed = false;
        self.is_released = false;

        if self.is_pressing != is_pressing {
            match is_pressing {
                true => self.is_pressed = true,
                false => self.is_released = true,
            }
        }

        self.is_pressing = is_pressing;
    }
}

// pub type ButtonsStatus = [ButtonStatus; 2];

// fn create_button_status(pin: ButtonPin) -> ButtonStatus {
//     ButtonStatus {
//         pin,
//         is_pressing: false,
//         is_pressed: false, is_released: false,
//     }
// }

// pub fn get_buttons_status(board: &Board) -> ButtonsStatus {
//     let buttons = &board.buttons;

//     [
//         create_button_status(buttons.button_a.degrade()),
//         create_button_status(buttons.button_b.degrade()),
//     ]
// }

// pub fn update_buttons_status(buttons: &mut ButtonsStatus) {
//     buttons[0].update();
//     buttons[1].update();
// }

