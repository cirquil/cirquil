use egui::{Shape, Rect, Vec2, Style, Pos2};

const GRID_STEP: f32 = 10.0;
const DOUBLED: f32 = GRID_STEP * 2.0;

pub trait ShapeExt: Sized {
    fn grid(viewport: Rect, offset: Vec2, style: &Style) -> Vec<Self>;
}

impl ShapeExt for Shape {
    fn grid(viewport: Rect, mut offset: Vec2, style: &Style) -> Vec<Self> {
        offset.x = offset.x.rem_euclid(GRID_STEP);
        offset.y = offset.y.rem_euclid(GRID_STEP);
        
        let mut top_left = viewport.min;
        top_left -= offset;
    
        let mut top_right = viewport.max;
        top_right.x -= top_right.x.rem_euclid(GRID_STEP) - top_left.x.rem_euclid(GRID_STEP);
        top_right.y = top_left.y;
    
        let mut bottom_right = top_right;
        bottom_right.y += GRID_STEP;
    
        let mut bottom_left = top_left;
        bottom_left.y += GRID_STEP;
    
        let size = viewport.size();
        let mut path = Vec::with_capacity((size.y / GRID_STEP * 2.0) as usize);
        let points_q = (size.y / DOUBLED) as isize;
        
        for _ in 0..=points_q {
            path.extend_from_slice(&[top_left, top_right, bottom_right, bottom_left]);
            top_left.y += DOUBLED;
            top_right.y += DOUBLED;
            bottom_right.y += DOUBLED;
            bottom_left.y += DOUBLED;
        }
        
        Self::dotted_line(&path, style.visuals.weak_text_color(), GRID_STEP, 1.0)
    }
}

pub fn nearest_grid_anchor(mut point: Pos2) -> Pos2 {
    point.x = (point.x / GRID_STEP).round() * GRID_STEP;
    point.y = (point.y / GRID_STEP).round() * GRID_STEP;
    
    point
}

pub fn grid_normalize_end(mut end: Pos2, start: Pos2) -> Pos2 {
    end = nearest_grid_anchor(end);
    if (start.x - end.x).abs() > (start.y - end.y).abs() {
        end.y = start.y;
        end.x -= 10.0 - start.x.rem_euclid(10.0)
    } else {
        end.x = start.x;
        end.y -= 10.0 - start.y.rem_euclid(10.0)
    }
    
    end
}
