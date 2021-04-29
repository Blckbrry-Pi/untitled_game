
use ggez::event::{self, EventHandler, KeyCode, KeyMods};
use ggez::{graphics, timer, conf};
use ggez::input::keyboard;
use ggez::{Context, GameResult};
use std::time::{SystemTime, UNIX_EPOCH};

use untitled_game::{game_objects, screen_context};
use game_objects::attractor::Attractor;
use screen_context::ScreenContext;


#[cfg(target_os = "macos")]
static MODIFIER_KEY: KeyMods = KeyMods::LOGO;
#[cfg(not(target_os = "macos"))]
static MODIFIER_KEY: KeyMods = KeyMods::CTRL;

static SHOW_FRAMERATE: bool = true;



struct MainState {
    attractors: Vec<Attractor>,
    position_x: f32,
    screen_ctx: ScreenContext,
    frame_time: (f64, u64),
    frame_rate: String
}

impl MainState {
    fn new() -> ggez::GameResult<MainState> {
        let s = MainState {
            attractors: Vec::new(),
            position_x: 0.0,
            screen_ctx: ScreenContext::default(),
            frame_time: (0.0, get_current_ms()),
            frame_rate: "".to_string()
        };
        Ok(s)
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {

        let lerp_amount = 0.05;
        self.frame_time.0 = self.frame_time.0 * (1.0 - lerp_amount) + (get_current_ms() - self.frame_time.1) as f64 * lerp_amount;
        self.frame_time.1 = get_current_ms();
        
        self.frame_rate = format!("{}", (1000.0 / self.frame_time.0 + 0.5) as u64);

        // Increase or decrease `position_x` by 0.5, or by 5.0 if Shift is held.
        if keyboard::is_key_pressed(ctx, KeyCode::Right) {
            if keyboard::is_mod_active(ctx, KeyMods::SHIFT) {
                self.position_x += 4.5;
            }
            self.position_x += 0.5;
        } else if keyboard::is_key_pressed(ctx, KeyCode::Left) {
            if keyboard::is_mod_active(ctx, KeyMods::SHIFT) {
                self.position_x -= 4.5;
            }
            self.position_x -= 0.5;
        }

        let screen_w = graphics::screen_coordinates(ctx).w;
        if self.position_x < 0.0 {
            self.position_x += screen_w;
        } else if self.position_x > screen_w {
            self.position_x -= screen_w;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let winit::dpi::LogicalSize {width, height} = graphics::window(ctx).inner_size().to_logical(graphics::window(ctx).scale_factor());

        graphics::set_screen_coordinates(ctx, graphics::Rect { x: 0.0, y: 0.0, w: width, h: height })?;

        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());

        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            mint::Point2{x: self.position_x, y: height / 2.0},
            100.0,
            0.5,
            [1.0, 1.0, 1.0, 1.0].into(),
        )?;

        if self.attractors.is_empty() {
            self.attractors.push(
                Attractor::new(
                    mint::Point2 {x: 400.0, y: 300.0},
                    150.0,
                    50.0,
                    true,
                )
            );
            self.attractors.push(
                Attractor::new(
                    mint::Point2 {x: 600.0, y: 300.0},
                    150.0,
                    50.0,
                    true,
                )
            );
        }

        for i in 0..self.attractors.len() {
            let attractor = self.attractors.get_mut(i).unwrap();
            attractor.rotate(1.0);
            attractor.draw(ctx, &self.screen_ctx)?;
        }

        graphics::draw(ctx, &circle, graphics::DrawParam::default())?;

        if SHOW_FRAMERATE {
            graphics::draw(ctx, &graphics::Text::new(self.frame_rate.as_str()), graphics::DrawParam::default().dest(mint::Point2 {x: 13.0, y: 37.0 }))?;
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
}

pub fn main() -> ggez::GameResult {

    let mut cb = ggez::ContextBuilder::new("untitled_game", "Sky C");
    cb = cb.window_setup(conf::WindowSetup {
        title: "Untitled Game".to_owned(),
        samples: conf::NumSamples::Four,
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

    let state = MainState::new()?;
    event::run(ctx, event_loop, state)
}

fn get_current_ms() -> u64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_secs() * 1000 + since_the_epoch.subsec_nanos() as u64 / 1_000_000
}