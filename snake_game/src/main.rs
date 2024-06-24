use piston_window;
mod snake;
mod draw;
mod game;

use piston_window::*;

use game::Game;
use draw::to_coord_u32;


fn main(){
    let (width, height) = (30, 30); 

    let mut window: PistonWindow = 
        WindowSettings::new(
            "Snake Game", 
            [to_coord_u32(width), to_coord_u32(height)]
        )
        .exit_on_esc(true)
        .build()
        .unwrap();
    
    let mut game = Game::new(width, height); 
    while let Some(event) = window.next(){ 
        if let Some(Button::Keyboard(key)) = event.press_args(){
            game.key_pressed(key); 
        }

        window.draw_2d(&event, |c, g, _| {
            clear([0.0, 0.0, 0.0, 1.0], g);
            game.draw(&c, g);
        });
        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}