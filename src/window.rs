use buffer::Buffer;

pub struct Window {
    height: u32,
    width: u32,
    buf: Buffer,
}

impl Window {
    pub fn new(height: u32, width: u32, buf: Buffer) -> Window {
        Window {
            height: height,
            width: width,
            buf: Buffer::new(),
        }
    }
}
