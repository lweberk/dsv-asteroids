//!
//! Asteroids
//!
extern crate sfml;

use sfml::system::Vector2f        as SfmlVector2f;
use sfml::graphics::Drawable      as SfmlDrawable;
use sfml::graphics::ConvexShape   as SfmlConvexShape;
use sfml::graphics::RenderStates  as SfmlRenderStates;
use sfml::graphics::RenderTarget  as SfmlRenderTarget;
use sfml::graphics::Transformable as SfmlTransformable;

mod ship;
mod asteroid;

pub use ship::Ship;
pub use asteroid::Asteroid;
