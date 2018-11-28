//!
//!
//!
extern crate rand;
extern crate sfml;
extern crate asteroids;

use rand::Rng;

use sfml::window::Key;
use sfml::window::Event;
use sfml::window::Style;

use sfml::graphics::Font;
use sfml::graphics::Text;
use sfml::graphics::Color;
use sfml::graphics::RenderTarget;
use sfml::graphics::RenderWindow;
use sfml::graphics::Transformable;

use sfml::system::Vector2f;

use asteroids::Ship;
use asteroids::Asteroid;


fn main()
{
    let mut rng = rand::thread_rng();

    let win_width  = 800;
    let win_height = 600;

    let mut window = RenderWindow::new(
        (win_width, win_height),
        "Asteroids",
        Style::CLOSE,
        &Default::default()
    );

    window.set_vertical_sync_enabled(true);

    let font = Font::from_memory(include_bytes!("../assets/FreeSans.ttf"))
        .expect("Could not load font from file");


    let mut ship      = Ship::new(win_width as f32 / 2.0, win_height as f32 / 2.0, 0.0);
    let mut asteroids = Vec::new();

    for _ in 0..rng.gen_range(1, 5)
    {
        let pos_x    = rng.gen_range(0.0, win_width  as f32);
        let pos_y    = rng.gen_range(0.0, win_height as f32);
        let vel_x    = rng.gen_range(-1.00,  1.00);
        let vel_y    = rng.gen_range(-1.00,  1.00);
        let rotation = rng.gen_range(-0.15,  0.15);
        let scale    = rng.gen_range(30.00, 100.00);

        let mut asteroid = Asteroid::new(pos_x, pos_y, Vector2f::new(vel_x, vel_y), rotation, scale);

        asteroids.push(asteroid);
    }

    while window.is_open()
    {
        let mut alive_asteroids = Vec::new();

        for asteroid in asteroids.drain(..)
        {
            let pos = asteroid.position();

            if pos.x < 0.0 || pos.x > win_width as f32 {
                continue;
            }

            if pos.y < 0.0 || pos.y > win_height as f32 {
                continue;
            }

            alive_asteroids.push(asteroid);
        }

        asteroids = alive_asteroids;

        while let Some(event) = window.poll_event()
        {
            match event
            {
                Event::Closed                               => { window.close(); },
                Event::KeyPressed { code: Key::Escape, .. } => { window.close(); },

                Event::KeyPressed { code: Key::Left, .. } => {
                    ship.rotate(-0.5);
                },

                Event::KeyPressed { code: Key::Right, .. } => {
                    ship.rotate(0.5);
                },

                Event::KeyPressed { code: Key::Up, .. } => {
                    ship.accelerate(0.5);
                },

                Event::KeyPressed { code: Key::Down, .. } => {
                    ship.accelerate(-0.5);
                },

                _ => ()
            }
        }

        ship.update();

        for asteroid in asteroids.iter_mut() {
            asteroid.update();
        }

        let ship_rotation = ship.rotation();
        let ship_velocity = ship.velocity();

        let mut text_angle    = Text::new(&format!("Rotation: {}", ship_rotation), &font, 24);
        let mut text_velocity = Text::new(&format!("Velocity: ({}, {})", ship_velocity.x, ship_velocity.y), &font, 24);

        text_angle.move_(Vector2f::new(0.0, 0.0));
        text_velocity.move_(Vector2f::new(0.0, 30.0));

        window.clear(Color::BLACK);

        window.draw(&text_angle);
        window.draw(&text_velocity);
        window.draw(&ship);

        for asteroid in asteroids.iter() {
            window.draw(asteroid);
        }

        window.display();
    }
}
