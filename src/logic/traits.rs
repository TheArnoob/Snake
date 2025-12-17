pub trait DrawableOn {
    fn draw_text(&mut self, text: &str, color_rgb: (u8, u8, u8), x: usize, y: usize, size: f32);
    fn height(&self) -> usize;
    fn width(&self) -> usize;
    fn fill_rectangle(
        &mut self,
        size: (usize, usize),
        color_rgb: (u8, u8, u8),
        top_left: (usize, usize),
    );
}
