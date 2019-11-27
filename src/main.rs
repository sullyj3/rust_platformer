extern crate ggez;
extern crate rand;

use std::path::*;
// use std::collections::HashMap;

use rand::thread_rng;
use rand::rngs::ThreadRng;

use ggez::*;
use ggez::graphics::*;
use ggez::event::{KeyCode, KeyMods, quit};

use nalgebra::*;

struct Platformer {
    rng: ThreadRng,
    dt: std::time::Duration,
    guy: Sprite,
    explosion: Sprite,
    laser: Sprite,
    ground: Image,

    avatar: Avatar
}

impl Platformer {
    fn new(ctx: &mut Context) -> Platformer {
        let mut platformer = Platformer {
            dt: std::time::Duration::new(0,0),
            rng: thread_rng(),
            guy: Sprite::load(3, 5, Path::new("/guy.png"), ctx),
            explosion: Sprite::load(6, 5, Path::new("/explosion.png"), ctx),
            laser: Sprite::load(4, 5, Path::new("/laser.png"), ctx),
            ground: Image::new(ctx, Path::new("/ground.png")).unwrap(),

            avatar: Avatar { 
                position: Point2::new(500., 500.),
                velocity: Vector2::new(0., 0.)
            }
        };
        platformer.ground.set_filter(FilterMode::Nearest);

        platformer
    }
}

const AVATAR_H_SPEED: f32 = 6.;

struct Avatar {
    position: Point2<f32>,
    velocity: Vector2<f32>
}

impl Avatar {
    fn left(&mut self) {
        self.velocity = Vector2::new(-AVATAR_H_SPEED, 0.);
    }

    fn right(&mut self) {
        self.velocity = Vector2::new(AVATAR_H_SPEED, 0.);
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

    fn draw(&self, ctx: &mut Context, param: DrawParam) -> GameResult<()> {
        self.sheet.draw(ctx, param.src(self.curr_frame_rect()))
    }
}

impl ggez::event::EventHandler for Platformer {

    fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods, _repeat: bool) {
        match keycode {
            KeyCode::Left => self.avatar.left(),
            KeyCode::Right => self.avatar.right(),
            KeyCode::Escape => quit(ctx),
            _ => {}
        }
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


        self.guy.draw(ctx, 
                      DrawParam::default()
                        .scale(Vector2::new(5., 5.))
                        .dest(self.avatar.position)
                     );
        self.explosion.draw(ctx, 
                      DrawParam::default()
                        .scale(Vector2::new(5., 5.))
                        .dest(Point2::new(300.0, 200.0))
                     );
        self.laser.draw(ctx, 
                      DrawParam::default()
                        .scale(Vector2::new(5., 5.))
                        .dest(Point2::new(400.0, 200.0))
                     );
        self.ground.draw(ctx,
                         DrawParam::default()
                         .scale(Vector2::new(5., 5.))
                         .dest(Point2::new(200.0, 400.0))
                        );

        present(ctx)
    }
}

fn main() {
    use ggez::conf::*;

    let winMode: WindowMode = WindowMode::default()
        .fullscreen_type(FullscreenType::True)
        .dimensions(2256.0, 1504.0);

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
