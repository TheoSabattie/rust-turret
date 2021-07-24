use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color, Mesh, DrawMode, FillOptions, Rect, DrawParam};
use ggez::event::{self, EventHandler, MouseButton};
use glam::*;

struct Resources {
    player:Mesh,
    shot:Mesh,
    enemy:Mesh
}

impl Resources {
    fn new(ctx:&mut Context) -> Self{
        let mut enemy_pos = vec![Vec2::default(); 3];
        let angle = std::f32::consts::PI*2.0/3.0;

        for i in 0..3 {
            enemy_pos[i] = Vec2::new((angle * i as f32).cos(), (angle * i as f32).sin());
        }

        Self {
            player:Mesh::new_rectangle(ctx, DrawMode::Fill(FillOptions::default()), Rect::new(-15.0, -15.0, 100.0, 30.0), Color::RED).unwrap(),
            shot:Mesh::new_circle(ctx, DrawMode::Fill(FillOptions::default()), Vec2::default(), 15.0, 1.0, Color::YELLOW).unwrap(),
            enemy:Mesh::new_polygon(ctx, DrawMode::Fill(FillOptions::default()), &enemy_pos, Color::GREEN).unwrap()
        }
    }
}

struct Transform {
    position:Vec2,
    scale:Vec2,
    rotation:f32,
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
    let my_game = MyGame::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, my_game);
}

struct Shot {
    transform:Transform,
    velocity:Vec2,
}

impl Shot {
    fn do_action(&mut self){
        self.transform.position += self.velocity;
    }
}

impl Default for Shot {
    fn default() -> Self {
        Self {
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

trait Graphic {
    fn get_draw_params(&self) -> DrawParam {
        let transform = self.get_transform();

        DrawParam::default()
            .scale(transform.scale)
            .dest(transform.position)
            .rotation(transform.rotation)
    }

    fn get_transform(&self) -> &Transform;
}

impl Graphic for Shot {
    fn get_transform(&self) -> &Transform {
        &self.transform
    }
}

impl Graphic for Player {
    fn get_transform(&self) -> &Transform {
        &self.transform
    }
}

impl Player {
    fn do_action(&mut self, ctx:&Context){
        let mouse_position = ggez::input::mouse::position(ctx);
        let position = Vec2::new(mouse_position.x - self.transform.position.x, mouse_position.y - self.transform.position.y);
        self.transform.rotation = position.y.atan2(position.x)
    }
}



struct MyGame{
    player:Player,
    shots:Vec<Shot>,
    resources:Resources
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> Self {
        // Load/create resources such as images here.
        let (width, height) = graphics::drawable_size(_ctx);
        let mut player = Player::default();
        player.transform.position = Vec2::new(width / 2.0, height / 2.0);

        Self {
            player:player,
            shots:Vec::new(),
            resources:Resources::new(_ctx)
        }
    }

    fn player_shoot(&mut self){
        let mut shot = Shot::default();
        let player_transform = &self.player.transform;
        let direction = Vec2::new(player_transform.rotation.cos(), player_transform.rotation.sin());
        shot.transform.position = player_transform.position + direction * 100.0;
        shot.velocity = direction * 10.0;
        self.shots.push(shot);
    }
}

impl EventHandler<ggez::GameError> for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        self.player.do_action(&*_ctx);

        for i in 0..self.shots.len() {
            self.shots[i].do_action();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::WHITE);
        // Draw code here...

        graphics::draw(ctx, &self.resources.player, self.player.get_draw_params())?;
        
        for i in 0..self.shots.len() {
            graphics::draw(ctx, &self.resources.shot, self.shots[i].get_draw_params())?;
        }

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