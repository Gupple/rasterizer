use crate::draw;
use crate::canvas;
use ndarray::arr2;

pub struct Line {
    a: ndarray::Array2::<f32>,
    b: ndarray::Array2::<f32>,
}

impl Line {
    pub fn new(a: (f32, f32, f32), b: (f32, f32, f32)) -> Line {
        let a = arr2(&[[a.0], [a.1], [a.2]]);
        let b = arr2(&[[b.0], [b.1], [b.2]]);

        Line {a: a, b: b}
    }
}

impl draw::Primitive for Line {
    fn translate(&mut self, vec: (f32, f32, f32)) {
        let vec = arr2(&[[vec.0], [vec.1], [vec.2]]);
        self.a += &vec;
        self.b += &vec;
    }

    fn scale(&mut self, factor: f32) {
        self.b *= factor;
    }

    fn rotate(&mut self, axis: draw::Axis, theta: f32) {
        let rot_matrix = draw::rotation_matrix(axis, theta);
        self.a = rot_matrix.dot(&self.a);
        self.b = rot_matrix.dot(&self.b);
    }

    fn draw(&self, canvas: &mut canvas::Canvas) {
        let a = (self.a[[0, 0]].round() as i32, self.a[[1, 0]].round() as i32);
        let b = (self.b[[0, 0]].round() as i32, self.b[[1, 0]].round() as i32);
        draw::draw_line(canvas, a, b);
    }
}
