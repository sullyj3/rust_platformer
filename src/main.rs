extern crate ggez;
//extern crate cgmath;
extern crate rand;

use std::io::prelude::*;
//use std::io::Result;
//use std::fs::File;
use std::path::*;
// use std::collections::HashMap;

//use rand::rngs::ThreadRng;
//use rand::thread_rng;

use ggez::event::{quit, KeyCode, KeyMods};
use ggez::graphics::*;
use ggez::*;

//use cgmath::*;
use nalgebra::*;


const GRAVITY_STRENGTH: f32 = 0.25; // px/frame/frame?
const AVATAR_H_SPEED: f32 = 1.;
const AVATAR_V_SPEED: f32 = 1.;
const AVATAR_JUMP_SPEED: f32 = -4.;

const TILE_SIZE: i32 = 16;

const SCREEN_WIDTH: i16 = 2256;
const SCREEN_HEIGHT: i16 = 1504;

const WORLD_WIDTH: i16 = 240;
const WORLD_HEIGHT: i16 = 160;

struct Platformer {
    input: InputState,

    //rng: ThreadRng,
    dt: std::time::Duration,
    guy: Sprite,
    explosion: Sprite,
    laser: Sprite,
    ground: Image,

    avatar: Avatar,
    current_level: Level,

    ui_canvas: Canvas,
    world_canvas: Canvas,
}


impl Platformer {
    fn new(ctx: &mut Context) -> Platformer {
        let mut ground_img = Image::new(ctx, Path::new("/ground.png")).unwrap();
        ground_img.set_filter(FilterMode::Nearest);
        let current_level = Level::load(Path::new("/levels/level1.txt"), ctx);


        Platformer {
            dt: std::time::Duration::new(0, 0),
            //rng: thread_rng(),

            guy: Sprite::load(3, 5, Path::new("/guy.png"), ctx),
            explosion: Sprite::load(6, 5, Path::new("/explosion.png"), ctx),
            laser: Sprite::load(4, 5, Path::new("/laser.png"), ctx),
            ground: ground_img,

            avatar: Avatar {
                physics: Physics { 
                    position: float_p2(current_level.player_start_loc),
                    velocity: Vector2::new(0., 0.),
                    acceleration: Vector2::new(0.0, GRAVITY_STRENGTH),
                },
            },
            input: InputState {
                arrow_left_down: false,
                arrow_right_down: false,
                arrow_up_down: false,
                arrow_down_down: false,

                space_down: false,
            },
            current_level: current_level,

            world_canvas: Canvas::new(ctx, 240, 160, ggez::conf::NumSamples::One)
                .expect("couldn't init world_canvas"),
            ui_canvas   : Canvas::new(ctx, 2206, 1504, ggez::conf::NumSamples::One)
                .expect("couldn't init ui_canvas"),
        }
    }
}


struct Avatar {
    physics: Physics,
}

struct Physics {
    position: Point2<f32>,
    velocity: Vector2<f32>,
    acceleration: Vector2<f32>,
}

impl Physics {
    fn apply_acceleration(&mut self) {
        self.velocity += self.acceleration;
    }

    // return previous position
    fn apply_x_velocity(&mut self) -> Point2<f32> {
        let previous_position = self.position;
        self.position += x_component(self.velocity);
        previous_position
    }

    // return previous position
    fn apply_y_velocity(&mut self) -> Point2<f32> {
        let previous_position = self.position;
        self.position += y_component(self.velocity);
        previous_position
    }

}


impl Avatar {
    fn key_down_event(&mut self, keycode: KeyCode, _input: &InputState) {
        match keycode {
            // KeyCode::Up => self.physics.velocity.y = -AVATAR_V_SPEED,
            // KeyCode::Down => self.physics.velocity.y = AVATAR_V_SPEED,
            KeyCode::Left => self.physics.velocity.x = -AVATAR_H_SPEED,
            KeyCode::Right => self.physics.velocity.x = AVATAR_H_SPEED,
            KeyCode::Space => self.physics.velocity.y = AVATAR_JUMP_SPEED,
            _ => {}
        }
    }

    fn key_up_event(&mut self, keycode: KeyCode, input: &InputState) {
        match keycode {
            // KeyCode::Up => {
            //     self.physics.velocity.y = if input.arrow_down_down {
            //         AVATAR_V_SPEED
            //     } else {
            //         0.
            //     }
            // }
            // KeyCode::Down => {
            //     self.physics.velocity.y = if input.arrow_up_down {
            //         -AVATAR_V_SPEED
            //     } else {
            //         0.
            //     }
            // }

            KeyCode::Left => {
                self.physics.velocity.x = if input.arrow_right_down {
                    AVATAR_H_SPEED
                } else {
                    0.
                }
            }
            KeyCode::Right => {
                self.physics.velocity.x = if input.arrow_left_down {
                    -AVATAR_H_SPEED
                } else {
                    0.
                }
            }
            _ => {}
        }
    }
}

