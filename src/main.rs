//!
//!
//!
extern crate sfml;

use sfml::window::Key;
use sfml::window::Event;
use sfml::window::Style;

use sfml::graphics::Font;
use sfml::graphics::Text;
use sfml::graphics::Color;
use sfml::graphics::Drawable;
use sfml::graphics::ConvexShape;
use sfml::graphics::RenderTarget;
use sfml::graphics::RenderWindow;
use sfml::graphics::RenderStates;
use sfml::graphics::Transformable;

use sfml::system::Vector2f;

struct Ship<'s>
{
    shape: ConvexShape<'s>,

    position: Vector2f,
    rotation: f32,

    linear_vel:  Vector2f,
    angular_vel: f32,
}


impl<'s> Ship<'s>
{
    fn new(x: f32, y: f32, rotation: f32) -> Self
    {
        let mut shape = ConvexShape::new(3);
        shape.set_point(0, Vector2f::new(-15.0, -15.0));
        shape.set_point(1, Vector2f::new( 15.0, -15.0));
        shape.set_point(2, Vector2f::new(  0.0,  15.0));

        Self {
            shape: shape,

            position:    Vector2f::new(x, y),
            linear_vel:  Vector2f::new(0.0, 0.0),
            rotation:    rotation,
            angular_vel: 0.0,
        }
    }

    fn update(&mut self)
    {
        self.position += self.linear_vel;
        self.rotation += self.angular_vel;
    }

    fn accelerate(&mut self, accel: f32)
    {
        let dir = self.rotation.to_radians();

        // how much 'x' and 'y' is affected by the acceleration given our current
        // heading/rotation/azimuth
        let delta = Vector2f::new(
            accel * dir.cos(),
            accel * dir.sin(),
        );

        self.linear_vel += delta;
    }

    fn rotate(&mut self, rotation: f32)
    {
        self.angular_vel += rotation;
    }

    fn rotation(&self) -> f32
    {
        self.rotation
    }

    fn velocity(&self) -> &Vector2f
    {
        &self.linear_vel
    }
}


impl<'s> Drawable for Ship<'s>
{
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
            &'a self,
            target: &mut RenderTarget,
            _: RenderStates<'texture, 'shader, 'shader_texture>
        )
    {
        let mut shape = self.shape.clone();

        shape.set_position(self.position);
        shape.set_rotation(self.rotation - 90.0);

        target.draw(&shape);
    }
}


fn main()
{
    let mut window = RenderWindow::new(
        (800, 600),
        "Asteroids",
        Style::CLOSE,
        &Default::default()
    );

    window.set_vertical_sync_enabled(true);

    let font = Font::from_memory(include_bytes!("../assets/FreeSans.ttf"))
        .expect("Could not load font from file");

    let mut ship = Ship::new(200.0, 200.0, 0.0);

    while window.is_open()
    {
        while let Some(event) = window.poll_event()
        {
            match event
            {
                Event::Closed                               => { window.close(); },
                Event::KeyPressed { code: Key::Escape, .. } => { window.close(); },

                Event::KeyPressed { code: Key::Left, .. } => {
                    ship.rotate(-1.0);
                },

                Event::KeyPressed { code: Key::Right, .. } => {
                    ship.rotate(1.0);
                },

                Event::KeyPressed { code: Key::Up, .. } => {
                    ship.accelerate(1.0);
                },

                Event::KeyPressed { code: Key::Down, .. } => {
                    ship.accelerate(-1.0);
                },

                _ => ()
            }
        }

        ship.update();

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

        window.display();
    }
}
