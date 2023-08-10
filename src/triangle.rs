use super::*;
use min_max::*;

use std::{thread};

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub vertices: [Vector; 3],
}

impl Triangle {
    pub fn new(vertices: [Vector; 3]) -> Triangle {
        Triangle {
            vertices: vertices
        }
    }
    
    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        let mut vertices = self.vertices
            .map(offset_to_screen)
            .to_vec();
        
        let normal = cross_product(vertices[1] - vertices[0], vertices[2] - vertices[0]);
        let normal = normal / normal.magnitude();
        
        if dot_product(normal, vertices[0]) < 0.0 {
            let light_source = Vector::new(0.0, 0.0, -1.0);
            let light_source = light_source / light_source.magnitude();
            let dot = (dot_product(light_source, normal)*255.0) as u8;
            canvas.set_draw_color(Color::RGB(dot, dot, dot));

            vertices = vertices.into_iter()
                .map(projection)
                .map(convert_screen_space)
                .collect();
            
            // let min_x = min!(vertices[0].x as i32, vertices[1].x as i32, vertices[2].x as i32);
            // let min_y = min!(vertices[0].y as i32, vertices[1].y as i32, vertices[2].y as i32);
            // let max_x = max!(vertices[0].x as i32, vertices[1].x as i32, vertices[2].x as i32);
            // let max_y = max!(vertices[0].y as i32, vertices[1].y as i32, vertices[2].y as i32);

            // let mutex_points: Arc<Mutex<Vec<Point>>> = Arc::new(Mutex::new(Vec::new()));
            // let vertices: Arc<Vec<Vector>> = Arc::new(vertices);
            // let mut thread_handles: Vec<thread::JoinHandle<_>> = Vec::new();

            // let n_threads = 16;
            // for i in 0..n_threads {
            //     let vertices = Arc::clone(&vertices);
            //     let mutex_points = Arc::clone(&mutex_points);

            //     thread_handles.push(thread::spawn(move || {
            //         for x in (min_x+i..=max_x).step_by(n_threads as usize) {
            //             for y in min_y..=max_y {
            //                 let p = Vector::new(x as f32, y as f32, 0.0);
            //                 if Triangle::point_is_inside_triangle(&vertices, p) {
            //                     let mut points = mutex_points.lock().unwrap();
            //                     points.push(Point::new(p.x as i32, p.y as i32));
            //                 }
            //             }
            //         }
            //     }));
            // }
            // for thread_handle in thread_handles {
            //     thread_handle.join().unwrap();
            // }
            // for point in mutex_points.lock().unwrap().to_vec() {
            //     canvas.draw_point(point).unwrap();
            // }
            canvas.draw_line(
                Point::new(vertices[0].x as i32, vertices[0].y as i32),
                Point::new(vertices[1].x as i32, vertices[1].y as i32)
            ).unwrap();
            canvas.draw_line(
                Point::new(vertices[1].x as i32, vertices[1].y as i32),
                Point::new(vertices[2].x as i32, vertices[2].y as i32)
            ).unwrap();
            canvas.draw_line(
                Point::new(vertices[2].x as i32, vertices[2].y as i32),
                Point::new(vertices[0].x as i32, vertices[0].y as i32)
            ).unwrap();
        }
    }
    
    pub fn rotate(&mut self, axis: Axis, angle: f32) -> Triangle {
        let rotation_matrix = get_rotation_mat(axis, angle);
        for vertex in &mut self.vertices {
            *vertex = mat_vec_mul(rotation_matrix, *vertex);
        }
        *self
    }

    pub fn translate(&mut self, axis: Axis, distance: f32) -> Triangle {
        for vertex in &mut self.vertices {
            match axis {
                Axis::X => *vertex = *vertex + Vector::new(distance, 0.0, 0.0),
                Axis::Y => *vertex = *vertex + Vector::new(0.0, distance, 0.0),
                Axis::Z => *vertex = *vertex + Vector::new(0.0, 0.0, distance)
            }
        }
        *self
    }

    pub fn point_is_inside_triangle(vertices: &Vec<Vector>, p: Vector) -> bool {
        let ab = vertices[1] - vertices[0];
        let bc = vertices[2] - vertices[1];
        let ca = vertices[0] - vertices[2];

        let ap = p - vertices[0];
        let bp = p - vertices[1];
        let cp = p - vertices[2];

        let c1 = cross_product(ab, ap).z;
        let c2 = cross_product(bc, bp).z;
        let c3 = cross_product(ca, cp).z;
        (c1 >= 0.0 && c2 >= 0.0 && c3 >= 0.0) || (c1 <= 0.0 && c2 <= 0.0 && c3 <= 0.0)
    }
}