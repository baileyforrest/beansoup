mod screen;
mod frame_buffer;

use screen::Screen;
use frame_buffer::FrameBuffer;

fn main() {
    let mut screen = Screen::new();
    let (max_y, max_x) = screen.dimensions();
    let mut fb = FrameBuffer::new(max_x, max_y);
    screen.render(&fb);
}
