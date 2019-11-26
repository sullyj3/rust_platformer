extern crate ggez;
extern crate rand;

use std::path::*;

use rand::thread_rng;
use rand::rngs::ThreadRng;

use ggez::*;
use ggez::graphics::*;

use mint::Vector2;
use mint::Point2;

struct Platformer {
    rng: ThreadRng,
    dt: std::time::Duration,
    guy: Sprite
}

impl Platformer {
    fn new(ctx: &mut Context) -> Platformer {
        let mut guy_sheet: Image = Image::new(ctx, Path::new("/guy.png"))
            .expect("sprite not found!");
        guy_sheet.set_filter(FilterMode::Nearest);

        Platformer {
            dt: std::time::Duration::new(0,0),
            rng: thread_rng(),
            guy: Sprite::new(3,5,guy_sheet)
        }

    }
}

struct Sprite {
    frame_timer: i32,
    n_frames: i32,
    frame_duration: i32,
    sheet: Image
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

    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.dt = timer::delta(ctx);
        self.guy.inc_frame_timer();

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
        // debug_msg.draw( ctx, DrawParam::default().dest(Point2 {x: 0., y: 20.}) );


        self.guy.draw(ctx, 
                      DrawParam::default()
                        .scale(Vector2 {x:5., y:5.})
                        .dest(Point2 {x: 200.0, y: 200.0})
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
