const PALETTE: [char; 16] = [
    '.', ',', ';', '\'', '"', '<', '>', '/', '(', ')', '{', '}', '&', '%', '#', '@',
];

// Plasma effect function used and adapted from https://rosettacode.org/wiki/Plasma_effect
fn generate_plasma(plasma_vec: &mut [f32], screen_height: f32, screen_width: f32, t: f32) {
    if plasma_vec.len() < (screen_height * screen_height) as usize {
        panic!("Plasma vector has incorrect size!");
    }

    for y in 0..screen_height as i32 {
        for x in 0..screen_width as i32 {
            plasma_vec[(y * screen_height as i32 + x) as usize] = (128.0
                + (128.0 * f32::sin((x as f32 / 8.0) - f32::cos(t / 2.0)))
                + 128.0
                + (128.0 * f32::sin((y as f32 / 16.0) - f32::sin(t) * 2.0))
                + 128.0
                + (128.0
                    * f32::sin(
                        f32::sqrt(
                            (x as f32 - screen_width / 2.0) * (x as f32 - screen_width / 2.0)
                                + (y as f32 - screen_height / 2.0)
                                    * (y as f32 - screen_height / 2.0),
                        ) / 4.0,
                    ))
                + 128.0
                + (128.0
                    * f32::sin(
                        (f32::sqrt(x as f32 * x as f32 + y as f32 * y as f32) / 4.0)
                            - f32::sin(t / 4.0),
                    )))
                / 4.0;
        }
    }
}

fn draw_text(window: &pancurses::Window, screen_height: f32, screen_width: f32) {
    let msg = "H Y P N O T I Z E";

    window.attron(pancurses::A_BLINK | pancurses::A_BOLD);
    for y in (screen_height as i32 / 2 - 1)..(screen_height as i32 / 2) + 2 {
        for x in (screen_width as i32 / 2 - (msg.len()) as i32)
            ..(screen_width as i32 / 2 + msg.len() as i32)
        {
            window.mvaddch(y, x, ' ');
        }
    }

    window.mvaddstr(
        screen_height as i32 / 2,
        screen_width as i32 / 2 - (msg.len() as i32 / 2),
        msg,
    );
    window.mv(screen_height as i32 - 1, screen_width as i32 - 1);
    window.attroff(pancurses::A_BLINK | pancurses::A_BOLD);
}

fn draw_outro(window: &pancurses::Window, screen_height: f32) {
    // https://ascii-art.net/about.php
    let text = "
CUBE3D AND PLASMA DEMO

2024 T

-------------------------------------------------
00000000: 2020 2020 2020 2020 2020 2020 2020 5f5f
00000010: 205f 5f0a 2020 2020 2020 2020 2020 2020
00000020: 2c3b 3a3a 5c3a 3a5c 0a20 2020 2020 2020
00000030: 2020 202c 272f 2720 602f 2760 2f0a 2020
00000040: 2020 2020 5f5c 2c3a 2027 2e2c 2d27 2e2d
00000050: 273a 2e0a 2020 2020 202d 2e2f 2227 2020
00000060: 3a20 2020 203a 2020 3a5c 2f2c 0a20 2020
00000070: 2020 203a 3a2e 2020 2c3a 5f5f 5f5f 3b5f
00000080: 5f3b 203a 2d0a 2020 2020 2020 3a22 2020
00000090: 2820 2e60 2d2a 276f 2a27 2c29 3b0a 2020
000000a0: 2020 2020 205c 2e2e 2060 2060 2d2d 2d27
000000b0: 6027 202f 0a20 2020 2020 2020 2060 3a2e
000000c0: 5f2e 2e2d 2020 205f 2e27 0a20 2020 2020
000000d0: 2020 202c 3b20 202e 2020 2020 2060 2e0a
000000e0: 2020 2020 2020 202f 2227 7c20 7c20 2020
000000f0: 2020 2020 5c0a 2020 2020 2020 3a3a 2e20
00000100: 2920 3a20 2020 2020 2020 203a 0a20 2020
00000110: 2020 207c 2220 2820 2020 5c20 2020 2020
00000120: 2020 7c0a 2020 2020 2020 3a2e 285f 2c20
00000130: 203a 2020 2020 2020 203b 0a20 2020 2020
00000140: 2020 5c27 602d 275f 2f20 2020 2020 202f
00000150: 0a20 2020 2020 2020 2060 2e2e 2e20 2020
00000160: 2c20 5f2c 270a 2020 2020 2020 2020 207c
00000170: 2c7c 2020 3a20 7c0a 2020 2020 2020 2020
00000180: 207c 607c 2020 7c20 7c0a 2020 2020 2020
00000190: 2020 207c 2c7c 2020 7c20 7c0a 2020 2020
000001a0: 202c 2d2d 2e3b 607c 2020 7c20 272e 2e2d
000001b0: 2d2e 0a20 2020 202f 3b27 2022 2720 3b20
000001c0: 2027 2e2e 2d2d 2e20 2929 0a20 2020 205c
000001d0: 3a2e 5f5f 5f28 5f5f 5f20 2020 2920 2929
000001e0: 270a 2020 2020 2020 2020 2020 2053 5374
000001f0: 602d 272d 2727 0a0a 4f48 2059 4545 4148
00000200: 210a
";

    window.scrollok(true);
    window.mv(screen_height as i32 - 1, 0);
    for ch in text.chars() {
        window.addch(ch);
        std::thread::sleep(std::time::Duration::from_millis(20));
        window.refresh();
    }
    window.scrollok(false);
}

pub fn run_plasma_demo(window: &pancurses::Window) {
    // Use t as timer for determining when to stop the demo.
    let mut t: f32 = 0.0;

    // Use time now to determine start_time and current time to calculate
    // plasma for the simulation.
    let time_now = || -> std::time::Duration {
        std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .unwrap()
    };

    let start_time = time_now();

    let screen_width: f32;
    let screen_height: f32;
    unsafe {
        screen_width = crate::SCREEN_WIDTH;
        screen_height = crate::SCREEN_HEIGHT;
    }

    // This vector contains the 2D plasma inforamtion for each cell
    let mut plasma: Vec<f32> = vec![0.0; (screen_height * screen_width) as usize];

    while t < 10.0 {
        let now = (time_now() - start_time).as_secs_f32();

        generate_plasma(&mut plasma, screen_height, screen_width, now);

        for y in 0..screen_height as i32 {
            for x in 0..screen_width as i32 {
                window.mvaddch(
                    y,
                    x,
                    PALETTE[((plasma[(y * screen_height as i32 + x) as usize].round() as i32
                        + ((now * 100.0) as i32))
                        / PALETTE.len() as i32
                        % PALETTE.len() as i32) as usize],
                );
            }
        }

        draw_text(window, screen_height, screen_width);
        window.refresh();

        t += 0.001;
    }

    draw_outro(window, screen_height);
}