impl AABB for Avatar {
    fn aabb(&self) -> Rect {
        let (x, y) = (self.physics.position.x, self.physics.position.y);
        Rect::new(x, y, 7.0, 16.0)
    }
}

struct Sprite {
    frame_timer: i32,
    n_frames: i32,
    frame_duration: i32,
    sheet: Image,
}

struct SpriteParam {
    n_frames: i32,
    frame_duration: i32,
}

impl Default for SpriteParam {
    fn default() -> SpriteParam {
        SpriteParam {
            n_frames: 1,
            frame_duration: 1,
        }
    }
}

impl Sprite {
    fn new(n_frames: i32, frame_duration: i32, sheet: Image) -> Sprite {
        Sprite {
            frame_timer: 0,
            n_frames: n_frames,
            frame_duration: frame_duration,
            sheet: sheet,
        }
    }

    // Assume FilterMode::Nearest (no antialiasing), for pixel art
    fn load(n_frames: i32, frame_duration: i32, path: &Path, ctx: &mut Context) -> Sprite {
        let mut sheet: Image =
            Image::new(ctx, path).expect(format!("{:?} not found!", path).as_str());
        sheet.set_filter(FilterMode::Nearest);

        Sprite::new(n_frames, frame_duration, sheet)
    }

    fn inc_frame_timer(&mut self) {
        self.frame_timer = (self.frame_timer + 1) % (self.n_frames * self.frame_duration);
    }

    fn curr_frame(&self) -> i32 {
        self.frame_timer / self.frame_duration
    }

    fn curr_frame_rect(&self) -> Rect {
        Sprite::nth_frame_rect(self.curr_frame(), self.n_frames)
    }

    fn nth_frame_rect(nth: i32, n_frames: i32) -> Rect {
        let width = 1. / n_frames as f32;
        let x = nth as f32 * width;
        Rect::new(x, 0., width, 1.)
    }

    fn draw(&self, ctx: &mut Context, dest: Point2<i32>) -> GameResult<()> {
        let draw_param = DrawParam::default()
            .src(self.curr_frame_rect())
            .dest(float_p2(dest));

        self.sheet.draw(ctx, draw_param)
    }
}

struct Level {
    ground_tiles: Vec<Tile>,
    player_start_loc: Point2<i32>,
}


trait AABB {
    fn aabb(&self) -> Rect;
}


impl Level {
    fn from_string(s: String) -> Level {
        let mut ground_tiles = Vec::new();
        let mut player_start_loc = Point2::new(0, 0);

        for (j, line) in s.lines().enumerate() {
            for (i, char) in line.char_indices() {

                let (x, y): (i32, i32) = (i as i32 * TILE_SIZE, j as i32 * TILE_SIZE);

                if char == 'g' {
                    // BUG these are tile width coordinates, not pixel width
                    let tile_position = Point2::new(x, y);
                    ground_tiles.push(Tile::new(tile_position));
                } else if char == 'p' {
                    player_start_loc = Point2::new(x, y);
                }
            }
        }

        Level {
            ground_tiles: ground_tiles,
            player_start_loc: player_start_loc,
        }
    }

    fn load(path: &Path, ctx: &mut Context) -> Level {
        let mut file = filesystem::open(ctx, path).expect("Couldn't find level!");
        let mut level_str = String::new();
        file.read_to_string(&mut level_str)
            .expect("couldn't read file!");

        Level::from_string(level_str)
    }
}

struct Tile {
    position: Point2<i32>,
}

impl Tile {
    fn new(position: Point2<i32>) -> Tile {
        Tile { position: position }
    }

    // TODO: create load function

    fn draw(&self, ctx: &mut Context, tile_img: &Image) -> GameResult<()> {
        let draw_param = DrawParam::default()
            .dest(float_p2(self.position));
        tile_img.draw(ctx, draw_param)
    }
}

impl AABB for Tile {
    fn aabb(&self) -> Rect {
        Rect::new(self.position.x as f32, self.position.y as f32, TILE_SIZE as f32, TILE_SIZE as f32)
    }
}

// both of these are nasty @HACKs
fn float_p2(p: Point2<i32>) -> Point2<f32> {
    Point2::new(p.x as f32, p.y as f32)
}

fn round_p2(p: Point2<f32>) -> Point2<i32> {
    Point2::new(p.x.round() as i32, p.y.round() as i32)
}

struct InputState {
    arrow_left_down: bool,
    arrow_right_down: bool,
    arrow_up_down: bool,
    arrow_down_down: bool,

    space_down: bool,
}

