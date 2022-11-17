use crate::canvas;
use ndarray::arr2;

pub enum Axis {
    X,
    Y,
    Z,
}

pub trait Primitive {
    fn translate(&mut self, vec: (f32, f32, f32));
    fn scale(&mut self, factor: f32);
    fn rotate(&mut self, axis: Axis, theta: f32);
    fn draw(&self, canvas: &mut canvas::Canvas);
}

fn draw_line(canvas: &mut canvas::Canvas, start: (i32, i32), end: (i32, i32)) {
    let mut dx = end.0 - start.0;
    let mut dy = end.1 - start.1; 

    let stepx = dx.signum();
    let stepy = dy.signum();

    let mut start = start;

    if dx != 0 && dy != 0 {
        let a: f64 = dy.into();
        let b: f64 = (-dx).into();
        let c: f64 = (-dy * start.0 + dx * start.1).into();

        while dx.abs() > 0 && dy.abs() > 0 {
            let (x, y) = start;
            canvas.place(x.try_into().unwrap(), 
                         y.try_into().unwrap(), 
                         (255, 0, 0),
                         );

            let d1 = (a * ((x + stepx) as f64) + b * (y as f64) + c).abs();
            let d2 = (a * (x as f64) + b * ((y + stepy) as f64) + c).abs();

            if d1.min(d2) == d1 {
                start = (x + stepx, y);
                dx -= stepx;
            } else {
                start = (x, y + stepy);
                dy -= stepy;
            }
        }
    }

    while dx != 0 {
        let (x, y) = start;
        canvas.place(x.try_into().unwrap(), 
                     y.try_into().unwrap(), 
                     (255, 0, 0),
                     );
        start = (x + stepx, y);
        dx -= stepx;
    }

    while dy != 0 {
        let (x, y) = start;
        canvas.place(x.try_into().unwrap(), 
                     y.try_into().unwrap(), 
                     (255, 0, 0),
                     );
        start = (x, y + stepy);
        dy -= stepy;
    }
}

fn rotation_matrix(axis: Axis, theta: f32) -> ndarray::Array2::<f32> {
    match axis {
        Axis::X => arr2(&[
                         [1.0, 0.0, 0.0], 
                         [0.0, theta.cos(), theta.sin()], 
                         [0.0, -theta.sin(), theta.cos()],
        ]),
        Axis::Y => arr2(&[
                         [theta.cos(), 0.0, -theta.sin()], 
                         [0.0, 1.0, 0.0], 
                         [theta.sin(), 0.0, theta.cos()],
        ]),
        Axis::Z => arr2(&[
                         [theta.cos(), -theta.sin(), 0.0], 
                         [theta.sin(), theta.cos(), 0.0], 
                         [0.0, 0.0, 1.0],
        ]),
    }
}

pub mod tri;
pub mod line;

