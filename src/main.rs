
use ggez::event::{self, EventHandler, KeyCode, KeyMods};
use ggez::{graphics, timer, conf};
use ggez::{Context, GameResult};
use std::time::{SystemTime, UNIX_EPOCH};

use skyship_rust::{game_objects, screen_context, states, get_resource_folder};
use game_objects::level::Level;
use states::State;
use screen_context::ScreenContext;


#[cfg(target_os = "macos")]
static MODIFIER_KEY: KeyMods = KeyMods::LOGO;
#[cfg(not(target_os = "macos"))]
static MODIFIER_KEY: KeyMods = KeyMods::CTRL;

static SHOW_FRAMERATE: bool = true;


struct MainState {
    state: State,
    state_start: u64,
    level: Level,
    screen_ctx: ScreenContext,
    frame_time: (f64, u64),
    frame_rate: String
}

impl MainState {
    fn new() -> MainState {
        let mut s = MainState {
            state: State::Start,
            state_start: get_current_ms(),
            level: Level {
                attractors: Vec::new(),
                zippers: Vec::new(),
            },
            screen_ctx: ScreenContext::default(),
            frame_time: (1000.0 / 60.0, get_current_ms()),
            frame_rate: "".to_string()
        };
        s.level.load_level(0);
        s
    }
}


impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let graphics::Rect {x: _, y: _, w: width, h: height} = graphics::screen_coordinates(ctx);
        
        self.screen_ctx.size = (width, height);
        let lerp_amount = 0.05;
        self.frame_time.0 = self.frame_time.0 * (1.0 - lerp_amount) + (get_current_ms() - self.frame_time.1) as f64 * lerp_amount;
        self.frame_time.1 = get_current_ms();
        
        self.frame_rate = format!("{}", (1000.0 / self.frame_time.0 + 0.5) as u64);

        
        match self.state.update(&mut self.screen_ctx, &mut self.level, get_current_ms() - self.state_start) {
            Some(s) => {
                self.state = s;
                self.state_start = get_current_ms();
                self.update(ctx)?;
            }
            _ => (),
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());

        self.state.draw(ctx, &self.screen_ctx, &self.level, get_current_ms() - self.state_start)?;

        if SHOW_FRAMERATE {
            graphics::draw(
                ctx,
                &graphics::Text::new("Framerate: ".to_string() + self.frame_rate.as_str()),
                graphics::DrawParam::default()
            )?;
        }

        graphics::present(ctx)?;
        timer::yield_now();
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, key: KeyCode, mods: KeyMods, _: bool) {
        match key {
            // Quit if MODIFIER_KEY+Q is pressed.
            KeyCode::Q => {
                if mods == MODIFIER_KEY {
                    event::quit(ctx);
                }
            }
            KeyCode::W => {
                if mods == MODIFIER_KEY | KeyMods::SHIFT {
                    event::quit(ctx);
                }
            }
            _ => (),
        }
    }

    fn quit_event(&mut self, _: &mut Context) -> bool {
        let (ctx, event_loop) = ggez::ContextBuilder::new("SkyShip", "Sky C").build().unwrap();
        event::run(ctx, event_loop, MainState::new())
    }

    fn resize_event(&mut self, ctx: &mut Context, _width: f32, _height: f32) {
        let winit::dpi::LogicalSize {width, height} = graphics::window(ctx).inner_size().to_logical(graphics::window(ctx).scale_factor());
        graphics::set_screen_coordinates(ctx, graphics::Rect { x: 0.0, y: 0.0, w: width, h: height }).unwrap();
    }
}

pub fn main() -> ggez::GameResult {

    println!("{}", get_resource_folder());

    let mut cb = ggez::ContextBuilder::new("SkyShip", "Sky C");
    cb = cb.window_setup(conf::WindowSetup {
        title: "SkyShip".to_owned(),
        samples: conf::NumSamples::Sixteen,
        vsync: true,
        icon: "".to_owned(),
        srgb: true,
    });
    let (mut ctx, event_loop) = cb.build()?;

    graphics::set_mode(
        &mut ctx, 
        conf::WindowMode {

            width: 800.0,
            height: 600.0,

            min_width: 400.0,
            min_height: 300.0,

            max_width: 0.0,
            max_height: 0.0,

            resizable: true,
            visible: true,
            borderless: false,

            maximized: false,
            fullscreen_type: conf::FullscreenType::Windowed,
        }
    )?;

    let state = MainState::new();
    println!("{:?}", ctx.continuing);
    event::run(ctx, event_loop, state)
}

fn get_current_ms() -> u64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_secs() * 1000 + since_the_epoch.subsec_nanos() as u64 / 1_000_000
}