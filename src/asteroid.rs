//!
//! Asteroid
//!

use ::SfmlVector2f;
use ::SfmlConvexShape;

use ::SfmlRenderTarget;
use ::SfmlRenderStates;

use ::SfmlDrawable;
use ::SfmlTransformable;


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
        let mut shape = SfmlConvexShape::new(10);

        shape.set_point(0, SfmlVector2f::new(-4.0 * scale,  1.0 * scale));
        shape.set_point(1, SfmlVector2f::new(-4.0 * scale, -2.0 * scale));
        shape.set_point(2, SfmlVector2f::new(-2.0 * scale, -3.0 * scale));
        shape.set_point(3, SfmlVector2f::new( 0.0 * scale, -5.0 * scale));
        shape.set_point(4, SfmlVector2f::new( 2.0 * scale, -1.0 * scale));
        shape.set_point(5, SfmlVector2f::new( 4.0 * scale,  2.0 * scale));
        shape.set_point(6, SfmlVector2f::new( 2.0 * scale,  4.0 * scale));
        shape.set_point(7, SfmlVector2f::new( 0.0 * scale,  5.0 * scale));
        shape.set_point(8, SfmlVector2f::new(-3.0 * scale,  4.0 * scale));
        shape.set_point(9, SfmlVector2f::new(-2.0 * scale,  2.0 * scale));

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
