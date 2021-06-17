pub trait Renderer2D {
    fn flush();

    fn set_render_area(offset_x: usize, offset_y: usize, width: usize, height: usize);

    fn reset_render_area();

    fn plot(x: usize, y: usize, color: &Color);

    fn fill(color: &Color);

    fn fill_circle(x: usize, y: usize, radius: usize, opacity: u8, color: &Color);

    fn fill_rect(x: usize, y: usize, width: usize, height: usize, color: &Color);

    fn fill_gradient(
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        color1: &Color,
        color2: &Color,
    );

    fn outline_rect(x: usize, y: usize, width: usize, height: usize, color: &Color);

    fn line_horizontal(x: usize, y: usize, width: usize, color: &Color);

    fn line_vertical(x: usize, y: usize, width: usize, color: &Color);

    fn darken();

    // TODO: rename parameters
    fn blur(x: usize, y: usize, start_x: usize, start_y: usize, width: usize, height: usize);
}

#[allow(dead_code)]
struct SoftwareRenderer2D;

impl Renderer2D for SoftwareRenderer2D {}
