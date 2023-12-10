pub trait DrawContext {
    fn draw_line(&self);
    fn draw_dot(&self);
    fn draw_pin(&self);
}