use ndarray::{arr2};
use crate::draw;
use crate::canvas;

#[derive(Debug)]
pub struct Tri {
  p1: ndarray::Array2::<f32>,
  p2: ndarray::Array2::<f32>,
  p3: ndarray::Array2::<f32>,
}

impl Tri {
    pub fn new(p1: (f32, f32, f32), p2: (f32, f32, f32), p3: (f32, f32, f32)) -> Tri {
        let p1 = arr2(&[[p1.0], [p1.1], [p1.2]]);
        let p2 = arr2(&[[p2.0], [p2.1], [p2.2]]);
        let p3 = arr2(&[[p3.0], [p3.1], [p3.2]]);

        Tri {p1: p1, p2: p2, p3: p3}
    }
}

impl draw::Primitive for Tri {
    
    fn translate(&mut self, vec: (f32, f32, f32)) {
        let vec = arr2(&[[vec.0], [vec.1], [vec.2]]);

        self.p1 += &vec;
        self.p2 += &vec;
        self.p3 += &vec;
    }

    fn scale(&mut self, factor: f32) {
        self.p1 *= factor;
        self.p2 *= factor;
        self.p3 *= factor;
    }

    fn rotate(&mut self, axis: draw::Axis, theta: f32) {
        let rot_matrix = match axis {
            draw::Axis::X => arr2(&[
                            [1.0, 0.0, 0.0], 
                            [0.0, theta.cos(), theta.sin()], 
                            [0.0, -theta.sin(), theta.cos()],
            ]),
            draw::Axis::Y => arr2(&[
                            [theta.cos(), 0.0, -theta.sin()], 
                            [0.0, 1.0, 0.0], 
                            [theta.sin(), 0.0, theta.cos()],
            ]),
            draw::Axis::Z => arr2(&[
                            [theta.cos(), -theta.sin(), 0.0], 
                            [theta.sin(), theta.cos(), 0.0], 
                            [0.0, 0.0, 1.0],
            ]),
        };
        self.p1 = rot_matrix.dot(&self.p1);
        self.p2 = rot_matrix.dot(&self.p2);
        self.p3 = rot_matrix.dot(&self.p3);
    }
    
    fn draw(&self, canvas: &mut canvas::Canvas) {

        let p1 = (self.p1[[0, 0]].round() as i32, self.p1[[1, 0]].round() as i32);
        let p2 = (self.p2[[0, 0]].round() as i32, self.p2[[1, 0]].round() as i32);
        let p3 = (self.p3[[0, 0]].round() as i32, self.p3[[1, 0]].round() as i32);

        draw::draw_line(canvas, p1, p2);
        draw::draw_line(canvas, p1, p3);
        draw::draw_line(canvas, p2, p3);
    }
}