fn x_component(v: Vector2<f32>) -> Vector2<f32> {
    Vector2::new(v.x, 0.0)
}

fn y_component(v: Vector2<f32>) -> Vector2<f32> {
    Vector2::new(0.0, v.y)
}

impl ggez::event::EventHandler for Platformer {
    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _key_mod: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::Left => self.input.arrow_left_down = true,
            KeyCode::Right => self.input.arrow_right_down = true,
            KeyCode::Up => self.input.arrow_up_down = true,
            KeyCode::Down => self.input.arrow_down_down = true,

            KeyCode::Escape => quit(ctx),
            _ => {}
        }

        self.avatar.key_down_event(keycode, &self.input);
    }

    fn key_up_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods) {
        match keycode {
            KeyCode::Left => self.input.arrow_left_down = false,
            KeyCode::Right => self.input.arrow_right_down = false,
            KeyCode::Up => self.input.arrow_up_down = false,
            KeyCode::Down => self.input.arrow_down_down = false,
            KeyCode::Escape => quit(ctx),
            _ => {}
        }

        self.avatar.key_up_event(keycode, &self.input);
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.dt = timer::delta(ctx);

        self.guy.inc_frame_timer();
        self.explosion.inc_frame_timer();
        self.laser.inc_frame_timer();

        self.avatar.physics.apply_acceleration();

        // x
        let old_position = self.avatar.physics.apply_x_velocity();
        let avatarAABB = self.avatar.aabb();
        for tile in self.current_level.ground_tiles.iter() {
            if avatarAABB.overlaps(&tile.aabb()) {
                self.avatar.physics.position = old_position;
                self.avatar.physics.velocity.x = 0.0;
                break;
            }
        }

        // y
        let old_position = self.avatar.physics.apply_y_velocity();
        let avatarAABB = self.avatar.aabb();
        for tile in self.current_level.ground_tiles.iter() {
            if avatarAABB.overlaps(&tile.aabb()) {
                self.avatar.physics.position = old_position;
                self.avatar.physics.velocity.y = 0.0;
                break;
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let bg_blue = Color::new(0.59, 0.75, 0.85, 1.);
        let RED = Color::new(1.0, 0.0, 0.0, 1.0);
        let alpha = Color::new(0.0, 0.0, 0.0, 0.0);

        ///////////
        // World //
        ///////////

        set_canvas(ctx, Some(&self.world_canvas));
        clear(ctx, bg_blue);

        self.guy.draw(ctx, round_p2(self.avatar.physics.position))?;
        let avatarBB = Mesh::new_rectangle(ctx,
                                           DrawMode::Stroke(StrokeOptions::default()),
                                           self.avatar.aabb(),
                                           RED).unwrap();
        avatarBB.draw(ctx, DrawParam::default());
        // self.explosion.draw(ctx, Point2::new(30, 20))?;
        // self.laser.draw(ctx, Point2::new(40, 20))?;
        for tile in self.current_level.ground_tiles.iter() {
            tile.draw(ctx, &self.ground)?;
        }

        ////////
        // UI //
        ////////

        set_canvas(ctx, Some(&self.ui_canvas));
        clear(ctx, alpha);

        // draw debug info on top
        // let perf = Text::new(format!(
        //     "fps: {:.*} hz, dt: {:.*}ns",
        //     2,
        //     timer::fps(&ctx),
        //     2,
        //     self.dt.subsec_nanos()
        // ));
        //perf.draw(ctx, DrawParam::default())?;

        let debug_msg = Text::new(format!("avatar position: {:?}", self.avatar.aabb()));
        debug_msg.draw(ctx, DrawParam::default())?;

        ////////////////////
        // Draw canvasses //
        ////////////////////
        set_canvas(ctx, None);

        let world_canvas_scale = Vector2::new(SCREEN_WIDTH as f32 / WORLD_WIDTH as f32,
                                              SCREEN_HEIGHT as f32 / WORLD_HEIGHT as f32);

        self.world_canvas.draw(ctx, DrawParam::default()
                               .scale(world_canvas_scale))?;
        self.ui_canvas.draw(ctx, DrawParam::default())?;

        present(ctx)
    }
}

fn main() {
    use ggez::conf::*;

    let win_mode: WindowMode = WindowMode::default()
        .fullscreen_type(FullscreenType::True)
        .dimensions(SCREEN_WIDTH.into(), SCREEN_HEIGHT.into());

    let c = conf::Conf::new().window_mode(win_mode);

    let res_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("resources");
    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("hello_ggez", "Jimmy")
        .conf(c)
        .add_resource_path(res_dir) //???
        .build()
        .unwrap();

    let mut platformer = Platformer::new(ctx);

    event::run(ctx, event_loop, &mut platformer).unwrap();
}
