use macroquad::prelude::*;

struct Enemy {
    pos: Vec2,
    texture: Texture2D,
    bounce: bool,
    collision_rect: Rect,
    health: i32,
    is_dead: bool,
}

impl Enemy {
    fn new(x: f32, y: f32, texture: Texture2D, health: i32) -> Enemy {
        let pos = Vec2::new(x, y);
        let collision_rect = Rect::new(pos.x, pos.y, 32.0, 32.0);

        Enemy {
            pos,
            texture,
            bounce: false,
            collision_rect,
            health,
            is_dead: false,
        }
    }
}

struct Ship {
    pos: Vec2,
    texture: Texture2D,
}

struct Bullet {
    pos: Vec2,
    is_active: bool,
    collision_rect: Rect,
}

#[macroquad::main("Mactrotoid")]
async fn main() {
    println!("Screen Width: {}", screen_width());
    println!("Screen height: {}", screen_height());

    let mut score: f32 = 0.0;

    // Load game textures
    let player_texture: Texture2D =
        Texture2D::from_file_with_format(include_bytes!("Jump.png"), None);
    let enemy_texture: Texture2D =
        Texture2D::from_file_with_format(include_bytes!("Jump.png"), None);

    let player_speed: f32 = 5.0;
    let mut last_shot = get_time();
    let fire_rate = 0.5;

    let mut ship = Ship {
        pos: Vec2::new(screen_width() / 2.0, 500.0),
        texture: player_texture,
    };

    let mut bullets = Vec::new();

    let mut enemies = Vec::new();

    for i in 1..2 {
        enemies.push(Enemy::new(i as f32 * 20.0, 1.0, enemy_texture.clone(), 10));
    }

    loop {
        clear_background(WHITE);
        let score_text = format!("{}", score);

        draw_text(&score_text, 50.0, 50.0, 50.0, BLACK);

        let current_time = get_time();
        draw_texture(&ship.texture, ship.pos.x, ship.pos.y, WHITE);

        if is_key_down(KeyCode::D) {
            ship.pos.x += player_speed;
        }
        if is_key_down(KeyCode::A) {
            ship.pos.x -= player_speed;
        }

        if ship.pos.x >= screen_width() - ship.texture.width() {
            ship.pos.x = screen_width() - ship.texture.width();
        }
        if ship.pos.x <= 0.0 {
            ship.pos.x = 0.0;
        }

        if is_key_down(KeyCode::Space) && current_time - last_shot > fire_rate {
            bullets.push(Bullet {
                pos: ship.pos,
                is_active: true,
                collision_rect: Rect::new(ship.pos.x, ship.pos.y, 5.0, 5.0),
            });
            last_shot = current_time;
        }

        if is_key_down(KeyCode::L) && current_time - last_shot > fire_rate {
            enemies.push(Enemy::new(0.0, 20.0, enemy_texture.clone(), 20));
            last_shot = current_time;
        }

        // Render Bullets
        for bullet in bullets.iter_mut() {
            if bullet.is_active {
                draw_rectangle(bullet.pos.x, bullet.pos.y, 5.0, 5.0, RED);
            }
        }

        // Move Bullets
        for bullet in bullets.iter_mut() {
            if bullet.is_active {
                bullet.pos.y -= 5.0;
            }

            bullet.collision_rect.x = bullet.pos.x;
            bullet.collision_rect.y = bullet.pos.y;
        }

        // Render enemies
        for enemy in enemies.iter_mut() {
            draw_texture(&enemy.texture, enemy.pos.x, enemy.pos.y, RED);
        }

        // Enemy Movement
        for enemy in enemies.iter_mut() {
            if enemy.bounce == false {
                enemy.pos.x += 2.0;
            } else {
                enemy.pos.x -= 2.0
            }
            if enemy.pos.x >= screen_width() - enemy.texture.width() {
                enemy.bounce = true;
            } else if enemy.pos.x <= 0.0 {
                enemy.bounce = false
            }

            if enemy.health <= 0 {
                score += 10.0;
                enemy.is_dead = true
            }

            enemy.pos.y += 0.5;
            enemy.collision_rect.x = enemy.pos.x;
            enemy.collision_rect.y = enemy.pos.y;
        }

        // Checks for bullet and enemy collsisions
        for enemy in enemies.iter_mut() {
            for bullet in bullets.iter_mut() {
                if bullet.collision_rect.overlaps(&enemy.collision_rect) {
                    bullet.is_active = false;
                    enemy.health -= 5;
                }
            }
        }

        bullets.retain(|bullet| bullet.is_active == true);
        enemies.retain(|enemy| enemy.is_dead == false);
        next_frame().await
    }
}
