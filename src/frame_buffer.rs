pub struct FrameBuffer {
    height: u32,
    width: u32,
    buf: Vec<u32>,
}

impl FrameBuffer {
    pub fn new(height: u32, width: u32) -> FrameBuffer {
        let mut fb = FrameBuffer {
            width: 0,
            height: 0,
            buf: Vec::new(),
        };

        fb.set_dimensions(height, width);

        fb
    }

    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn set_dimensions(&mut self, height: u32, width: u32) {
        self.width = width;
        self.height = height;
        // TODO: Set to ' '
        //self.buf.resize((width * height) as usize, ' ' as u32);
        self.buf.resize((width * height) as usize, 'a' as u32);
    }

    pub fn get(&self, y: u32, x: u32) -> u32 {
        assert!(x < self.width);
        assert!(y < self.height);

        self.buf[(y * self.width + x) as usize]
    }
}
