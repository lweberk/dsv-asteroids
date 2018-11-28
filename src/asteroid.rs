//!
//! Asteroid
//!
use ::RandRng;

use ::SfmlVector2f;
use ::SfmlConvexShape;
use ::SfmlRenderTarget;
use ::SfmlRenderStates;

use ::SfmlDrawable;
use ::SfmlTransformable;

use std::f32::consts::PI;


pub struct Asteroid<'s>
{
    shape: SfmlConvexShape<'s>,

    position: SfmlVector2f,
    rotation: f32,

    linear_vel:  SfmlVector2f,
    angular_vel: f32,
}

impl<'s> Asteroid<'s>
{
    pub fn new(x: f32, y: f32, velocity: SfmlVector2f, rotation: f32, scale: f32) -> Self
    {
        let mut rng = rand::thread_rng();
        let mut shape = SfmlConvexShape::new(120);

        let     raycount    = 120;
        let mut last_raylen = rng.gen_range(scale - (scale * 0.1), scale + (scale * 0.1));

        for i in 0..raycount
        {
            let raylen = rng.gen_range(last_raylen - (last_raylen * 0.1), last_raylen + (last_raylen * 0.1));
            last_raylen = raylen;

            let alpha = (360.0 / raycount as f32) * i as f32;

            let cos_alpha = (alpha * PI / 180.0).cos();
            let sin_alpha = (alpha * PI / 180.0).sin();

            let ray_x = (raylen * cos_alpha) - (0.0 * sin_alpha);
            let ray_y = (raylen * sin_alpha) + (0.0 * cos_alpha);

            shape.set_point(i, SfmlVector2f::new(ray_x, ray_y));
        }

        Self {
            shape: shape,

            position:    SfmlVector2f::new(x, y),
            linear_vel:  velocity,
            angular_vel: rotation,
            rotation:    0.0,
        }
    }

   pub fn update(&mut self)
    {
        self.position += self.linear_vel;
        self.rotation += self.angular_vel;
    }

    pub fn position(&self) -> SfmlVector2f
    {
        return self.position;
    }
}

impl<'s> SfmlDrawable for Asteroid<'s>
{
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
            &'a self,
            target: &mut SfmlRenderTarget,
            _: SfmlRenderStates<'texture, 'shader, 'shader_texture>
        )
    {
        let mut shape = self.shape.clone();

        shape.set_position(self.position);
        shape.set_rotation(self.rotation - 90.0);

        target.draw(&shape);
    }
}
