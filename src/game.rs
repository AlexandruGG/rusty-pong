use crate::assets::Assets;
use crate::entity::Entity;
use crate::settings::*;

use std::cmp::Ordering;
use tetra::graphics::text::Text;
use tetra::graphics::{self, Color};
use tetra::input::{self, Key};
use tetra::math::Vec2;
use tetra::{Context, State};

pub struct GameState {
    player1: Entity,
    player2: Entity,
    ball: Entity,
    game_score: Text,
    assets: Assets,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let assets = Assets::load(ctx)?;

        let player1_position = Vec2::new(
            16.0,
            (WINDOW_HEIGHT - assets.player1_texture.height() as f32) / 2.0,
        );
        let player2_position = Vec2::new(
            WINDOW_WIDTH - assets.player2_texture.width() as f32 - 16.0,
            (WINDOW_HEIGHT - assets.player2_texture.height() as f32) / 2.0,
        );
        let ball_position = Vec2::new(
            WINDOW_WIDTH / 2.0 - assets.ball_texture.width() as f32 / 2.0,
            WINDOW_HEIGHT / 2.0 - assets.ball_texture.height() as f32 / 2.0,
        );
        let ball_velocity = Vec2::new(-BALL_SPEED, 0.0);

        assets.game_start_fx.play(ctx)?;

        Ok(GameState {
            player1: Entity::new(assets.player1_texture.clone(), player1_position),
            player2: Entity::new(assets.player2_texture.clone(), player2_position),
            ball: Entity::with_velocity(assets.ball_texture.clone(), ball_position, ball_velocity),
            game_score: Text::new("0/0", assets.score_font.with_size(ctx, 64.0)?),
            assets,
        })
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        // Update player and ball position based on key pressed
        if input::is_key_down(ctx, Key::W) && self.player1.position.y > 0.0 {
            self.player1.position.y -= PADDLE_SPEED;
        }

        if input::is_key_down(ctx, Key::S)
            && self.player1.position.y + self.player1.height() < WINDOW_HEIGHT
        {
            self.player1.position.y += PADDLE_SPEED;
        }

        if input::is_key_down(ctx, Key::Up) && self.player2.position.y > 0.0 {
            self.player2.position.y -= PADDLE_SPEED;
        }

        if input::is_key_down(ctx, Key::Down)
            && self.player2.position.y + self.player2.height() < WINDOW_HEIGHT
        {
            self.player2.position.y += PADDLE_SPEED;
        }

        self.ball.position += self.ball.velocity;

        // Detect a paddle hitting the ball and update its velocity - AABB
        let player1_bounds = self.player1.bounds();
        let player2_bounds = self.player2.bounds();
        let ball_bounds = self.ball.bounds();
        let paddle_hit = if ball_bounds.intersects(&player1_bounds) {
            Some(&self.player1)
        } else if ball_bounds.intersects(&player2_bounds) {
            Some(&self.player2)
        } else {
            None
        };

        if let Some(paddle) = paddle_hit {
            self.assets.paddle_hit_fx.play(ctx)?;
            self.ball.velocity.x =
                -(self.ball.velocity.x + (BALL_ACC * self.ball.velocity.x.signum()));

            let offset = (paddle.centre().y - self.ball.centre().y) / paddle.height();
            self.ball.velocity.y += PADDLE_SPIN * -offset;
        }

        // Don't allow the ball to go off screen top and bottom
        if self.ball.position.y <= 0.0 || self.ball.position.y + self.ball.height() >= WINDOW_HEIGHT
        {
            self.ball.velocity.y = -self.ball.velocity.y;
        }

        // Closure which resets the ball to center
        let reset_ball = |game: &mut GameState, direction: f32| {
            game.ball.position = Vec2::new(
                WINDOW_WIDTH / 2.0 - game.ball.width() as f32 / 2.0,
                WINDOW_HEIGHT / 2.0 - game.ball.height() as f32 / 2.0,
            );
            game.ball.velocity = Vec2::new(direction * BALL_SPEED, 0.0);
        };

        // Handle Player 2 scoring
        if self.ball.position.x <= 0.0 {
            self.player2.score += 1;

            match self.player2.score.cmp(&WIN_SCORE) {
                Ordering::Less => {
                    self.assets.goal_scored_fx.play(ctx)?;
                    reset_ball(self, 1.00);
                    self.game_score
                        .set_content(format!("{}/{}", self.player1.score, self.player2.score));
                }
                Ordering::Equal => {
                    if self.player1.score == 0 {
                        self.assets.flawless_win_fx.play(ctx)?;
                    } else {
                        self.assets.game_win_fx.play(ctx)?;
                    }

                    self.game_score.set_content("Player 2 Wins!");
                }
                _ => {}
            }
        }

        // Handle Player 1 scoring
        if self.ball.position.x >= WINDOW_WIDTH {
            self.player1.score += 1;

            match self.player1.score.cmp(&WIN_SCORE) {
                Ordering::Less => {
                    self.assets.goal_scored_fx.play(ctx)?;
                    reset_ball(self, -1.00);
                    self.game_score
                        .set_content(format!("{}/{}", self.player1.score, self.player2.score));
                }
                Ordering::Equal => {
                    if self.player2.score == 0 {
                        self.assets.flawless_win_fx.play(ctx)?;
                    } else {
                        self.assets.game_win_fx.play(ctx)?;
                    }

                    self.game_score.set_content("Player 1 Wins!");
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));

        let score_width = self.game_score.get_bounds(ctx).unwrap().width;
        graphics::draw(
            ctx,
            &self.game_score,
            Vec2::new(WINDOW_WIDTH / 2.0 - score_width / 2.0, 0.0),
        );
        graphics::draw(ctx, &self.player1.texture, self.player1.position);
        graphics::draw(ctx, &self.player2.texture, self.player2.position);
        graphics::draw(ctx, &self.ball.texture, self.ball.position);

        Ok(())
    }
}
