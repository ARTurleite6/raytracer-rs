pub mod ambient_light;
pub mod point_light;

use self::ambient_light::AmbientLight;
use self::point_light::PointLight;

#[derive(Debug, Clone)]
pub enum Light {
    Ambient(AmbientLight),
    Point(PointLight),
}
