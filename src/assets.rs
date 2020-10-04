use tetra::audio::Sound;
use tetra::graphics::text::VectorFontBuilder;
use tetra::graphics::Texture;
use tetra::Context;

pub struct Assets {
    pub player1_texture: Texture,
    pub player2_texture: Texture,
    pub ball_texture: Texture,
    pub score_font: VectorFontBuilder,
    pub paddle_hit_fx: Sound,
    pub goal_scored_fx: Sound,
    pub game_start_fx: Sound,
    pub game_win_fx: Sound,
    pub flawless_win_fx: Sound,
}

impl Assets {
    pub fn load(ctx: &mut Context) -> tetra::Result<Assets> {
        Ok(Assets {
            player1_texture: Texture::new(ctx, "src/resources/player1.png")?,
            player2_texture: Texture::new(ctx, "src/resources/player2.png")?,
            ball_texture: Texture::new(ctx, "src/resources/ball.png")?,
            score_font: VectorFontBuilder::new("src/resources/deja_vu_font.ttf")?,
            paddle_hit_fx: Sound::new("src/resources/paddle_hit.ogg")?,
            goal_scored_fx: Sound::new("src/resources/goal_scored.ogg")?,
            game_start_fx: Sound::new("src/resources/game_start.ogg")?,
            game_win_fx: Sound::new("src/resources/game_over.ogg")?,
            flawless_win_fx: Sound::new("src/resources/flawless_victory.ogg")?,
        })
    }
}
