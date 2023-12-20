use eframe::emath::Rect;
use egui::{Color32, Painter, Shape};

const MESH_STEP: f32 = 10.0;
const DOUBLE_MESH_STEP: f32 = MESH_STEP * 2.0;

pub fn draw(rect: &Rect, painter: &Painter) {
    let mut tl = rect.min;

    let mut tr = rect.max;
    tr.x = (tr.x / MESH_STEP).round() * MESH_STEP + tl.x % MESH_STEP;
    tr.y = tl.y;

    let mut br = tr;
    br.y += MESH_STEP;

    let mut bl = tl;
    bl.y += MESH_STEP;

    let size = rect.size();
    let mut path = Vec::with_capacity((size.y / MESH_STEP * 2.0).round() as usize);
    let full_quantity = (size.y / DOUBLE_MESH_STEP).round() as isize;
    for _ in 0..=full_quantity {
        path.extend_from_slice(&[tl, tr, br, bl]);
        tl.y += DOUBLE_MESH_STEP;
        tr.y += DOUBLE_MESH_STEP;
        br.y += DOUBLE_MESH_STEP;
        bl.y += DOUBLE_MESH_STEP;
    }

    painter.add(Shape::dotted_line(&path, Color32::GRAY, MESH_STEP, 1.0));
}
