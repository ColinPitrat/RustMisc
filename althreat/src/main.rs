extern crate sdl2; 
extern crate rand;

use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::KeyboardState;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Scancode;
use sdl2::image::{LoadTexture, INIT_PNG};
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use sdl2::video::Window;
use sdl2::video::WindowContext;
use std::cmp::Ordering;
use std::time::{SystemTime,Duration};
 
const SCREEN_WIDTH : i32 = 1600;
const SCREEN_HEIGHT : i32 = 1200;
const STAGE_START : i32 = 1800;
const STAGE_END : i32 = 10000;
const NB_STARS : u32 = SCREEN_WIDTH as u32 * SCREEN_HEIGHT as u32 / 10000;

struct Point {
    x: f64,
    y: f64,
}

impl<'a> From<&'a Point> for sdl2::rect::Point {
    fn from(p: &'a Point) -> Self {
        sdl2::rect::Point::new(p.x as i32, p.y as i32)
    }
}

impl Point {
    fn new(x: f64, y: f64) -> Point {
        Point{x, y}
    }
}

struct Rect {
    p: Point,
    w: f64,
    h: f64,
}

impl<'a> From<&'a Rect> for sdl2::rect::Rect {
    fn from(r: &'a Rect) -> Self {
        sdl2::rect::Rect::new(r.p.x as i32, r.p.y as i32, r.w as u32, r.h as u32)
    }
}

impl<'a> From<&'a Rect> for std::option::Option<sdl2::rect::Rect> {
    fn from(r: &'a Rect) -> Self {
        Some(sdl2::rect::Rect::new(r.p.x as i32, r.p.y as i32, r.w as u32, r.h as u32))
    }
}

impl Rect {
    fn new(x: f64, y: f64, w: f64, h: f64) -> Rect {
        Rect{p: Point{x, y}, w, h}
    }
}

struct Star {
    p: Point,
    color: Color,
}

struct Player<'a> {
    r: Rect,
    speed: f64,
    sprite: Texture<'a>,
    last_laser: SystemTime,
}

struct Enemy<'a> {
    r: Rect,
    speed: Point,
    sprite: Texture<'a>,
}

struct Laser {
    r: Rect,
    speed: Point,
}

struct DrawingContext {
    sdl_context: sdl2::Sdl,
    canvas: Canvas<Window>,
    texture_creator: TextureCreator<WindowContext>,
}

