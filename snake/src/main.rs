extern crate clap;
extern crate rand;
extern crate sdl2;

use clap::{Arg,App};
use itertools::Itertools;
use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mixer::{InitFlag, AUDIO_S16LSB, DEFAULT_CHANNELS, MAX_VOLUME};
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use serde::{Serialize,Deserialize};
use std::cmp::Ordering;
use std::collections::LinkedList;
use std::error::Error;
use std::fs;
use std::path::{Path,PathBuf};
use std::str::FromStr;
use std::thread::sleep;
use std::time::{Duration, SystemTime};
use std::vec::Vec;

const CELL_SIZE : i32 = 20;
const GRID_WIDTH : i32 = 40;
const GRID_HEIGHT : i32 = 40;
const SCREEN_WIDTH : i32 = CELL_SIZE * GRID_WIDTH;
const SCREEN_HEIGHT : i32 = CELL_SIZE * GRID_HEIGHT;

// Provide Serialize/Deserialize for sdl2::pixels::Color
// cf. https://serde.rs/remote-derive.html
#[derive(Serialize, Deserialize)]
#[serde(remote = "Color")]
struct ColorDef {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive (PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive (Clone,Copy,PartialEq,Serialize,Deserialize)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    #[allow(dead_code)]
    fn new(x: i32, y: i32) -> Position {
        Position {x, y}
    }
}

struct Snake {
    segments: LinkedList<Position>,
    dir: Direction,
    last_move: SystemTime,
    // TODO: score shouldn't be at the snake level (the snake can be reset between each level, we
    // want to keep the score)
    score: i32,
}

impl Snake {
    fn new(start_pos: Position) -> Snake {
        let mut segments = LinkedList::new();
        segments.push_back(start_pos);
        Snake {
            segments: segments,
            dir: Direction::Right,
            last_move: SystemTime::now(),
            score: 0,
        }
    }
}

struct Target {
    pos: Position,
    color: Color,
    points: i32,
}

impl Target {
    // TODO: Prevent targets from appearing on obstacles & snake
    fn new() -> Target {
        let green = Color::RGB(0, 255, 0);
        Target{
            pos: Position {
                x: rand::thread_rng().gen_range(0, GRID_WIDTH),
                y: rand::thread_rng().gen_range(0, GRID_HEIGHT),
            },
            color: green,
            points: 1,
        }
    }
}

#[derive(Clone,Copy,Serialize, Deserialize)]
struct Obstacle {
    pos: Position,
    #[serde(with = "ColorDef")]
    color: Color,
}

#[derive(Clone,Serialize, Deserialize)]
struct Level {
    name: String,
    start_pos: Position,
    obstacles: Vec<Obstacle>,
}

impl Level {
    fn load_levels() -> Vec<Level> {
        std::fs::read_dir("resources/levels/").unwrap()
            .map(|res| res.map(|e| Level::load(&e.path())).unwrap())
            .sorted()
            .collect::<Vec<_>>()
    }

    fn load(filename: &Path) -> Level {
        serde_json::from_str(&fs::read_to_string(filename).expect(&format!("Unable to read file {:?}.", filename))).unwrap()
    }

    #[allow(dead_code)]
    fn save(&self, filename: &Path) {
        fs::write(&filename, serde_json::to_string_pretty(&self).unwrap()).expect(&format!("Unable to write file {:?}.", filename));
        println!("Saved level {:?}", filename);
    }
}

impl Ord for Level {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl PartialOrd for Level {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.name.cmp(&other.name))
    }
}

impl PartialEq for Level {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Level {
}

struct Game {
    levels: Vec<Level>,
    level_idx: usize,
    snake: Snake,
    target: Target,
}

impl Game {
    fn new(start_level: usize) -> Game {
        // TODO: This is dirty: the game is created in an invalid state because the snake is not in
        // the right position. We have to call reset on it.
        let mut g = Game {
            levels: Level::load_levels(),
            level_idx: start_level,
            snake: Snake::new(Position::new(0,0)),
            target: Target::new(),
        };
        g.reset_level();
        g
    }

