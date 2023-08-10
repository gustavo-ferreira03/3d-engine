use super::*;

use std::thread;

#[derive(Clone, Copy)]
pub struct Cube {
    pub faces: [Rectangle; 6]
}

impl Cube {
    pub fn new(center: Vector, size: f32) -> Cube {
        let front_square = Rectangle::new(
            Vector::new(size/2.0, -size/2.0, -size/2.0),
            Vector::new(-size/2.0, size/2.0, -size/2.0),
            false
        );
        let top_square = Rectangle::new(
            Vector::new(size/2.0, -size/2.0, size/2.0),
            Vector::new(-size/2.0, -size/2.0, -size/2.0),
            true
        );

        let side_square = Rectangle::new(
            Vector::new(size/2.0, size/2.0, -size/2.0),
            Vector::new(size/2.0, -size/2.0, size/2.0),
            false
        );

        let faces = [
            front_square,
            front_square.clone().rotate(Axis::X, 180.0),

            top_square,
            top_square.clone().rotate(Axis::Z, 180.0),

            side_square,
            side_square.clone().rotate(Axis::Y, 180.0),
        ].map(|mut face| 
            face
            .translate(Axis::X, center.x)
            .translate(Axis::Y, center.y)
            .translate(Axis::Z, center.z)
        );
        
        Cube { faces }
    }
    
    pub fn rotate(&mut self, axis: Axis, angle: f32) {
        let angle = angle.to_radians();
        for face in &mut self.faces {
            face.rotate(axis, angle);
        }
    }
    
    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        for face in self.faces {
            face.draw(canvas);
        }
    }
}