fn init_dc() -> DrawingContext {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let _image_context = sdl2::image::init(INIT_PNG).unwrap();
 
    let window = video_subsystem.window("AlThreat", SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();
 
    let canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    DrawingContext{ sdl_context, canvas, texture_creator }
}

fn random(min: i32, max: i32) -> f64 {
    rand::thread_rng().gen_range(min as i32, max as i32) as f64
}

fn random_speed(min: f64, max: f64) -> f64 {
    random(1000*min as i32, 1000*max as i32) / 1000 as f64
}

fn random_x() -> f64 {
    random(0, SCREEN_WIDTH)
}

fn random_pos() -> f64 {
    random(STAGE_START, STAGE_END)
}

fn random_y() -> f64 {
    random(0, SCREEN_HEIGHT)
}

fn random_color() -> Color {
    Color::RGB(rand::thread_rng().gen_range(0, 255),
               rand::thread_rng().gen_range(0, 255),
               rand::thread_rng().gen_range(0, 255))
}

fn black_background(canvas: &mut Canvas<Window>) {
        let black = Color::RGB(0, 0, 0);
        canvas.set_draw_color(black);
        canvas.clear();
}

fn init_stars(nb: u32) -> Vec<Star> {
    let mut stars = vec!();
    let mut i = 0;
    while i < nb {
        let p = Point::new(random_x(), random_y());
        let color = random_color();
        let s = Star{p, color};
        stars.push(s);
        i += 1;
    }
    stars
}

fn init_player(texture_creator: &TextureCreator<WindowContext>) -> Player {
    // TODO: (SCREEN_HEIGHT-PLAYER_HEIGHT)/2
    // TODO: Variable size for player
    let r = Rect::new(0.0, (SCREEN_HEIGHT/2) as f64, 116.0, 48.0);
    let sprite = texture_creator.load_texture("/home/cpitrat/Perso/RustMisc/althreat/resources/z95.png").unwrap();
    let speed = 5.0;
    let player = Player{r, speed, sprite, last_laser: SystemTime::now()};
    player
}

fn show_stars(canvas: &mut Canvas<Window>, stars: &Vec<Star>) {
        for s in stars.iter() {
            canvas.set_draw_color(s.color);
            canvas.draw_point(&s.p).expect("Draw point failed !");
        }
}

fn move_stars(stars: &mut Vec<Star>) {
    for s in stars.iter_mut() {
        s.p.x -= 1.0;
        if s.p.x < 0.0 {
            s.p.x = SCREEN_WIDTH as f64;
            s.p.y = random_y();
            s.color = random_color();
        }
    }
}

fn show_player(canvas: &mut Canvas<Window>, player: &Player) {
    canvas.copy(&player.sprite, None, &player.r).expect("Rendering player failed");
}

fn move_player(player: &mut Player, pump: &sdl2::EventPump) {
    let ks = KeyboardState::new(&pump);
    if ks.is_scancode_pressed(Scancode::Up) {
        player.r.p.y -= player.speed;
        if player.r.p.y < 0.0 {
            player.r.p.y = 0.0;
        }
    }
    if ks.is_scancode_pressed(Scancode::Down) {
        player.r.p.y += player.speed;
        if player.r.p.y > (SCREEN_HEIGHT-48) as f64 {
            player.r.p.y = (SCREEN_HEIGHT-48) as f64;
        }
    }
    if ks.is_scancode_pressed(Scancode::Left) {
        player.r.p.x -= player.speed;
        if player.r.p.x < 0.0 {
            player.r.p.x = 0.0;
        }
    }
    if ks.is_scancode_pressed(Scancode::Right) {
        player.r.p.x += player.speed;
        if player.r.p.x > (SCREEN_WIDTH-116) as f64{
            player.r.p.x = (SCREEN_WIDTH-116) as f64;
        }
    }
}

fn player_shoots(player: &mut Player, lasers: &mut Vec<Laser>, pump: &sdl2::EventPump) {
    let ks = KeyboardState::new(&pump);
    if ks.is_scancode_pressed(Scancode::Space) {
        add_laser(lasers, player);
    }
}

fn init_enemies(texture_creator: &TextureCreator<WindowContext>) -> Vec<Enemy> {
    let mut enemies = vec!();
    let mut i = 0;
    // Number of enemies should depend on stage
    while i < 1000 {
        // TODO: Variable size for enemies
        let r = Rect::new(random_pos(), random_y(), 93.0, 41.0);
        // Speed ranges should depend on stage
        let speed = Point::new(random_speed(-5.0, -1.0), random_speed(-2.0, 2.0));
        // TODO: Animated sprites
        let sprite = texture_creator.load_texture("/home/cpitrat/Perso/RustMisc/althreat/resources/ovni1/ovni1.png").unwrap();
        let enemy = Enemy{r, speed, sprite};
        enemies.push(enemy);
        i += 1;
    }
    enemies.sort_unstable_by(|a,b| a.r.p.x.partial_cmp(&b.r.p.x).unwrap());
    enemies
}

fn move_enemies(enemies: &mut Vec<Enemy>) {
    for e in enemies.iter_mut() {
        if e.r.p.x > SCREEN_WIDTH as f64 {
            e.r.p.x -= 1.0;
        } else {
            // TODO: Remove all the ones that are out of the screen ?
            e.r.p.x += e.speed.x;
            e.r.p.y += e.speed.y;
        }
    }
}

fn show_enemies(canvas: &mut Canvas<Window>, enemies: &Vec<Enemy>) {
    for e in enemies.iter() {
        // TODO: Size should be part of the sprite somehow
        canvas.copy(&e.sprite, None, &e.r).expect("Rendering enemy failed");
    }
}

fn init_lasers() -> Vec<Laser> {
    vec!()
}

fn move_lasers(lasers: &mut Vec<Laser>) {
    for l in lasers.iter_mut() {
        l.r.p.x += l.speed.x;
        l.r.p.y += l.speed.y;
    }
    lasers.retain(|l| l.r.p.x < SCREEN_WIDTH as f64)
}

fn add_laser(lasers: &mut Vec<Laser>, player: &mut Player) {
    // TODO: Replace the line by a sprite
    // TODO: Handle multiple weapons (how to replace last_laser ? need a hashmap instead ?)
    let now = SystemTime::now();
    if now.duration_since(player.last_laser).expect("Couldn't compute duration").cmp(&Duration::new(0, 300_000_000)) == Ordering::Greater {
        let r = Rect::new(player.r.p.x + 50.0, player.r.p.y + 22.0, 20.0, 0.0);
        let speed = Point{x: 10.0, y: 0.0};
        let laser = Laser{r, speed};
        lasers.push(laser);
        player.last_laser = SystemTime::now();
    }
}

fn show_lasers(canvas: &mut Canvas<Window>, lasers: &Vec<Laser>) {
    let blue = Color::RGB(0, 0, 255);
    canvas.set_draw_color(blue);
    for l in lasers.iter() {
        let p = Point{x: l.r.p.x + l.r.w, y: l.r.p.y + l.r.h};
        canvas.draw_line(&l.r.p, &p).expect("Rendering laser failed");
    }
}

fn intersect<R: Into<sdl2::rect::Rect>>(r1: R, r2: R) -> bool {
    r1.into().has_intersection(r2.into())
}

fn laser_hits(lasers: &mut Vec<Laser>, enemies: &mut Vec<Enemy>) {
    for l in lasers.iter_mut() {
        for e in enemies.iter_mut() {
            if intersect(&l.r, &e.r) {
                // Funny way to get rid of the laser & the enemy
                l.r.p.x = SCREEN_WIDTH as f64;
                // TODO: Enemy should explode instead
                e.r.p.x = -e.r.w;
            }
        }
    }
}

// TODO: Stages
// TODO: Score
// TODO: Energy
// TODO: Lives
// TODO: Animated sprites
pub fn main() {
    let mut dc = init_dc();
    let mut stars = init_stars(NB_STARS);
    let mut player = init_player(&dc.texture_creator);
    let mut enemies = init_enemies(&dc.texture_creator);
    let mut lasers = init_lasers();

    let mut event_pump = dc.sdl_context.event_pump().unwrap();
    'game_loop: loop {
        black_background(&mut dc.canvas);
        show_stars(&mut dc.canvas, &stars);
        move_stars(&mut stars);
        show_player(&mut dc.canvas, &player);
        move_player(&mut player, &event_pump);
        show_enemies(&mut dc.canvas, &enemies);
        move_enemies(&mut enemies);
        show_lasers(&mut dc.canvas, &lasers);
        move_lasers(&mut lasers);
        player_shoots(&mut player, &mut lasers, &event_pump);
        laser_hits(&mut lasers, &mut enemies);
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'game_loop
                },
                _ => {}
            }
        }

        dc.canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

