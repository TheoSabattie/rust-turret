use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color, Mesh, DrawMode, FillOptions, Rect, DrawParam};
use ggez::event::{self, EventHandler, MouseButton};
use glam::*;

struct Resources {
    shield:Mesh,
    player:Mesh,
    shot:Mesh,
    enemy:Mesh
}

impl Resources {
    fn new(ctx:&mut Context) -> Self{
        let enemy_pos = vec![
            Vec2::new(-30.0,  30.0), 
            Vec2::new(-30.0,  -30.0),
            Vec2::new(60.0,  0.0)
        ];

        Self {
            shield:Mesh::new_circle(ctx, DrawMode::Fill(FillOptions::default()), Vec2::default(), SHIELD_RADIUS, 1.0, Color::new(0.5, 0.5, 0.5, 1.0)).unwrap(),
            player:Mesh::new_rectangle(ctx, DrawMode::Fill(FillOptions::default()), Rect::new(-15.0, -15.0, 100.0, 30.0), Color::RED).unwrap(),
            shot:Mesh::new_circle(ctx, DrawMode::Fill(FillOptions::default()), Vec2::default(), 15.0, 1.0, Color::YELLOW).unwrap(),
            enemy:Mesh::new_polygon(ctx, DrawMode::Fill(FillOptions::default()), &enemy_pos, Color::GREEN).unwrap()
        }
    }
}

#[derive(Clone)]
struct Transform {
    position:Vec2,
    scale:Vec2,
    rotation:f32,
}

struct Shield {
    transform:Transform,
    radius:f32,
}

impl Default for Transform {
    fn default() -> Self {
        Self { 
            position:Vec2::default(),
            scale:Vec2::new(1.0, 1.0),
            rotation:0.0,
        }
    }
}

fn main() {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let my_game = Game::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, my_game);
}

#[derive(Clone)]
struct Mobile {
    transform:Transform,
    velocity:Vec2,
}

#[derive(Clone)]
struct Enemy {
    transform:Transform,
    velocity:Vec2,
}

impl Mobile {
    fn do_action(&mut self, size:&Vec2) -> bool {
        move_with_velocity(&mut self.transform.position, &self.velocity);
        let position = &self.transform.position;

        if position.x < 0.0 || position.x > size.x || position.y < 0.0 || position.y > size.y {
            return true;
        }

        false
    }
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            velocity:Vec2::default(),
            transform:Transform::default(),
        }
    }
}

impl Enemy {
    fn do_action(&mut self, size:&Vec2, player_shield:&Shield){
        move_with_velocity(&mut self.transform.position, &self.velocity);

        let position = &self.transform.position;
        let mut velocity_changed = false;

        if position.x > size.x {
            self.velocity.x = -self.velocity.x.abs();
            velocity_changed = true;
        } else if position.x < 0.0 {
            self.velocity.x = self.velocity.x.abs();
            velocity_changed = true;
        }

        if position.y > size.y {
            self.velocity.y = -self.velocity.y.abs();
            velocity_changed = true;
        } else if position.y < 0.0 {
            self.velocity.y = self.velocity.y.abs();
            velocity_changed = true;
        }

        let shield_to_self = Vec2::new(player_shield.transform.position.x - position.x, player_shield.transform.position.y - position.y);

        if get_length(&shield_to_self) <= player_shield.radius {
            self.velocity *= -1.0;
            velocity_changed = true;
        }

        if velocity_changed {
            self.transform.rotation = self.velocity.y.atan2(self.velocity.x);
        }
    }
}

fn get_length(vec:&Vec2) -> f32 {
    (vec.x * vec.x + vec.y * vec.y).sqrt()
}

fn move_with_velocity(position:&mut Vec2, velocity:&Vec2){
    position.x += velocity.x;
    position.y += velocity.y;
}

impl Default for Mobile {
    fn default() -> Self {
        Mobile {
            transform:Transform::default(),
            velocity:Vec2::default()
        }
    }
}

struct Player {
    transform:Transform,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            transform:Transform::default(),
        }
    }
}

