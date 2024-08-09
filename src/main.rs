mod cube3d;
mod plasma;

// Default values for screen width & height
pub static mut SCREEN_WIDTH: f32 = 80.0;
pub static mut SCREEN_HEIGHT: f32 = 40.0;

fn main() {
    let window = pancurses::initscr();

    /*
    Convert maximum screen height & width to f32
    to be used later in the calculations for cube3d and plasma.
    */
    unsafe {
        (SCREEN_HEIGHT, SCREEN_WIDTH) = (|| -> (f32, f32) {
            let (y, x) = window.get_max_yx();
            (y as f32, x as f32)
        })();
    }

    cube3d::run_cube_demo(&window);
    plasma::run_plasma_demo(&window);

    window.getch();
    pancurses::endwin();
}
