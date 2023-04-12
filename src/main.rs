use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

type Vertex = (f32, f32, f32);

struct Square {
    vertices: [Vertex; 4],
}

impl Square {
    fn new(vertices: [Vertex; 4]) -> Square {
        Square {
            vertices: vertices
        }
    }

    fn rotate(&mut self, axis: Axis, angle: f32) {
        let rotation_matrix = get_rotation_mat(axis, angle);
        for vertex in &mut self.vertices {
            *vertex = mat_vec_mul(rotation_matrix, *vertex);
        }
    }

    fn draw(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color((255, 255, 255));
        let vertices: Vec<Vertex> = self.vertices.into_iter().map(translate_axes).collect();

        for vertex in &vertices {
            canvas.draw_point(Point::new(vertex.0 as i32, vertex.1 as i32)).unwrap();
        }
        for i in 1..3_usize {
            canvas.draw_line(
                Point::new(vertices[0].0 as i32, vertices[0].1 as i32),
                Point::new(vertices[i].0 as i32, vertices[i].1 as i32)
            ).unwrap();
            canvas.draw_line(
                Point::new(vertices[3].0 as i32, vertices[3].1 as i32),
                Point::new(vertices[i].0 as i32, vertices[i].1 as i32)
            ).unwrap();
        }
    }
}

enum Axis {
    X,
    Y,
    Z
}

fn translate_axes(vec: Vertex) -> Vertex {
    let mut vec = vec;
    vec.0 += 300.0;
    vec.1 += 300.0;
    vec.2 += 300.0;
    vec
}

fn get_rotation_mat(axis: Axis, angle: f32) -> [[f32; 3]; 3] {
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

fn mat_vec_mul(matrix: [[f32; 3]; 3], vec: Vertex) -> Vertex {
    let mut new_vec = (0.0, 0.0, 0.0);
    new_vec.0 = (vec.0 * matrix[0][0]) + (vec.1 * matrix[1][0]) + (vec.2 * matrix[2][0]);
    new_vec.1 = (vec.0 * matrix[0][1]) + (vec.1 * matrix[1][1]) + (vec.2 * matrix[2][1]);
    new_vec.2 = (vec.0 * matrix[0][2]) + (vec.1 * matrix[1][2]) + (vec.2 * matrix[2][2]);
    
    new_vec
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("cubo rotatorio garaio", 600, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;

    let mut squares = [
        Square::new([
            (-100.0, 100.0, 100.0),
            (100.0, 100.0, 100.0),
            (-100.0, -100.0, 100.0),
            (100.0, -100.0, 100.0),
        ]),
        Square::new([
            (-100.0, 100.0, -100.0),
            (100.0, 100.0, -100.0),
            (-100.0, -100.0, -100.0),
            (100.0, -100.0, -100.0),
        ]),
        Square::new([
            (-100.0, 100.0, 100.0),
            (100.0, 100.0, 100.0),
            (-100.0, 100.0, -100.0),
            (100.0, 100.0, -100.0),
        ]),
        Square::new([
            (100.0, -100.0, 100.0),
            (-100.0, -100.0, 100.0),
            (100.0, -100.0, -100.0),
            (-100.0, -100.0, -100.0),
        ]),
    ];
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
        for square in &mut squares {
            square.rotate(Axis::X, 0.001);
            square.rotate(Axis::Y, 0.001);
            square.rotate(Axis::Z, 0.001);
            square.draw(&mut canvas);
        }
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.present();
    }

    Ok(())
}