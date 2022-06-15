use ggez;
use ggez::event;
use ggez::graphics;
use ggez::input::keyboard::{self, KeyCode};
use ggez::nalgebra as na;
use ggez::{Context, GameResult};
use ggez::graphics::pipe::new;
use rand::{self, thread_rng, Rng};

const CHARACTER_SIZE: f32 = 30.0;
const CHARACTER_SIZE_HALF : f32 =CHARACTER_SIZE *  0.5;
const PLAYER_SPEED : f32 =  500.0;
const ENEMY_SPEED : f32 = 600.0;

fn clamp(value: &mut f32, low: f32, high: f32){
    if *value < low{
        *value = low;
    } else if *value > high {
        *value = high;
    }
}

fn randomize_vel(vec : &mut na::Vector2<f32>, x: f32, y: f32){
    let mut rng = thread_rng();
    vec.x = match rng.gen_bool(0.5) {
        true => x,
        false => -x,
    };
    vec.y = match rng.gen_bool(0.5) {
        true => y,
        false => -y,
    };
}

struct  MainState {
    player_pos: na::Point2<f32>,
    enemy1_pos: na::Point2<f32>,
    enemy2_pos: na::Point2<f32>,
    enemy1_vel: na::Vector2<f32>,
    enemy2_vel: na::Vector2<f32>,
    death_score: i32,

}