    fn level(&self) -> &Level {
        &self.levels[self.level_idx]
    }

    fn new_target(&mut self) {
        loop {
            self.target = Target::new();
            if self.snake.segments.iter().all(|&s| s != self.target.pos) &&
                self.level().obstacles.iter().all(|&o| o.pos != self.target.pos) {
                    break;
                }
        }
    }

    fn reset_level(&mut self) {
        self.snake = Snake::new(self.level().start_pos);
        self.new_target();
    }

    fn next_level(&mut self) {
        if self.level_idx < self.levels.len()-1 {
            self.level_idx += 1;
            self.reset_level();
        }
    }

    fn prev_level(&mut self) {
        if self.level_idx > 0 {
            self.level_idx -= 1;
            self.reset_level();
        }
    }
}

fn black_background(canvas: &mut Canvas<Window>) {
    let black = Color::RGB(0, 0, 0);
    canvas.set_draw_color(black);
    canvas.clear();
}

fn list_musics() -> Vec<PathBuf> {
    std::fs::read_dir("resources/musics/").unwrap()
        .map(|res| res.map(|e| e.path()).unwrap())
        .collect::<Vec<_>>()
}

// TODO: Move all the music logic into a "Jukebox" type
fn play_current_music(musics: &Vec<PathBuf>, current_music: usize) -> Option<sdl2::mixer::Music> {
    if current_music < musics.len() {
        let path = musics[current_music].clone();
        println!("Playing {:?}", path);
        let music = sdl2::mixer::Music::from_file(path).unwrap();
        music.play(1).unwrap();
        Some(music)
    } else {
        None
    }
}

#[allow(dead_code)]
fn generate_levels() {
    let blue = Color::RGB(0, 0, 255);
    // Level 1 has no obstacle
    {
        let obstacles = vec!();
        let level1 = Level{name: "level1".to_string(), obstacles, start_pos: Position::new(0, 0)};
        level1.save(&PathBuf::from("resources/levels/level1"));
    }
    // Level 2 has borders
    {
        let mut obstacles = vec!();
        for x in 0..GRID_WIDTH {
            obstacles.push(Obstacle{pos: Position::new(x, 0), color: blue});
            obstacles.push(Obstacle{pos: Position::new(x, GRID_HEIGHT-1), color: blue});
        }
        for y in 0..GRID_HEIGHT {
            obstacles.push(Obstacle{pos: Position::new(0, y), color: blue});
            obstacles.push(Obstacle{pos: Position::new(GRID_WIDTH-1, y), color: blue});
        }
        let level2 = Level{name: "level2".to_string(), obstacles, start_pos: Position::new(1, 1)};
        level2.save(&PathBuf::from("resources/levels/level2"));
    }
    // Level 3 has a small cross in the middle
    {
        let mut obstacles = vec!();
        for x in 10..GRID_WIDTH-10 {
            obstacles.push(Obstacle{pos: Position::new(x, (GRID_HEIGHT-1)/2), color: blue});
            obstacles.push(Obstacle{pos: Position::new(x, (GRID_HEIGHT+1)/2), color: blue});
        }
        for y in 10..GRID_HEIGHT-10 {
            obstacles.push(Obstacle{pos: Position::new((GRID_WIDTH-1)/2, y), color: blue});
            obstacles.push(Obstacle{pos: Position::new((GRID_WIDTH+1)/2, y), color: blue});
        }
        let level3 = Level{name: "level3".to_string(), obstacles, start_pos: Position::new(0, 0)};
        level3.save(&PathBuf::from("resources/levels/level3"));
    }
    // Level 4 has a large cross in the middle forcing to use borders to change sides
    {
        let mut obstacles = vec!();
        for x in 0..GRID_WIDTH {
            obstacles.push(Obstacle{pos: Position::new(x, (GRID_HEIGHT-1)/2), color: blue});
            obstacles.push(Obstacle{pos: Position::new(x, (GRID_HEIGHT+1)/2), color: blue});
        }
        for y in 0..GRID_HEIGHT {
            obstacles.push(Obstacle{pos: Position::new((GRID_WIDTH-1)/2, y), color: blue});
            obstacles.push(Obstacle{pos: Position::new((GRID_WIDTH+1)/2, y), color: blue});
        }
        let level4 = Level{name: "level4".to_string(), obstacles, start_pos: Position::new(0, 0)};
        level4.save(&PathBuf::from("resources/levels/level4"));
    }
    // Level 5 is a spiral
    {
        let passage_width = 5;
        let wall_width = 1;
        let width = passage_width + wall_width;
        let mut length = width;
        let mut obstacles = vec!();
        let mut x = (GRID_WIDTH-1)/2;
        let mut y = (GRID_HEIGHT-width)/2;
        let mut next_x = x;
        let mut next_y = y + width;
        let mut direction = 1;
        while length < std::cmp::min(GRID_WIDTH, GRID_HEIGHT) {
            while x != next_x || y != next_y {
                for i in 0..wall_width {
                    for j in 0..wall_width {
                        obstacles.push(Obstacle{pos: Position::new(x+i, y+j), color: blue});
                    }
                }
                if x < next_x {
                    x += wall_width;
                }
                if x > next_x {
                    x -= wall_width;
                }
                if y < next_y {
                    y += wall_width;
                }
                if y > next_y {
                    y -= wall_width;
                }
            }
            match direction % 4 {
                0 => {
                    next_y += length;
                }
                1 => {
                    next_x += length;
                    length += width;
                }
                2 => {
                    next_y -= length;
                }
                3 => {
                    next_x -= length;
                    length += width;
                }
                x => {
                    panic!("Unexpected modulo result: {}", x);
                }
            }
            direction += 1;
        }
        let level5 = Level{name: "level5".to_string(), obstacles, start_pos: Position::new(0, 0)};
        level5.save(&PathBuf::from("resources/levels/level5"));
    }
}

fn game(start_level: usize) -> Result<(), Box<dyn Error>> {
    let sdl_context = sdl2::init().unwrap();

    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("Snake", SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    let _audio_subsystem = sdl_context.audio().unwrap();
    let frequency = 44100;
    let format = AUDIO_S16LSB;
    let channels = DEFAULT_CHANNELS;
    let chunk_size = 1024;
    sdl2::mixer::open_audio(frequency, format, channels, chunk_size).unwrap();
    let _mixer_context = sdl2::mixer::init(InitFlag::MP3 | InitFlag::FLAC | InitFlag::MOD | InitFlag::OGG).unwrap();
    // 4 channels should be more than enough for everybody
    sdl2::mixer::allocate_channels(4);
    let mut target_eaten_sound = sdl2::mixer::Chunk::from_file("resources/sounds/eaten.wav").unwrap();
    target_eaten_sound.set_volume(MAX_VOLUME);
    let musics = list_musics();
    let mut current_music = 0;
    let mut music = play_current_music(&musics, current_music);
    // TODO: Make music & sound effects volumes configurable
    sdl2::mixer::Music::set_volume(2*MAX_VOLUME/10);

    let mut game = Game::new(start_level);

    let mut event_pump = sdl_context.event_pump().unwrap();
    'game_loop: loop {
        if let Some(_) = music {
            if !sdl2::mixer::Music::is_playing() {
                current_music = (current_music + 1) % musics.len();
                music = play_current_music(&musics, current_music);
            }
        }
        // Forcing move after direction change prevents bug leading to death when 2 direction
        // change are done very quickly (e.g left then down while going up).
        // We also break the loop to be sure we force the move (otherwise, pressing 2 keys at the
        // same time can still lead to reversing direction before having moved).
        let mut force_move = false;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'game_loop
                },
                Event::KeyDown { keycode: Some(Keycode::KpPlus), .. } => {
                    game.next_level()
                },
                Event::KeyDown { keycode: Some(Keycode::KpMinus), .. } => {
                    game.prev_level()
                },
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    if game.snake.dir != Direction::Down {
                        game.snake.dir = Direction::Up;
                        force_move = true;
                        break;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    if game.snake.dir != Direction::Up {
                        game.snake.dir = Direction::Down;
                        force_move = true;
                        break;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    if game.snake.dir != Direction::Right {
                        game.snake.dir = Direction::Left;
                        force_move = true;
                        break;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    if game.snake.dir != Direction::Left {
                        game.snake.dir = Direction::Right;
                        force_move = true;
                        break;
                    }
                },
                _ => {}
            }
        }

        match SystemTime::now().duration_since(game.snake.last_move) {
            Ok(n) => {
                if n.as_millis() > 100 || force_move {
                    game.snake.last_move = SystemTime::now();
                    let mut new_head = game.snake.segments.back().unwrap().clone();
                    match game.snake.dir {
                        Direction::Up => new_head.y -= 1,
                        Direction::Down => new_head.y += 1,
                        Direction::Left => new_head.x -= 1,
                        Direction::Right => new_head.x += 1,
                    }
                    new_head.x = (new_head.x + GRID_WIDTH) % GRID_WIDTH;
                    new_head.y = (new_head.y + GRID_HEIGHT) % GRID_HEIGHT;
                    if game.snake.segments.iter().any(|&s| s == new_head) ||
                        game.level().obstacles.iter().any(|&o| o.pos == new_head) {
                        println!("Game over! Score: {}", game.snake.score);
                        // TODO: Better game over
                        sleep(Duration::new(2, 0));
                        // TODO: Move the musics into a Jukebox type and then have a Jukebox be
                        // part of the Game.
                        music = play_current_music(&musics, current_music);
                        game.reset_level();
                    } else {
                        if new_head == game.target.pos {
                            game.snake.score += game.target.points;
                            sdl2::mixer::Channel::all().play(&target_eaten_sound, 0).unwrap();
                            game.new_target();
                        } else {
                            game.snake.segments.pop_front();
                        }
                        game.snake.segments.push_back(new_head);
                    }
                }
            },
            Err(e) => println!("Error calculating time since last move: {:?}", e),
        }

        black_background(&mut canvas);
        let red = Color::RGB(255, 0, 0);
        canvas.set_draw_color(red);
        game.snake.segments.iter().for_each(|&s| {
                canvas.fill_rect(sdl2::rect::Rect::new(s.x * CELL_SIZE, s.y * CELL_SIZE, CELL_SIZE as u32, CELL_SIZE as u32)).unwrap();
        });
        // TODO: Support changing level
        game.level().obstacles.iter().for_each(|&o| {
                canvas.set_draw_color(o.color);
                canvas.fill_rect(sdl2::rect::Rect::new(o.pos.x * CELL_SIZE, o.pos.y * CELL_SIZE, CELL_SIZE as u32, CELL_SIZE as u32)).unwrap();
        });
        canvas.set_draw_color(game.target.color);
        canvas.fill_rect(sdl2::rect::Rect::new(game.target.pos.x * CELL_SIZE, game.target.pos.y * CELL_SIZE, CELL_SIZE as u32, CELL_SIZE as u32)).unwrap();
        canvas.present();
    }
    Ok(())
}

fn is_type<T: FromStr>(val: String) -> Result<(), String>
where <T as std::str::FromStr>::Err : std::string::ToString
{
    match val.parse::<T>() {
        Ok(_) => Ok(()),
        Err(m) => Err(m.to_string()),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("Snake")
        .version("0.1")
        .author("Colin Pitrat")
        .about("Lead your snake so that it grows without eating itself.")
        .arg(Arg::with_name("generate_levels")
                .short("G")
                .long("generate_levels")
                .help("Regenerate the programmatically generated levels."))
        .arg(Arg::with_name("level")
                .short("l")
                .long("level")
                .value_name("NUMBER")
                .help("Starting level")
                .takes_value(true)
                .default_value("0")
                .validator(is_type::<usize>))
        .get_matches();
    if matches.is_present("generate_levels") {
        generate_levels();
    }
    // TODO: Validate that level is valid (0 < level < max_level) (or better, only allow level
    // selection in game)
    game(matches.value_of("level").unwrap().parse::<usize>().unwrap())
}
