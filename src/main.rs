use macroquad::prelude::*;

struct Ship {
    pos: Vec2,
    texture: Texture2D
}

struct Bullet {
    pos: Vec2,
}

#[macroquad::main("Mactrotoid")]
async fn main() { 
    let player_speed: f32 = 5.0; 
    let player_texture = Texture2D::from_file_with_format(include_bytes!("Jump.png"), None);
    
    let mut ship = Ship {
        pos: Vec2::new(screen_width() / 2.0, 500.0),
        texture: player_texture
    };
    let mut bullets= Vec::new();
    let mut last_shot = get_time();
    let fire_rate = 0.5;

    
    loop {
        clear_background(WHITE);
        let current_time = get_time();
        draw_texture(&ship.texture, ship.pos.x, ship.pos.y, WHITE);

        if is_key_down(KeyCode::D) {
            ship.pos.x += player_speed;
        }
        if is_key_down(KeyCode::A) {
            ship.pos.x -= player_speed;
        }

        if  is_key_down(KeyCode::Space) && current_time - last_shot > fire_rate{
            bullets.push(Bullet{
                pos: ship.pos
            });
            last_shot = current_time;
        }

        // Render Bullets
        for bullet in bullets.iter_mut(){
            draw_rectangle(bullet.pos.x, bullet.pos.y, 5.0, 5.0, RED);
        }

        // Move Bullets
        for bullet in bullets.iter_mut(){
            bullet.pos.y -= 5.0;
        }



        next_frame().await
    }
}