impl MainState {
    pub fn new(ctx: &mut Context) -> Self{
        let (screen_w, screen_h) = graphics::drawable_size(ctx);
        let(screen_w_half, screen_h_half) = (screen_w * 0.5, screen_h * 0.5);

        let mut  enemy1_vel = na::Vector2::new(0.0, 0.0);
        let mut  enemy2_vel = na::Vector2::new(0.0, 0.0);
        randomize_vel(&mut enemy1_vel, ENEMY_SPEED, ENEMY_SPEED);
        randomize_vel(&mut enemy2_vel, ENEMY_SPEED, ENEMY_SPEED);

        MainState  {
            player_pos : na::Point2::new(screen_w_half, screen_h_half),
            enemy1_pos : na::Point2::new(20.0,20.0 ),
            enemy2_pos : na::Point2::new(80.0,80.0 ),
            enemy1_vel: enemy1_vel,
            enemy2_vel: enemy2_vel,
            death_score: 0,
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt = ggez::timer::delta(ctx).as_secs_f32();

        //get the screen with and height for the screen border
        let screen_h = graphics::drawable_size(ctx).1;
        let screen_w = graphics::drawable_size(ctx).0;

        //to make the player move with w,a,s,d
        if keyboard::is_key_pressed(ctx, KeyCode::W){
            self.player_pos.y -= PLAYER_SPEED * dt;
        }
        if keyboard::is_key_pressed(ctx, KeyCode::S){
            self.player_pos.y += PLAYER_SPEED * dt;
        }
        if keyboard::is_key_pressed(ctx, KeyCode::A){
            self.player_pos.x -= PLAYER_SPEED * dt;
        }
        if keyboard::is_key_pressed(ctx, KeyCode::D){
            self.player_pos.x += PLAYER_SPEED * dt;
        }
        //enemy 1&2 moving
        self.enemy1_pos += self.enemy1_vel * dt;
        self.enemy2_pos += self.enemy2_vel * dt;

        //make the window of the game the border for the player
        clamp(&mut self.player_pos.y,CHARACTER_SIZE_HALF, screen_h-CHARACTER_SIZE_HALF);
        clamp(&mut self.player_pos.x,CHARACTER_SIZE_HALF, screen_w-CHARACTER_SIZE_HALF);

        //don't let enemy1 get into space
        if self.enemy1_pos.y < CHARACTER_SIZE_HALF{
            self.enemy1_vel.y = self.enemy1_vel.y.abs();
        }
        else if self.enemy1_pos.y > screen_h - CHARACTER_SIZE_HALF {

            self.enemy1_vel.y = -self.enemy1_vel.y.abs();
        }
        if self.enemy1_pos.x < CHARACTER_SIZE_HALF{
            self.enemy1_vel.x = self.enemy1_vel.x.abs();
        }
        else if self.enemy1_pos.x > screen_w - CHARACTER_SIZE_HALF {
            self.enemy1_vel.x = -self.enemy1_vel.x.abs();
        }

        //don't let enemy2 get into space
        if self.enemy2_pos.y < CHARACTER_SIZE_HALF{
            self.enemy2_vel.y = self.enemy2_vel.y.abs();
        }
        else if self.enemy2_pos.y > screen_h - CHARACTER_SIZE_HALF {

            self.enemy2_vel.y = -self.enemy2_vel.y.abs();
        }
        if self.enemy2_pos.x < CHARACTER_SIZE_HALF{
            self.enemy2_vel.x = self.enemy2_vel.x.abs();
        }
        else if self.enemy1_pos.x > screen_w - CHARACTER_SIZE_HALF {
            self.enemy2_vel.x = -self.enemy2_vel.x.abs();
        }

        //make the enemy1 touch not feel good
        let intersects_player1 = self.enemy1_pos - CHARACTER_SIZE_HALF
        < self.player_pos.x + CHARACTER_SIZE_HALF
        && self.enemy1_pos.x + CHARACTER_SIZE_HALF > self.player_pos.x - CHARACTER_SIZE_HALF
        && self.enemy1_pos.y - CHARACTER_SIZE_HALF < self.player_pos.y + CHARACTER_SIZE_HALF
        && self.enemy1_pos.y + CHARACTER_SIZE_HALF > self.player_pos.y - CHARACTER_SIZE_HALF;

        if(intersects_player1) {
            self.enemy1_vel.x = - self.enemy1_vel.x.abs();
            self.death_score += 1;
        }
        //make the enemy2 touch not feel good
        let intersects_player2 = self.enemy2_pos - CHARACTER_SIZE_HALF
            < self.player_pos.x + CHARACTER_SIZE_HALF
            && self.enemy2_pos.x + CHARACTER_SIZE_HALF > self.player_pos.x - CHARACTER_SIZE_HALF
            && self.enemy2_pos.y - CHARACTER_SIZE_HALF < self.player_pos.y + CHARACTER_SIZE_HALF
            && self.enemy2_pos.y + CHARACTER_SIZE_HALF > self.player_pos.y - CHARACTER_SIZE_HALF;

        if(intersects_player2) {
            self.enemy1_vel.x = - self.enemy1_vel.x.abs();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);

        //define what player should be drawn
        let green = [0.0, 1.0, 0.0, 1.0].into();
        let player_rect = graphics::Rect::new(-CHARACTER_SIZE_HALF,-CHARACTER_SIZE_HALF,CHARACTER_SIZE,CHARACTER_SIZE);
        let player_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            player_rect,
            green)?;

        //define what enemy should be drawn
        let red = [1.0, 0.0, 0.0, 1.0].into();
        let enemy_rect = graphics::Rect::new(-CHARACTER_SIZE_HALF,-CHARACTER_SIZE_HALF,CHARACTER_SIZE,CHARACTER_SIZE);
        let enemy_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            enemy_rect,
            red)?;


        let mut draw_param = graphics::DrawParam::default();

        //draw the player
        draw_param.dest = self.player_pos.into();
        graphics::draw(ctx, &player_mesh,draw_param)?;

        //draw the enemy1
        draw_param.dest = self.enemy1_pos.into();
        graphics::draw(ctx, &enemy_mesh,draw_param)?;

        //draw the enemy2
        draw_param.dest = self.enemy2_pos.into();
        graphics::draw(ctx, &enemy_mesh,draw_param)?;

        //create death text
        let death_text = graphics::Text::new(format!(
            "Death: {}", self.death_score
        ));

        //quick redo of the screen_w_half
        let screen_w = graphics ::drawable_size(ctx).0;
        let screen_w_half = screen_w * 0.5;

        //count deaths
        let mut death_pos = na::Point2::new(screen_w_half,40.0);
        let(death_text_w, death_text_h) = death_text.dimensions(ctx);
        death_pos -= na::Vector2::new(death_text_w as f32 *0.5, death_text_h as f32 * 0.5);
        draw_param.dest = death_pos.into();

        //draw the score
        graphics::draw(ctx, &death_text, draw_param);

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
