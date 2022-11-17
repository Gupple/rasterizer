pub mod canvas;
pub mod draw;

use core::f32::consts::PI;
use crate::draw::tri;
use crate::draw::line;
use crate::draw::Primitive;

fn main() {
    let mut tri = tri::Tri::new((-120.0, 0.0, 0.0), (120.0, 0.0, 0.0), (0.0, 120.0, 0.0));
    let mut line = line::Line::new((-240.0, 0.0, 0.0), (-120.0, -120.0, 0.0));
    let mut canvas = canvas::Canvas::new();
    for i in 0..360 {
        tri.draw(&mut canvas);
        line.draw(&mut canvas);
        canvas.export(&format!("{i}"));
        canvas.clear();
        tri.rotate(draw::Axis::Z, 2.0 * PI / 360.0);
        line.rotate(draw::Axis::Z, 2.0 * PI / 360.0);
    }
}
