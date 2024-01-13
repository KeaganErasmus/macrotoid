use macroquad::prelude::*;

struct Enemy {
    pos: Vec2,
    texture: Texture2D,
    bounce: bool
}

struct Ship {
    pos: Vec2,
    texture: Texture2D
}

struct Bullet {
    pos: Vec2,
}

#[macroquad::main("Mactrotoid")]
async fn main() {
    
    println!("Screen Width: {}", screen_width());
    println!("Screen height: {}", screen_height());


    let player_speed: f32 = 5.0; 
    let player_texture = Texture2D::from_file_with_format(include_bytes!("Jump.png"), None);
    let enemy_texture = Texture2D::from_file_with_format(include_bytes!("Jump.png"), None);
    let mut last_shot = get_time();
    let fire_rate = 0.5;
    
    let mut ship = Ship {
        pos: Vec2::new(screen_width() / 2.0, 500.0),
        texture: player_texture
    };

    let mut bullets= Vec::new();

    let mut enemies = Vec::new();

    for i in 1..2{
        enemies.push(Enemy {
            pos: Vec2::new(i as f32 * 20.0, 1.0),
            texture: enemy_texture.clone(),
            bounce: false
        });
    }
    
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

        if ship.pos.x >= screen_width() - ship.texture.width(){
            ship.pos.x = screen_width() - ship.texture.width();
        }
        if ship.pos.x <= 0.0{
            ship.pos.x = 0.0;
        }

        if  is_key_down(KeyCode::Space) && current_time - last_shot > fire_rate{
            bullets.push(Bullet{
                pos: ship.pos,
            });
            last_shot = current_time;
        }

        if  is_key_down(KeyCode::L) && current_time - last_shot > fire_rate{
            enemies.push(Enemy{
                pos: Vec2::new(0.0, 20.0),
                texture: enemy_texture.clone(),
                bounce: false
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

        bullets.retain(|bullet| bullet.pos.y > 0.0);
        

        // Render enemies
        for enemy in enemies.iter_mut(){
            draw_texture(&enemy.texture, enemy.pos.x, enemy.pos.y, RED);
        }

        // Enemy Movement
        for enemy in enemies.iter_mut(){
            if enemy.bounce == false{
                enemy.pos.x += 2.0;
            }else {
                enemy.pos.x -= 2.0
            }
            if enemy.pos.x >= screen_width() -  enemy.texture.width(){
                enemy.bounce = true;
            }else if enemy.pos.x <= 0.0{
                enemy.bounce = false
            }

            enemy.pos.y += 0.5;
        }

        next_frame().await
    }
}
