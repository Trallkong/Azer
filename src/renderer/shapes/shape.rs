use crate::renderer::shapes::rectangle::Rectangle;
use crate::renderer::shapes::triangle::Triangle;

pub enum Shape {
    Triangle(Triangle),
    Rectangle(Rectangle)
}