fn get_draw_params(transform:&Transform) -> DrawParam {
    DrawParam::default()
        .scale(transform.scale)
        .dest(transform.position)
        .rotation(transform.rotation)
}

impl Player {
    fn do_action(&mut self, ctx:&Context){
        let mouse_position = ggez::input::mouse::position(ctx);
        let position = Vec2::new(mouse_position.x - self.transform.position.x, mouse_position.y - self.transform.position.y);
        self.transform.rotation = position.y.atan2(position.x)
    }
}

const N_ENEMY:usize = 50;
const SHIELD_RADIUS:f32 = 50.0;

struct Game {
    shield:Shield,
    player:Player,
    shots:Vec<Mobile>,
    enemies:Vec<Enemy>,
    resources:Resources,
    size:Vec2,
}

impl Game {
    pub fn new(_ctx: &mut Context) -> Self {
        let (width, height) = graphics::drawable_size(_ctx);
        let mut player = Player::default();
        let size = Vec2::new(width, height);
        let center = size / 2.0;
        player.transform.position = center;
        let size = Vec2::new(width, height);

        Game {
            shield:Shield{transform:player.transform.clone(), radius:SHIELD_RADIUS},
            player:player,
            shots:Vec::new(),
            enemies:Self::create_enemies(&size, N_ENEMY),
            resources:Resources::new(_ctx),
            size:Vec2::new(width, height)
        }
    }

    fn create_enemies(size:&Vec2, n_enemy:usize) -> Vec<Enemy> {
        let mut enemies:Vec<Enemy> = vec![Enemy::default();n_enemy];

        for i in 0..n_enemy {
            let rotation_rand:f32 = rand::random();
            let width_rand:f32    = rand::random();
            let height_rand:f32   = rand::random();
            let rotation          = (rotation_rand * 4.0).floor() * std::f32::consts::PI / 2.0 + std::f32::consts::PI /4.0;
            enemies[i].transform.position = Vec2::new(width_rand * size.x, size.y * height_rand);
            enemies[i].transform.rotation = rotation;
            enemies[i].velocity = Vec2::new(rotation.cos() * 10.0, rotation.sin() * 10.0);
        }

        enemies
    }

    fn player_shoot(&mut self){
        let mut shot = Mobile::default();
        let player_transform = &self.player.transform;
        let direction = Vec2::new(player_transform.rotation.cos(), player_transform.rotation.sin());
        shot.transform.position = player_transform.position + direction * 100.0;
        shot.velocity = direction * 10.0;
        self.shots.push(shot);
    }
}

impl EventHandler<ggez::GameError> for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        self.player.do_action(&*_ctx);

        for i in (0..self.shots.len()).rev() {
            if self.shots[i].do_action(&self.size) {
                self.shots.remove(i);
            }
        }

        for i in 0..self.enemies.len() {
            self.enemies[i].do_action(&self.size, &self.shield);
        }

        for i in (0..self.shots.len()).rev() {
            let shot = &self.shots[i];

            for j in (0..self.enemies.len()).rev() {
                let enemy = &self.enemies[j];
                let shot_to_enemy = shot.transform.position - enemy.transform.position;

                if get_length(&shot_to_enemy) <= 40.0 {
                    self.shots.remove(i);
                    self.enemies.remove(j);
                    break;
                }
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::WHITE);
        
        graphics::draw(ctx, &self.resources.shield, get_draw_params(&self.shield.transform))?;

        for i in 0..self.enemies.len() {
            graphics::draw(ctx, &self.resources.enemy, get_draw_params(&self.enemies[i].transform))?;
        }

        for i in 0..self.shots.len() {
            graphics::draw(ctx, &self.resources.shot, get_draw_params(&self.shots[i].transform))?;
        }

        graphics::draw(ctx, &self.resources.player, get_draw_params(&self.player.transform))?;
        graphics::present(ctx)
    }

    fn mouse_button_down_event(&mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32) {
        match _button {
            MouseButton::Left => self.player_shoot(),
            _ => (),
        }
    }
}