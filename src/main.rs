use std::fs;
use std::fs::File;
use std::sync::Arc;
use std::sync::Mutex;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

mod triangle;
mod rectangle;
mod cube;
use crate::triangle::*;
use crate::rectangle::*;
use crate::cube::*;

#[derive(Debug, Clone, Copy)]
pub struct Vector {
    x: f32,
    y: f32,
    z: f32,
}

impl Vector {
    fn new(x: f32, y: f32, z: f32) -> Vector {
        Vector { x, y, z }
    }

    fn from_matrix(mat: [[f32; 1]; 3]) -> Vector {
        Vector { x: mat[0][0], y: mat[1][0], z: mat[2][0] }
    }

    fn to_matrix(&self) -> [[f32; 1]; 3] {
        [
            [self.x],
            [self.y],
            [self.z]
        ]
    }

    fn magnitude(&self) -> f32 {
        (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt()
    }
}

impl std::ops::Add<Vector> for Vector {
    type Output = Vector;
    fn add(self, vec: Vector) -> Self::Output {
        Vector::new(
            self.x + vec.x,
            self.y + vec.y,
            self.z + vec.z
        )
    }
}

impl std::ops::Sub<Vector> for Vector {
    type Output = Vector;
    fn sub(self, vec: Vector) -> Self::Output {
        Vector::new(
            self.x - vec.x,
            self.y - vec.y,
            self.z - vec.z
        )
    }
}

impl std::ops::Div<f32> for Vector {
    type Output = Vector;
    fn div(self, n: f32) -> Self::Output {
        Vector::new(
            self.x / n,
            self.y / n,
            self.z / n
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Axis {
    X,
    Y,
    Z
}

fn offset_to_screen(vec: Vector) -> Vector {
    Vector::new(
        vec.x,
        vec.y,
        vec.z + 5.0
    )
}

fn projection(vec: Vector) -> Vector {
    let a = 600.0/600.0;
    let zfar: f32 = 100.0;
    let znear: f32 = 10.0;
    let f = f32::tan(90_f32.to_radians() / 2.0);

    let vec = Vector::new(
        ((a*f*vec.x))/vec.z,
        ((f*vec.y))/vec.z,
        vec.z
    );
    vec
}

fn convert_screen_space(vec: Vector) -> Vector {
    Vector::new(
        (vec.x + 1.0)*(0.5*600.0),
        (-vec.y + 1.0)*(0.5*600.0),
        vec.z
    )
}

fn get_rotation_mat(axis: Axis, angle: f32) -> [[f32; 3]; 3] {
    let angle = angle.to_radians();
    match axis {
        Axis::X => [
            [1.0, 0.0, 0.0],
            [0.0, angle.cos(), -(angle.sin())],
            [0.0, angle.sin(), angle.cos()],
        ],
        Axis::Y => [
            [angle.cos(), 0.0, angle.sin()],
            [0.0, 1.0, 0.0],
            [-(angle.sin()), 0.0, angle.cos()],
        ],
        Axis::Z => [
            [angle.cos(), -(angle.sin()), 0.0],
            [angle.sin(), angle.cos(), 0.0],
            [0.0, 0.0, 1.0],
        ],
    }
}

fn mat_vec_mul(matrix: [[f32; 3]; 3], vec: Vector) -> Vector {
    let vec = vec.to_matrix();
    let mut new_mat = [[0.0; 1]; 3];
    for i in 0..matrix.len() {
        for j in 0..vec[0].len() {
            for k in 0..matrix.len() {
                new_mat[i][j] += matrix[i][k] * vec[k][j];
            }
        }
    }
    Vector::from_matrix(new_mat)
}

pub fn cross_product(v1: Vector, v2: Vector) -> Vector {
    Vector::new(
        v1.y*v2.z - v1.z*v2.y,
        -(v1.x*v2.z - v1.z*v2.x),
        v1.x*v2.y - v1.y*v2.x
    )
}

pub fn dot_product(v1: Vector, v2: Vector) -> f32 {
    v1.x*v2.x + v1.y*v2.y + v1.z*v2.z
}

pub fn main() -> Result<(), String> {
    let obj_lines = fs::read_to_string("teapot.obj").unwrap();
    let obj_lines: Vec<&str> = obj_lines.split("\n").collect();

    let mut vertices = Vec::new();
    let mut triangles = Vec::new();
    for line in obj_lines {
        let parts: Vec<&str> = line.split(" ").collect();

        match parts[0] {
            "v" => {
                let coordinates: Vec<f32> = parts[1..].into_iter().map(|x| x.parse().unwrap()).collect();
                vertices.push(Vector::new(coordinates[0], coordinates[1], coordinates[2]));
            },
            "f" => {
                let vertices_index: Vec<usize> = parts[1..].into_iter().map(|x| x.parse().unwrap()).collect();
                triangles.push(Triangle::new([
                    vertices[vertices_index[0]-1],
                    vertices[vertices_index[1]-1],
                    vertices[vertices_index[2]-1]
                ]));
            }
            _ => {}
        }
    }

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("cubo", 600, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.clear();

        for triangle in &mut triangles {
            triangle.draw(&mut canvas);
            triangle.rotate(Axis::X, 5.0);
            triangle.rotate(Axis::Y, 5.0);  
            triangle.rotate(Axis::Z, 5.0);
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.present();
    }

    Ok(())
}