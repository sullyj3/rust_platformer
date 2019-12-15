extern crate ggez;
//extern crate cgmath;
extern crate rand;

use std::path::*;
// use std::collections::HashMap;

use rand::thread_rng;
use rand::rngs::ThreadRng;

use ggez::*;
use ggez::graphics::*;
use ggez::event::{KeyCode, KeyMods, quit};

//use cgmath::*;
use nalgebra::*;

const PIXEL_SIZE: i32 = 5;

struct Platformer {
    input: InputState,

    rng: ThreadRng,
    dt: std::time::Duration,
    guy: Sprite,
    explosion: Sprite,
    laser: Sprite,
    ground: Tile,

    avatar: Avatar
}

impl Platformer {
    fn new(ctx: &mut Context) -> Platformer {
        let mut groundImg = Image::new(ctx, Path::new("/ground.png")).unwrap();
        groundImg.set_filter(FilterMode::Nearest);

        let mut platformer = Platformer {
            dt: std::time::Duration::new(0,0),
            rng: thread_rng(),

            guy: Sprite::load(3, 5, Path::new("/guy.png"), ctx),
            explosion: Sprite::load(6, 5, Path::new("/explosion.png"), ctx),
            laser: Sprite::load(4, 5, Path::new("/laser.png"), ctx),
            ground: Tile::new(groundImg, Point2::new(50, 50)),

            avatar: Avatar { 
                position: Point2::new(60., 60.),
                velocity: Vector2::new(0., 0.)
            },
            input: InputState {
                arrowLeftDown:  false,
                arrowRightDown: false,
                arrowUpDown:    false,
                arrowDownDown:  false,

                spaceDown:      false
            }
        };

        platformer
    }
}

const AVATAR_H_SPEED: f32 = 1.;
const AVATAR_V_SPEED: f32 = 1.;

struct Avatar {
    // pixel space
    position: Point2<f32>,
    velocity: Vector2<f32>
}

impl Avatar {
    fn key_down_event(&mut self, keyCode: KeyCode, _input: &InputState) {
        match keyCode {
            KeyCode::Up    => self.velocity.y = -AVATAR_V_SPEED,
            KeyCode::Down  => self.velocity.y = AVATAR_V_SPEED,
            KeyCode::Left  => self.velocity.x = -AVATAR_H_SPEED,
            KeyCode::Right => self.velocity.x = AVATAR_H_SPEED,
            _ => {}
        }
    }

    fn key_up_event(&mut self, keyCode: KeyCode, input: &InputState) {
        match keyCode {
            KeyCode::Up    => {
                self.velocity.y = if input.arrowDownDown { AVATAR_V_SPEED } else { 0. }
            }
            KeyCode::Down => {
                self.velocity.y = if input.arrowUpDown { -AVATAR_V_SPEED } else { 0. }
            }

            KeyCode::Left => {
                self.velocity.x = if input.arrowRightDown { AVATAR_H_SPEED } else { 0. }
            } 
            KeyCode::Right => {
                self.velocity.x = if input.arrowLeftDown { -AVATAR_H_SPEED } else { 0. }
            } 
            _ => {}
        }
    }
}

struct Sprite {
    frame_timer: i32,
    n_frames: i32,
    frame_duration: i32,
    sheet: Image
}

struct SpriteParam {
    n_frames: i32,
    frame_duration: i32
}

impl Default for SpriteParam {
    fn default() -> SpriteParam {
        SpriteParam {
            n_frames: 1,
            frame_duration: 1
        }
    }
}

impl Sprite {
    fn new(n_frames: i32, frame_duration: i32, sheet: Image) -> Sprite {
        Sprite {
            frame_timer: 0,
            n_frames: n_frames,
            frame_duration: frame_duration,
            sheet: sheet
        }
    }

    // Assume FilterMode::Nearest (no antialiasing), for pixel art
    fn load(n_frames: i32, frame_duration: i32, path: &Path, ctx: &mut Context) -> Sprite {
        let mut sheet: Image = Image::new(ctx, path)
            .expect(format!("{:?} not found!", path).as_str());
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
        let width = 1./n_frames as f32;
        let x = nth as f32 * width;
        Rect::new(x, 0., width, 1.)
    }

    fn draw(&self, ctx: &mut Context, dest: Point2<i32>) -> GameResult<()> {
        let drawParam = PixelDrawParam::default().src(self.curr_frame_rect())
                                                 .dest(dest);

        self.sheet.pixelDraw(ctx, drawParam)
    }
}

struct Tile {
    image: Image,
    position: Point2<i32>
}

impl Tile {
    fn new(image: Image, position: Point2<i32>) -> Tile {
        Tile { image: image, position: position }
    }

