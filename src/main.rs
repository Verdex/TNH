

mod curses;

use std::ffi::CString;

use curses::*;

fn main() {
    unsafe {
        initscr();
        let x = CString::new( "zap {}" ).unwrap();
        printw( x.as_ptr(), 77 ); 
        refresh();
        getch();
        endwin();
    }
}
