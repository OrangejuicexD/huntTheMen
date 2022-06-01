use ggez;
use ggez::event;
use ggez::graphics;
use ggez::input::keyboard::{self, KeyCode};
use ggez::nalgebra as na;
use ggez::{Context, GameResult};
use rand::{self, thread_rng, Rng};

const CHARACTER_SIZE: f32 = 30.0;
const CHARACTER_SIZE_HALF : f32 =CHARACTER_SIZE *  0.5;

struct  MainState {
    player_pos: na::Point2<f32>,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> Self{
        let (screen_w, screen_h) = graphics::drawable_size(ctx);
        let(screen_w_half, screen_h_half) = (screen_w * 0.5, screen_h * 0.5);
        MainState  {
            player_pos : na::Point2::new(screen_w_half, screen_h_half),
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);

        let color = [0.0, 1.0, 0.0, 1.0].into();
        let player_rect = graphics::Rect::new(-CHARACTER_SIZE_HALF,-CHARACTER_SIZE_HALF,CHARACTER_SIZE,CHARACTER_SIZE);
        let player_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            player_rect,
            color)?;


        let mut draw_param = graphics::DrawParam::default();

        draw_param.dest = self.player_pos.into();
        graphics::draw(ctx, &player_mesh,draw_param)?;

        graphics::present(ctx)?;
        Ok(())
    }
}


fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("hunt_the_men", "Ramias");
    let (mut ctx,mut event_loop ) = cb.build()?;
    graphics::set_window_title(&ctx, "Hunt The Man");

    let mut state = MainState::new(&mut ctx);
    event::run(&mut ctx, &mut event_loop, &mut state);

    Ok(())
}