    // TODO: create load function

    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let drawParam = DrawParam::default().dest(floatP2(self.position * PIXEL_SIZE))
                                            .scale(Vector2::new(PIXEL_SIZE as f32, PIXEL_SIZE as f32));
        self.image.draw(ctx, drawParam)
    }
}

// both of these are nasty @HACKs
fn floatP2(p: Point2<i32>) -> Point2<f32> {
    Point2::new(p.x as f32, p.y as f32)
}

fn roundP2(p: Point2<f32>) -> Point2<i32> {
    Point2::new(p.x.round() as i32, p.y.round() as i32)
}

struct InputState {
    arrowLeftDown: bool,
    arrowRightDown: bool,
    arrowUpDown: bool,
    arrowDownDown: bool,

    spaceDown: bool
}

impl ggez::event::EventHandler for Platformer {

    fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods, _repeat: bool) {
        match keycode {
            KeyCode::Left => self.input.arrowLeftDown = true,
            KeyCode::Right => self.input.arrowRightDown = true,
            KeyCode::Up => self.input.arrowUpDown = true,
            KeyCode::Down => self.input.arrowDownDown = true,

            KeyCode::Escape => quit(ctx),
            _ => {}
        }

        self.avatar.key_down_event(keycode, &self.input);
    }

    fn key_up_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods) {
        match keycode {
            KeyCode::Left => self.input.arrowLeftDown = false,
            KeyCode::Right => self.input.arrowRightDown = false,
            KeyCode::Up => self.input.arrowUpDown = false,
            KeyCode::Down => self.input.arrowDownDown = false,
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

        self.avatar.position += self.avatar.velocity;

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        //use rand::seq::SliceRandom;

        // let RED = Color::new(1.0,0.0,0.0,1.0);
        // let GREEN = Color::new(0.0,1.0,0.0,1.0);
        // let BLUE = Color::new(0.0,0.0,1.0,1.0);
        //
        let bg_color = Color::new(0.59, 0.75, 0.85, 1.);

        clear(ctx, bg_color);

        let perf = Text::new(format!("fps: {:.*} hz, dt: {:.*}ns",
                                     2, timer::fps(&ctx),
                                     2, self.dt.subsec_nanos()));
        //let debug_msg = Text::new(format!("rect = {:?}", self.guy.curr_frame_rect()));


        perf.draw( ctx, DrawParam::default() );

        // let iPosition: Point2<i32> = self.avatar.position.into();
        let iPosition: Point2<i32> = roundP2(self.avatar.position);
        self.guy.draw(ctx, iPosition);
        self.explosion.draw(ctx, Point2::new(30, 20));
        self.laser.draw(ctx, Point2::new(40, 20));
        self.ground.draw(ctx);

        present(ctx)
    }
}

trait DrawPixelSpace {
    fn pixelDraw(&self, ctx: &mut Context, pixelDrawParam: PixelDrawParam) -> GameResult<()>;
}

struct PixelDrawParam {
    src: Rect,
    dest: Point2<i32>,
    scale: Vector2<f32>,
    offset: Point2<f32>
}

impl PixelDrawParam {
    fn toDrawParam(&self) -> DrawParam {
        DrawParam::default()
            .src(self.src)
            .dest(floatP2(self.dest * PIXEL_SIZE))
            .scale(self.scale * PIXEL_SIZE as f32)
            .offset(self.offset * PIXEL_SIZE as f32)
    }

    fn src(&self, src: Rect) -> PixelDrawParam {
        PixelDrawParam { src: src, ..*self }
    }

    fn dest(&self, dest: Point2<i32>) -> PixelDrawParam {
        PixelDrawParam { dest: dest, ..*self }
    }
}

impl Default for PixelDrawParam {
    fn default() -> PixelDrawParam {
        PixelDrawParam { src: Rect::new(0., 0., 1., 1.),
                         dest: Point2::new(0, 0),
                         scale: Vector2::new(1., 1.),
                         offset: Point2::new(0., 0.) }
    }
}

impl DrawPixelSpace for Image {
    fn pixelDraw(&self, ctx: &mut Context, pixelDrawParam: PixelDrawParam) -> GameResult<()> {
        self.draw(ctx, pixelDrawParam.toDrawParam())
    }
}

fn main() {
    use ggez::conf::*;

    let winMode: WindowMode = WindowMode::default()
        .fullscreen_type(FullscreenType::True)
        .dimensions( (240*PIXEL_SIZE) as f32
                   , (160*PIXEL_SIZE) as f32); 

    let c = conf::Conf::new()
        .window_mode(winMode);

    let res_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("resources");
    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("hello_ggez", "Jimmy")
        .conf(c)
        .add_resource_path(res_dir) //???
        .build()
        .unwrap();

    let mut platformer = Platformer::new(ctx);

    event::run(ctx, event_loop, &mut platformer).unwrap();

}
