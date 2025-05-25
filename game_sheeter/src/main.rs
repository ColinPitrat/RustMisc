use argh::FromArgs;
use enigo::{
    Button, Coordinate,
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Mouse, Settings,
};
use screenshots::image::Rgba;
use screenshots::Screen;
use std::thread;
use std::time;

#[derive(Clone, Default, FromArgs)]
/// A clicker for Storm the House (on crazygames.com)
pub struct CommandLineOptions {
    /// which screen to catpure from
    #[argh(option, default="0")]
    pub screen: usize,

    /// verbose output
    #[argh(switch, short='v')]
    pub verbose: bool,
}

struct Region {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    frames: i32,
}

impl Region {
    fn around(x: i32, y: i32, radius: i32, frames: i32) -> Region {
        Region{
            x1: x-radius,
            y1: y-radius,
            x2: x+radius,
            y2: y+radius,
            frames,
        }
    }

    fn rect(x: i32, y: i32, width: i32, height: i32, frames: i32) -> Region {
        Region{
            x1: x,
            y1: y,
            x2: x+width,
            y2: y+height,
            frames,
        }
    }

    fn contains(&self, x: i32, y: i32) -> bool {
        self.x1 < x && self.x2 > x && self.y1 < y && self.y2 > y
    }

    fn decrement(&mut self) {
        self.frames -= 1
    }

    fn finished(&self) -> bool {
        self.frames <= 0
    }
}

struct GameState {
    day: u32,
    frame: u32,
    avoid: Vec<Region>,
}

impl GameState {
    fn new() -> GameState {
        GameState{
            day: 0,
            frame: 0,
            avoid: vec!(),
        }
    }

    fn avoid_region(&mut self, region: Region) {
        self.avoid.push(region);
    }

    fn avoid_duration(&self) -> i32 {
        match self.day {
            0..5 => 10,
            5..10 => 7,
            10..20 => 5,
            _ => 3,
        }
    }

    fn avoid_radius(&self) -> i32 {
        match self.day {
            0..5 => 100,
            5..10 => 75,
            10..20 => 50,
            _ => 25,
        }
    }

    fn decrement_avoid(&mut self) {
        for r in self.avoid.iter_mut() {
            r.decrement();
        }

        self.avoid.retain(|x| !x.finished());
    }

    fn next_day(&mut self) {
        self.day += 1;
        self.avoid.clear();
        println!("Day {}", self.day);
    }

    fn next_frame(&mut self) {
        self.frame += 1;
    }
}

fn play_once(screen: &Screen, enigo: &mut Enigo, state: &mut GameState, options: &CommandLineOptions) {
    state.next_frame();

    let shop_discriminator_ref = Rgba::from([0xcb, 0xcb, 0xcb, 0xff]);
    // Sky color changes, so this doesn't work ...
    //let game_discriminator = Rgba::from([0x9d, 0xbb, 0xe4, 0xff]);

    let image = screen.capture().unwrap();
    {
        let shop_discriminator = image.get_pixel(1450, 656);
        if options.verbose {
            println!("Checking shop discriminator: {shop_discriminator:?}");
        }
        if shop_discriminator.eq(&shop_discriminator_ref) {
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
        }
    }

    {
        // TODO: Use something else for game discriminator (green pixel in life bar?)
        // TODO: Use a pixel more to the left for reload discriminator
        // TODO: Auto buy: click on fortify, then sniper, then upgrade wall and then clip size
        // TODO: Reduce region around enemy that has been shot and reduce duration
        let game_discriminator_ref = Rgba::from([0x50, 0xd3, 0x45, 0xff]);
        let game_discriminator = image.get_pixel(1830, 470);
        if options.verbose {
            println!("Checking game discriminator: {game_discriminator:?}");
        }
        if game_discriminator.eq(&game_discriminator_ref) {
            if options.verbose {
                println!("Game mode!");
            }
            // Game mode
            // If near the end of the round, avoid shooting in the "hire gunman" area as this may
            // lead to buying a gunman if the switch to the shop occurs just at that time.
            /*
             * Doesn't work, not sure why but there's a better way
            let day_end_discriminator = image.get_pixel(750, 525);
            if day_end_discriminator.0[3] <= 69 {
                println!("Frame {} - Day end protection triggers: {day_end_discriminator:?}", state.frame);
                state.avoid_region(Region::rect(850, 1180, 190, 320, 10))
            }
            */
            let black = Rgba::from([0, 0, 0, 0xff]);
            // We divide everything by 2 because somehow the area seen in 1920x1200 despite the resolution
            // being 3840x2400.
            let warzone = screen.capture_area(750/2, 1200/2, 1650/2, 620/2).unwrap();
            let shop_warzone_discriminator = warzone.get_pixel(1440, 1255);
            let shop_warsone_discriminator_ref = Rgba::from([0xe7, 0x13, 0x0d, 0xff]);
            if shop_warzone_discriminator.eq(&shop_warsone_discriminator_ref) {
                // We switched to the shop since previous screenshot, bail out!
                return
            }
            'outer: for (x, y, pixel) in warzone.enumerate_pixels() {
                for r in state.avoid.iter() {
                    if r.contains(x as i32, y as i32) {
                        continue 'outer;
                    }
                }
                if pixel.eq(&black) {
                    println!("Frame {} - Shooting at {},{}", state.frame, 750+x, 1200+y);
                    // Shoot
                    enigo.move_mouse(750+x as i32, 1200+y as i32, Coordinate::Abs).unwrap();
                    enigo.button(Button::Left, Click).unwrap();
                    state.avoid_region(Region::around(x as i32, y as i32, state.avoid_radius(), state.avoid_duration()));
                }
            }

            // Reload
            let reload_discriminator_ref = Rgba::from([0xcb, 0xd1, 0x47, 0xff]);
            let reload_discriminator = image.get_pixel(849, 470);
            if !reload_discriminator.eq(&reload_discriminator_ref) {
                println!("Recharging: {reload_discriminator:?} != {reload_discriminator_ref:?}");
                enigo.key(Key::Space, Press).unwrap();
                enigo.key(Key::Space, Release).unwrap();
            } else {
                if options.verbose {
                    println!("Not recharging: {reload_discriminator:?} == {reload_discriminator_ref:?}");
                }
            }
        }
    }

    state.decrement_avoid();

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

fn main() {
    let options : CommandLineOptions = argh::from_env();
    let screen = Screen::all().unwrap()[options.screen];
    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    let mut state = GameState::new();
    state.next_day();
    loop {
        play_once(&screen, &mut enigo, &mut state, &options);
    }
}
