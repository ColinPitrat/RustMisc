use crate::options;

use enigo::{
    Button, Coordinate,
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Mouse, Settings,
};
use regex::Regex;
use screenshots::image::Rgba;
use screenshots::Screen;
use std::thread;
use std::time;

struct GameState {
    frame: u32,
    last_frame: time::SystemTime,
}

impl GameState {
    fn new() -> GameState {
        GameState{
            frame: 0,
            last_frame: time::SystemTime::now(),
        }
    }

    fn next_frame(&mut self) {
        self.frame += 1;
        let duration = time::SystemTime::now().duration_since(self.last_frame).unwrap();
        let fps = if duration.as_millis() > 0 {
            1000/duration.as_millis()
        } else {
            0
        };
        println!("New frame after {:?} ({} FPS)", duration, fps);
        self.last_frame = time::SystemTime::now();
    }
}

fn play_once(screen: &Screen, enigo: &mut Enigo, state: &mut GameState, options: &options::CommandLineOptions) {
    state.next_frame();

    let image = screen.capture().unwrap();
    image.save(format!("screenshot_{}", state.frame)).unwrap();
    // Check 2 pixels because:
    //  - white is not a very good discriminator
    //  - the other pixel is not specific to the dino game
    let game_discriminator_ref = Rgba::from([0xff, 0xff, 0xff, 0xff]);
    let game_discriminator = image.get_pixel(500, 500);
    let game_discriminator_ref2 = Rgba::from([0x0c, 0x0d, 0x14, 0xff]);
    let game_discriminator2 = image.get_pixel(470, 410);

    if options.verbose {
        println!("Checking game discriminator: {game_discriminator:?} - {game_discriminator2:?}");
    }
    if game_discriminator.eq(&game_discriminator_ref) && game_discriminator2.eq(&game_discriminator_ref2) {
        let white = Rgba::from([0xff, 0xff, 0xff, 0xff]);
        let min_x = 750;
        let max_x = u32::min(3070,1300+(10*state.frame)/40);
        'outer: for y in [1230,1260].into_iter() {
            for x in (min_x..max_x).step_by(20) {
                let c = image.get_pixel(x, y);
                if c.ne(&white) {
                    println!("Obstacle detected at {x},{y}");
                    enigo.key(Key::Space, Press).unwrap();
                    thread::sleep(time::Duration::from_millis(1));
                    enigo.key(Key::Space, Release).unwrap();
                    thread::sleep(time::Duration::from_millis(1));
                    break 'outer;
                }
            }
        }
        /*
        // Shop mode
        println!("Shop mode!");
        // Give some time for player to react if they want to buy something
        thread::sleep(time::Duration::from_secs(5));
        // Repair if needed (shouldn't be normally)
        enigo.move_mouse(1250 as i32, 750 as i32, Coordinate::Abs).unwrap();
        for _ in 0..10 {
        enigo.button(Button::Left, Click).unwrap();
        thread::sleep(time::Duration::from_millis(10));
        }
        // Sniper has priority
        enigo.move_mouse(1800 as i32, 750 as i32, Coordinate::Abs).unwrap();
        enigo.button(Button::Left, Click).unwrap();
        thread::sleep(time::Duration::from_millis(10));
        // 10k on clips if possible (shifts fortifying to later in the game)
        enigo.move_mouse(950 as i32, 750 as i32, Coordinate::Abs).unwrap();
        for _ in 0..10 {
        enigo.button(Button::Left, Click).unwrap();
        thread::sleep(time::Duration::from_millis(1));
        }
        // Fortify
        enigo.move_mouse(2100 as i32, 750 as i32, Coordinate::Abs).unwrap();
        enigo.button(Button::Left, Click).unwrap();
        thread::sleep(time::Duration::from_millis(10));
        // Wall
        enigo.move_mouse(1500 as i32, 750 as i32, Coordinate::Abs).unwrap();
        enigo.button(Button::Left, Click).unwrap();
        thread::sleep(time::Duration::from_millis(10));
        // Clips
        enigo.move_mouse(950 as i32, 750 as i32, Coordinate::Abs).unwrap();
        for _ in 0..1000 {
        enigo.button(Button::Left, Click).unwrap();
        thread::sleep(time::Duration::from_millis(1));
        }
        // Give some time for player to react
        thread::sleep(time::Duration::from_secs(10));
        // Done
        enigo.move_mouse(1700 as i32, 1750 as i32, Coordinate::Abs).unwrap();
        enigo.button(Button::Left, Click).unwrap();

        // Next day is starting.
        state.next_day();

        // Avoid reentering shop mode immediately
        thread::sleep(time::Duration::from_secs(1));
        return
        */
    }

    // Examples
    //enigo.key(Key::Control, Press);
    //enigo.key(Key::Unicode('v'), Click);
    //enigo.key(Key::Control, Release);
    //enigo.move_mouse(500, 200, Coordinate::Abs);
    //enigo.button(Button::Left, Press);
    //enigo.move_mouse(100, 100, Coordinate::Rel);
    //enigo.button(Button::Left, Release);
    //enigo.text("hello world");
}

pub fn play_game(options: &options::CommandLineOptions) {
    let re = Regex::new(r"^screenshot.*\.png$").unwrap();
    for file in fs::read_dir(".").unwrap() {
        if re.is_match(file) {
            std::fs::remove_file(file).unwrap();
        }
    }

    let screen = Screen::all().unwrap()[options.screen];
    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    let mut state = GameState::new();
    loop {
        play_once(&screen, &mut enigo, &mut state, &options);
    }
}
