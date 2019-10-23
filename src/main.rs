extern crate piston_window;
extern crate find_folder;

extern crate tetris_lib;

use piston_window::*;
use tetris_lib::game::*;

fn main() {
    let mut window: PistonWindow = WindowSettings::new(
            "tetris",
            [500, 500]
        )
        .exit_on_esc(true)
        .build()
        .unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    let mut glyphs = window.load_font(assets.join("FiraSans-Regular.ttf")).unwrap();

    let mut game = Game::new();
    let mut button = [false; 7];
    let mut frames = 0;

    // window.set_lazy(true);
    while let Some(e) = window.next() {
        if let Some(_) = e.render_args() {
            const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
            const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
            const GRAY : [f32; 4] = [0.3, 0.3, 0.3, 1.0];

            window.draw_2d(&e, |c, g, device| {
                // Clear the screen.
                clear(WHITE, g);

                let c2 = c.trans(173.0, 98.0);
                let rect = [0.0, 0.0, 154.0, 304.0];
                rectangle(WHITE, rect, c2.transform, g);
                Rectangle::new_border(BLACK, 2.0).draw(rect, &c2.draw_state, c2.transform, g);

                // hold
                for (i, line) in game.rend_hold().iter().enumerate() {
                    for (j, block) in line.iter().enumerate() {
                        let square = rectangle::square(0.0, 0.0, 15.0);
                        let (x, y) = (130 + (i as u32)*15, 100 + (j as u32)*15);
                        let transform = c.transform.trans(y as f64, x as f64);
                        let color = if !block.is_filled() || game.can_use_hold() {
                            block.get_color().to_rgb()
                        } else {
                            GRAY
                        };
                        rectangle(color, square, transform, g);
                    }
                }

                // field
                for (i, line) in game.rend_field().iter().enumerate() {
                    for (j, block) in line.iter().enumerate() {
                        let square = rectangle::square(0.0, 0.0, 15.0);
                        let (x, y) = (100 + (i as u32)*15, 175 + (j as u32)*15);
                        let transform = c.transform.trans(y as f64, x as f64);
                        let mut color = block.get_color().to_rgb();
                        if block.is_clearing() { color[3] = 1.0 - game.get_interval_ratio(); }
                        rectangle(color, square, transform, g);
                    }
                }

                // nexts
                for k in 0..3 {
                    let size = if k == 0 { 10 } else { 8 };
                    for (i, line) in game.rend_next(2-k).iter().enumerate() {
                        for (j, block) in line.iter().enumerate() {
                            let square = rectangle::square(0.0, 0.0, size as f64);
                            let k = k as u32;
                            let (x, y) = ((120+k*42) + (i as u32)*size, 345 + (j as u32)*size);
                            let transform = c.transform.trans(y as f64, x as f64);
                            rectangle(block.get_color().to_rgb(), square, transform, g);
                        }
                    }
                }

                // moji
                let transform = c.transform.trans(80.0, 100.0);
                text::Text::new_color(BLACK, 32).draw(
                    "Hold",
                    &mut glyphs,
                    &c.draw_state,
                    transform, g
                ).unwrap();

                let transform = c.transform.trans(350.0, 100.0);
                text::Text::new_color(BLACK, 32).draw(
                    "Next",
                    &mut glyphs,
                    &c.draw_state,
                    transform, g
                ).unwrap();

                let transform = c.transform.trans(350.0, 270.0);
                text::Text::new_color(BLACK, 24).draw(
                    &format!("Score: {}", game.get_score()),
                    &mut glyphs,
                    &c.draw_state,
                    transform, g
                ).unwrap();
                let transform = c.transform.trans(350.0, 294.0);
                text::Text::new_color(BLACK, 24).draw(
                    &format!("Lines: {}", game.get_clearlines()),
                    &mut glyphs,
                    &c.draw_state,
                    transform, g
                ).unwrap();

                glyphs.factory.encoder.flush(device);
            });
        }

        if let Some(btn) = e.press_args() {
            match btn {
                Button::Keyboard(Key::Space)  => button[0] = true,
                Button::Keyboard(Key::Z)      => button[0] = true,
                Button::Keyboard(Key::X)      => button[1] = true,
                Button::Keyboard(Key::Up)     => button[2] = true,
                Button::Keyboard(Key::Down)   => button[3] = true,
                Button::Keyboard(Key::Right)  => button[4] = true,
                Button::Keyboard(Key::Left)   => button[5] = true,
                Button::Keyboard(Key::LShift) => button[6] = true,
                _                             => (),
            }
        }

        e.update(|u| {
            frames += 1;
            if frames % std::cmp::max((1.0 / (u.dt*60.0)) as u32, 1) != 0 {
                return;
            }

            if !game.is_gameover() {
                game.tick(button);
            }

            button = [false; 7];
        });
    }
}
