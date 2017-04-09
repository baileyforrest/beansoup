extern crate ncurses;

use frame_buffer::FrameBuffer;

pub struct Screen {
    max_y: u32,
    max_x: u32,
}

// TODO: Handle screen size change
impl Screen {
    pub fn new() -> Screen {
        // Initialize ncurses
        ncurses::initscr();
        ncurses::keypad(ncurses::stdscr(), true);
        ncurses::noecho();

        // Get the screen bounds
        let mut max_y = 0;
        let mut max_x = 0;
        ncurses::getmaxyx(ncurses::stdscr(), &mut max_y, &mut max_x);
        assert!(max_y >= 0);
        assert!(max_x >= 0);

        Screen {
            max_y: max_y as u32,
            max_x: max_x as u32,
        }
    }

    pub fn render(&mut self, fb: &FrameBuffer) {
        let (fb_height, fb_width) = fb.dimensions();
        assert!(self.max_y == fb_height && self.max_x == fb_width);

        ncurses::mv(0, 0);
        ncurses::clear();

        for y in 0..self.max_y {
            for x in 0..self.max_x {
                ncurses::mvaddch(y as i32, x as i32, fb.get(x, y) as ncurses::chtype);
            }
        }

        // TODO: Remove
        ncurses::getch();
    }

    pub fn dimensions(&self) -> (u32, u32) {
        (self.max_y, self.max_x)
    }
}

impl Drop for Screen {
    fn drop(&mut self) {
        ncurses::endwin();
    }
}
