use crate::options;

use enigo::{
    Button, Coordinate,
    Direction::{Click,Press,Release},
    Enigo, Mouse, Settings,
};
use screenshots::image::Rgba;
use screenshots::Screen;
use std::thread;
use std::time;

struct GameState {
    frames: u32,
}

impl GameState {
    fn new() -> GameState {
        GameState {
            frames: 0,
        }
    }

    fn next_frame(&mut self) {
        self.frames += 1;
    }
}

fn upgrade(enigo: &mut Enigo, y: u32) {
    let x = 2880;
    /*
    let unlocked_ref = Rgba::from([0xf5, 0xf5, 0xf5, 0xff]);
    let unlocked = image.get_pixel(x, y);
    if unlocked.eq(&unlocked_ref) {
        */
        enigo.move_mouse(x as i32, y as i32, Coordinate::Abs).unwrap();
        for _ in 0..1000 {
            enigo.button(Button::Left, Click).unwrap();
            thread::sleep(time::Duration::from_millis(3));
        }
    //}
}

fn scroll_to(enigo: &mut Enigo, y: u32) {
    let press_delay = 500;
    let refresh_delay = 1000;
    enigo.move_mouse(2965 as i32, y as i32, Coordinate::Abs).unwrap();
    enigo.button(Button::Left, Press).unwrap();
    thread::sleep(time::Duration::from_millis(press_delay));
    enigo.button(Button::Left, Release).unwrap();
    thread::sleep(time::Duration::from_millis(1));
    enigo.button(Button::Left, Click).unwrap();
    thread::sleep(time::Duration::from_millis(1));
    enigo.button(Button::Left, Click).unwrap();
    thread::sleep(time::Duration::from_millis(1));
    enigo.button(Button::Left, Click).unwrap();
    thread::sleep(time::Duration::from_millis(refresh_delay));
}

fn play_once(screen: &Screen, enigo: &mut Enigo, state: &mut GameState, _options: &options::CommandLineOptions) {
    let image = screen.capture().unwrap();
    let game_discriminator_ref = Rgba::from([0xfc, 0xca, 0x55, 0xff]);
    let game_discriminator = image.get_pixel(570, 500);
    if game_discriminator.eq(&game_discriminator_ref) {
        if false {
            let refresh_delay = 3000;
            enigo.move_mouse(2965 as i32, 1640 as i32, Coordinate::Abs).unwrap();
            enigo.button(Button::Left, Click).unwrap();
            thread::sleep(time::Duration::from_millis(refresh_delay));
            enigo.move_mouse(2965 as i32, 1240 as i32, Coordinate::Abs).unwrap();
            enigo.button(Button::Left, Click).unwrap();
            thread::sleep(time::Duration::from_millis(refresh_delay));
            enigo.move_mouse(2965 as i32, 1040 as i32, Coordinate::Abs).unwrap();
            enigo.button(Button::Left, Click).unwrap();
            thread::sleep(time::Duration::from_millis(refresh_delay));
            enigo.move_mouse(2965 as i32, 840 as i32, Coordinate::Abs).unwrap();
            enigo.button(Button::Left, Click).unwrap();
            thread::sleep(time::Duration::from_millis(refresh_delay));
            enigo.move_mouse(2965 as i32, 640 as i32, Coordinate::Abs).unwrap();
            enigo.button(Button::Left, Click).unwrap();
            thread::sleep(time::Duration::from_millis(refresh_delay));
        }
        // Game mode
        // Check if there's an upgrade worth clicking
        if state.frames % 500 == 0 {
            thread::sleep(time::Duration::from_millis(2000));
            // Scroll to the bottom
            scroll_to(enigo, 1640);
            // Sunny thingy (TODO: check real name)
            upgrade(enigo, 1250);
            // Godly finger (TODO: check real name)
            upgrade(enigo, 780);
            // Scroll up
            scroll_to(enigo, 1240);
            // God clicker (TODO: check real name)
            upgrade(enigo, 1100);
            // Pope clicker (TODO: check real name)
            upgrade(enigo, 640);
            // Scroll up
            scroll_to(enigo, 1040);
            // Emperor clicker (TODO: check real name)
            upgrade(enigo, 950);
            // Scroll up
            scroll_to(enigo, 840);
            // King clicker (TODO: check real name)
            upgrade(enigo, 1270);
            // President clicker
            upgrade(enigo, 810);
            // Scroll up
            scroll_to(enigo, 640);
            // Mr Clicker
            upgrade(enigo, 1120);
            // Cursor
            upgrade(enigo, 660);

            // Attempt ascension
            thread::sleep(time::Duration::from_millis(1000));
            // Press ascent (even if not present)
            enigo.move_mouse(2840 as i32, 1750 as i32, Coordinate::Abs).unwrap();
            enigo.button(Button::Left, Click).unwrap();
            thread::sleep(time::Duration::from_millis(1000));
            // Press yes (assuming ascent was present)
            enigo.move_mouse(1650 as i32, 1500 as i32, Coordinate::Abs).unwrap();
            enigo.button(Button::Left, Click).unwrap();
            thread::sleep(time::Duration::from_millis(1000));
        }

        // Click a bunch on the capybara
        enigo.move_mouse(1400 as i32, 1200 as i32, Coordinate::Abs).unwrap();
        for _ in 0..10 {
            enigo.button(Button::Left, Click).unwrap();
            thread::sleep(time::Duration::from_millis(1));
        }
    }
}

pub fn play_game(options: &options::CommandLineOptions) {
    let screen = Screen::all().unwrap()[options.screen];
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let mut state = GameState::new();

    loop {
        state.next_frame();
        play_once(&screen, &mut enigo, &mut state, &options);
    }
}
