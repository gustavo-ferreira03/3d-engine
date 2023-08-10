use super::*;
use std::thread;

#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    pub triangles: [Triangle; 2],
}

impl Rectangle {
    pub fn new(top_left: Vector, bottom_right: Vector, invert: bool) -> Rectangle {
        let bottom_left: Vector;
        let top_right: Vector;
        if invert {
            bottom_left = Vector::new(top_left.x, bottom_right.y, bottom_right.z);
            top_right = Vector::new(bottom_right.x, top_left.y, top_left.z);
        }
        else {
            bottom_left = Vector::new(top_left.x, bottom_right.y, top_left.z);
            top_right = Vector::new(bottom_right.x, top_left.y, bottom_right.z);
        }
        
        Rectangle {
            triangles: [
                Triangle::new([
                    top_left,
                    bottom_right,
                    bottom_left,
                ]),
                Triangle::new([
                    top_left,
                    top_right,
                    bottom_right,
                ])
            ]
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        for triangle in self.triangles {
            triangle.draw(canvas);
        }
    }
    
    pub fn rotate(&mut self, axis: Axis, angle: f32) -> Rectangle {
        for triangle in &mut self.triangles {
            triangle.rotate(axis, angle);
        }
        *self
    }

    pub fn translate(&mut self, axis: Axis, distance: f32) -> Rectangle {
        for triangle in &mut self.triangles {
            triangle.translate(axis, distance);
        }
        *self
    }
}