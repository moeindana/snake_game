extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};

use std::collections::LinkedList;

#[derive(Clone, PartialEq)]
enum Direction {
    Right, Left, Up, Down
}

struct Game {
    gl: GlGraphics,
    snake: Snake,
}

impl Game {
    fn render(&mut self, arg: &RenderArgs) {
        use graphics;
        let GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        self.gl.draw(arg.viewport(), |_c, gl|{
            graphics::clear(GREEN, gl);
        });

        self.snake.render(&mut self.gl, arg);
    }

    fn update(&mut self) {
        self.snake.update();
    }

    fn pressed(&mut self, btn: &Button) {
        let last_direction = self.snake.dir.clone();

        self.snake.dir = match btn {
            &Button::Keyboard(Key::Up)
                if last_direction != Direction::Down => Direction::Up,
            &Button::Keyboard(Key::Down)
                if last_direction != Direction::Up => Direction::Down,
            &Button::Keyboard(Key::Left)
                if last_direction != Direction::Right => Direction::Left,
            &Button::Keyboard(Key::Right)
                if last_direction != Direction::Left => Direction::Right,
            _ => last_direction
        }
    }
}

struct Snake {
    pos_x: i32,
    pos_y: i32,
    dir: Direction,
}

impl Snake {
    fn render(&mut self, gl: &mut GlGraphics, arg: &RenderArgs){
        use graphics;

        let RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = graphics::rectangle::square((self.pos_x * 20) as f64, (self.pos_y * 20) as f64, 20_f64);
        gl.draw(arg.viewport(), |c,gl| {
            let transform = c.transform;
            graphics::rectangle(RED, square, transform, gl);
        })
    }
    
    fn update(&mut self) {
        match self.dir {
            Direction::Left => self.pos_x -= 1,
            Direction::Right => self.pos_x += 1,
            Direction::Up => self.pos_y -= 1,
            Direction::Down => self.pos_y += 1,
        }
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: GlutinWindow = WindowSettings::new("Snake Game", [200, 200]).graphics_api(opengl).exit_on_esc(true).build().unwrap();

    let mut game = Game {
        gl: GlGraphics::new(opengl),
        snake: Snake {pos_x: 0, pos_y: 0, dir: Direction::Right},
    };


    let mut events = Events::new(EventSettings::new()).ups(8);
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            game.render(&args);
        }

        if let Some(args) = e.update_args(){
            game.update();
        }

        if let Some(args) = e.button_args(){
            if args.state == ButtonState::Press {
                game.pressed(&args.button);
            }
        }
    }
}
