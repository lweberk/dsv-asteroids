//!
//! Spaceship
//!
use ::SfmlVector2f;

use ::SfmlDrawable;
use ::SfmlConvexShape;
use ::SfmlRenderStates;
use ::SfmlRenderTarget;
use ::SfmlTransformable;


pub struct Ship<'s>
{
    shape: SfmlConvexShape<'s>,

    position: SfmlVector2f,
    rotation: f32,

    linear_vel:  SfmlVector2f,
    angular_vel: f32,
}


impl<'s> Ship<'s>
{
    pub fn new(x: f32, y: f32, rotation: f32) -> Self
    {
        let mut shape = SfmlConvexShape::new(3);
        shape.set_point(0, SfmlVector2f::new(-15.0, -15.0));
        shape.set_point(1, SfmlVector2f::new( 15.0, -15.0));
        shape.set_point(2, SfmlVector2f::new(  0.0,  15.0));

        Self {
            shape: shape,

            position:    SfmlVector2f::new(x, y),
            linear_vel:  SfmlVector2f::new(0.0, 0.0),
            rotation:    rotation,
            angular_vel: 0.0,
        }
    }

    pub fn update(&mut self)
    {
        self.position += self.linear_vel;
        self.rotation += self.angular_vel;
    }

    pub fn accelerate(&mut self, accel: f32)
    {
        let dir = self.rotation.to_radians();

        // how much 'x' and 'y' is affected by the acceleration given our current
        // heading/rotation/azimuth
        let delta = SfmlVector2f::new(
            accel * dir.cos(),
            accel * dir.sin(),
        );

        self.linear_vel += delta;
    }

    pub fn rotate(&mut self, rotation: f32)
    {
        self.angular_vel += rotation;
    }

    pub fn rotation(&self) -> f32
    {
        self.rotation
    }

    pub fn velocity(&self) -> &SfmlVector2f
    {
        &self.linear_vel
    }
}


impl<'s> SfmlDrawable for Ship<'s>
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
