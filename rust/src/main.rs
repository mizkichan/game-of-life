#![feature(step_by)]
#![feature(type_ascription)]

extern crate rand;
extern crate pancurses;

use rand::Rng;
use pancurses::Window;
use pancurses::Input::Character;

type Matrix = Vec<Vec<bool>>;

trait State {
    fn new(width: usize, height: usize) -> Self;
    fn randomize(&mut self) -> &mut Self;
    fn finalize(&self) -> Self;
    fn next(&self) -> Self;
    fn print(&self, window: &Window);
}

impl State for Matrix {
    fn new(width: usize, height: usize) -> Self {
        let width = width * 2 + 2;
        let height = height * 2 + 2;
        vec![vec![false; width]; height]
    }

    fn randomize(&mut self) -> &mut Self {
        let mut rng = rand::thread_rng();
        let width = self[0].len();
        let height = self.len();

        for y in 1..height-1 {
            for x in 1..width-1 {
                self[y][x] = rng.gen();
            }
        }

        self
    }

    fn finalize(&self) -> Self {
        self.clone()
    }

    fn next(&self) -> Self {
        let width = self[0].len();
        let height = self.len();

        let mut next_state = self.clone();
        for y in 1..height-1 {
            for x in 1..width-1 {
                let mut livings = 0;
                    if self[y-1][x-1] { livings += 1; }
                    if self[y-1][x  ] { livings += 1; }
                    if self[y-1][x+1] { livings += 1; }
                    if self[y  ][x-1] { livings += 1; }
                    if self[y  ][x+1] { livings += 1; }
                    if self[y+1][x-1] { livings += 1; }
                    if self[y+1][x  ] { livings += 1; }
                    if self[y+1][x+1] { livings += 1; }

                if self[y][x] {
                    if livings != 2 && livings != 3 {
                        next_state[y][x] = false;
                    }
                } else {
                    if livings == 3 {
                        next_state[y][x] = true;
                    }
                }
            }
        }

        next_state
    }

    fn print(&self, window: &Window) {
        let width = self[0].len();
        let height = self.len();

        window.mv(0, 0);
        for y in (1..height-1).step_by(2) {
            for x in (1..width-1).step_by(2) {
                let ch = match (self[y][x], self[y+1][x], self[y][x+1], self[y+1][x+1]) {
                    (false, false, false, false) => " ",
                    (false, false, false, true ) => "▗",
                    (false, false, true , false) => "▝",
                    (false, false, true , true ) => "▐",
                    (false, true , false, false) => "▖",
                    (false, true , false, true ) => "▄",
                    (false, true , true , false) => "▞",
                    (false, true , true , true ) => "▟",
                    (true , false, false, false) => "▘",
                    (true , false, false, true ) => "▚",
                    (true , false, true , false) => "▀",
                    (true , false, true , true ) => "▜",
                    (true , true , false, false) => "▌",
                    (true , true , false, true ) => "▙",
                    (true , true , true , false) => "▛",
                    (true , true , true , true ) => "█",
                };
                window.addstr(ch);
            }
        }
        window.refresh();
    }
}

fn main() {
    let window = pancurses::initscr();
    window.nodelay(true);

    let width  = window.get_max_x() as usize;
    let height = window.get_max_y() as usize;

    let mut prev_state = (State::new(width, height): Matrix).randomize().finalize();

    loop {
        let state = prev_state.next();
        state.print(&window);
        prev_state = state;

        match window.getch() {
            Some(Character('q')) => break,
            _                    => (),
        }
    }

    pancurses::endwin();
}