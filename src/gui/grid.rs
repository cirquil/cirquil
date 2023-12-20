use eframe::emath::Rect;
use egui::{Color32, Painter, Shape};

const GRID_STEP: f32 = 10.0;
const DOUBLE_GRID_STEP: f32 = GRID_STEP * 2.0;

pub fn draw(rect: &Rect, painter: &Painter) {
    let mut tl = rect.min;

    let mut tr = rect.max;
    tr.x = (tr.x / GRID_STEP).round() * GRID_STEP + tl.x % GRID_STEP;
    tr.y = tl.y;

    let mut br = tr;
    br.y += GRID_STEP;

    let mut bl = tl;
    bl.y += GRID_STEP;

    let size = rect.size();
    let mut path = Vec::with_capacity((size.y / GRID_STEP * 2.0).round() as usize);
    let full_quantity = (size.y / DOUBLE_GRID_STEP).round() as isize;
    for _ in 0..=full_quantity {
        path.extend_from_slice(&[tl, tr, br, bl]);
        tl.y += DOUBLE_GRID_STEP;
        tr.y += DOUBLE_GRID_STEP;
        br.y += DOUBLE_GRID_STEP;
        bl.y += DOUBLE_GRID_STEP;
    }

    painter.add(Shape::dotted_line(&path, Color32::GRAY, GRID_STEP, 1.0));
}
