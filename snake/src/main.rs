extern crate sdl2; 
extern crate rand;

use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mixer::{InitFlag, AUDIO_S16LSB, DEFAULT_CHANNELS, MAX_VOLUME};
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::collections::LinkedList;
use std::path::PathBuf;
use std::thread::sleep;
use std::time::{Duration, SystemTime};
use std::vec::Vec;

const CELL_SIZE : i32 = 20;
const GRID_WIDTH : i32 = 40;
const GRID_HEIGHT : i32 = 40;
const SCREEN_WIDTH : i32 = CELL_SIZE * GRID_WIDTH;
const SCREEN_HEIGHT : i32 = CELL_SIZE * GRID_HEIGHT;

#[derive (PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive (Clone,Copy,PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Position {
        Position {x, y}
    }
}

struct Snake {
    segments: LinkedList<Position>,
    dir: Direction,
    last_move: SystemTime,
    score: i32,
}

impl Snake {
    fn new() -> Snake {
        let mut segments = LinkedList::new();
        segments.push_back(Position::new(0, 0));
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

fn main() {
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

    let mut snake = Snake::new();
    let mut target = Target::new();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'game_loop: loop {
        if let Some(_) = music {
            if !sdl2::mixer::Music::is_playing() {
                current_music = (current_music + 1) % musics.len();
                music = play_current_music(&musics, current_music);
            }
        }
        // Forcing move after direction change prevents bug leading to death when 2 direction
        // change are done very quickly (e.g left then down while going up)
        let mut force_move = false;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'game_loop
                },
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    if snake.dir != Direction::Down {
                        snake.dir = Direction::Up;
                        force_move = true;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    if snake.dir != Direction::Up {
                        snake.dir = Direction::Down;
                        force_move = true;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    if snake.dir != Direction::Right {
                        snake.dir = Direction::Left;
                        force_move = true;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    if snake.dir != Direction::Left {
                        snake.dir = Direction::Right;
                        force_move = true;
                    }
                },
                _ => {}
            }
        }

        match SystemTime::now().duration_since(snake.last_move) {
            Ok(n) => {
                if n.as_millis() > 100 || force_move {
                    snake.last_move = SystemTime::now();
                    let mut new_head = snake.segments.back().unwrap().clone();
                    match snake.dir {
                        Direction::Up => new_head.y -= 1,
                        Direction::Down => new_head.y += 1,
                        Direction::Left => new_head.x -= 1,
                        Direction::Right => new_head.x += 1,
                    }
                    new_head.x = (new_head.x + GRID_WIDTH) % GRID_WIDTH;
                    new_head.y = (new_head.y + GRID_HEIGHT) % GRID_HEIGHT;
                    if snake.segments.iter().any(|&s| s == new_head) {
                        println!("Game over! Score: {}", snake.score);
                        // TODO: Better game over
                        sleep(Duration::new(2, 0));
                        music = play_current_music(&musics, current_music);
                        snake = Snake::new();
                        target = Target::new();
                    } else {
                        if new_head == target.pos {
                            snake.score += target.points;
                            sdl2::mixer::Channel::all().play(&target_eaten_sound, 0).unwrap();
                            target = Target::new();
                        } else {
                            snake.segments.pop_front();
                        }
                        snake.segments.push_back(new_head);
                    }
                }
            },
            Err(e) => println!("Error calculating time since last move: {:?}", e),
        }

        black_background(&mut canvas);
        let red = Color::RGB(255, 0, 0);
        canvas.set_draw_color(red);
        snake.segments.iter().for_each(|&s| {
                canvas.fill_rect(sdl2::rect::Rect::new(s.x * CELL_SIZE, s.y * CELL_SIZE, CELL_SIZE as u32, CELL_SIZE as u32)).unwrap();
        });
        canvas.set_draw_color(target.color);
        canvas.fill_rect(sdl2::rect::Rect::new(target.pos.x * CELL_SIZE, target.pos.y * CELL_SIZE, CELL_SIZE as u32, CELL_SIZE as u32)).unwrap();
        canvas.present();
    }
}
