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
        let image = screen.capture().unwrap();
        /* This is actually the '1st tab' discriminator
        let game_discriminator_ref = Rgba::from([0xcf, 0xc7, 0xbf, 0xff]);
        let game_discriminator = image.get_pixel(900, 500);
        if options.verbose {
            println!("Game discriminator: {game_discriminator:?} vs. {game_discriminator_ref:?}");
        }
        */
        let game_discriminator_ref = Rgba::from([0x4a, 0x9a, 0xbd, 0xff]);
        let game_discriminator = image.get_pixel(630, 435);
        if options.verbose {
            println!("Game discriminator: {game_discriminator:?} vs. {game_discriminator_ref:?}");
        }
        if game_discriminator.eq(&game_discriminator_ref) {
            let boss_discriminator_ref = Rgba::from([0xe7, 0x42, 0x18, 0xff]);
            let boss_discriminator = image.get_pixel(1880, 610);
            if boss_discriminator.eq(&boss_discriminator_ref) {
                // Ensure we're in the "go to next level" mode, in case we failed previously.
                let next_level_discriminator = image.get_pixel(1720, 1720);
                if next_level_discriminator.0[0] != 0xff {
                    thread::sleep(time::Duration::from_millis(100));
                    enigo.move_mouse(1720, 1760, Coordinate::Abs).unwrap();
                    enigo.button(Button::Left, Click).unwrap();
                }
                // Focus on the boss much more, assuming it's in the center
                for _ in 0..20 {
                    for x in (2000..2400).step_by(20) { // 400 / 20 = 20
                        for y in (1050..1300).step_by(30) {  // 250 / 20 = 12
                            enigo.move_mouse(x, y, Coordinate::Abs).unwrap();
                            enigo.button(Button::Left, Click).unwrap();
                            thread::sleep(time::Duration::from_millis(5));
                        }
                    }
                }
            } else {
                // Shoot whatever is there
                //for x in (1750..2750).step_by(50) { // 1000 / 50 = 20
                //    for y in (850..1550).step_by(50) {  // 700 / 50 = 14
                for _ in 0..5 {
                    for x in (1950..2550).step_by(30) { // 600 / 30 = 20
                        for y in (950..1450).step_by(30) {  // 500 / 30 = 16
                            enigo.move_mouse(x, y, Coordinate::Abs).unwrap();
                            enigo.button(Button::Left, Click).unwrap();
                            thread::sleep(time::Duration::from_millis(10));
                        }
                    }
                }
            }
            // Give time to the game/browser/OS to recover
            thread::sleep(time::Duration::from_millis(1000));
            // Click on the bonus zone
            for x in (2600..2900).step_by(10) { // 300 / 10 = 30
                for y in (700..780).step_by(25) {  // 80 / 25 = 3
                    enigo.move_mouse(x, y, Coordinate::Abs).unwrap();
                    enigo.button(Button::Left, Click).unwrap();
                    thread::sleep(time::Duration::from_millis(5));
                }
            }
            // Give time to the game/browser/OS to recover
            thread::sleep(time::Duration::from_millis(1000));
            // Click on all the upgrades
            let x = 1300;
            for y in (490..1500).step_by(155) {
                for _ in 0..3 {
                    thread::sleep(time::Duration::from_millis(100));
                    // We need a new image for each upgrade in case the previous one reached the max.
                    let image = screen.capture().unwrap();
                    let gold_discriminator = image.get_pixel(x, y);
                    if gold_discriminator.0[0] == 0xff {
                        enigo.move_mouse(x as i32, y as i32, Coordinate::Abs).unwrap();
                        enigo.button(Button::Left, Click).unwrap();
                    }
                }
            }
            // Give time to the game/browser/OS to recover
            thread::sleep(time::Duration::from_millis(1000));
            if false {
                break
            }
        }
    }

    if false {
        loop {
            state.next_frame();
            play_once(&screen, &mut enigo, &mut state, &options);
        }
    }
}
