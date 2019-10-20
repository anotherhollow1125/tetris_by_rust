extern crate rand;

pub mod game {
    use rand::seq::SliceRandom;
    use std::collections::VecDeque;

    const NEXTS : u32 = 3;
    const SAFTY_FRAMES : u32 = 90;
    const INTERVAL : u32 = 20;
    const LEVEL_UP_LINE_NUM : u32 = 10;
    const MAX_GRAVITY_RECIP : u32 = 5;
    const SPEEDUP_RATE : f32 = 0.9;

    const SCORE_TABLE : [u32; 5] = [0, 100, 300, 600, 900];

    // const WHITE : [f32; 4] = [1.0 , 1.0 , 1.0 , 1.0];
    const GRAY  : [f32; 4] = [0.3 , 0.3 , 0.3 , 1.0];
    const TRANS : [f32; 4] = [0.0 , 0.0 , 0.0 , 0.0];
    const CYAN  : [f32; 4] = [0.0 , 1.0 , 1.0 , 1.0];
    const YELLOW: [f32; 4] = [1.0 , 1.0 , 0.0 , 1.0];
    const LIME  : [f32; 4] = [0.0 , 1.0 , 0.0 , 1.0];
    const RED   : [f32; 4] = [1.0 , 0.0 , 0.0 , 1.0];
    const BLUE  : [f32; 4] = [0.0 , 0.0 , 1.0 , 1.0];
    const ORANGE: [f32; 4] = [1.0 , 0.65, 0.0 , 1.0];
    const PURPLE: [f32; 4] = [0.5 , 0.0 , 0.5 , 1.0];

    struct Mino {
        shape: [[bool; 4]; 4],
        size : usize,
        color: [f32; 4],
        // kind : MinoKind,
    }

    // enum MinoKind {
    //     D, I, O, S, Z, J, L, T,
    // }

    static D: Mino = Mino {
        shape: [
            [false, false, false, false],
            [false, false, false, false],
            [false, false, false, false],
            [false, false, false, false],
        ],
        size : 4,
        color: TRANS,
        // kind : MinoKind::D,
    };

    static I: Mino = Mino {
        shape: [
            [false, false, false, false],
            [true , true , true , true ],
            [false, false, false, false],
            [false, false, false, false],
        ],
        size : 4,
        color: CYAN, // cyan
        // kind : MinoKind::I,
    };
    static O: Mino = Mino {
        shape: [
            [true , true , false, false],
            [true , true , false, false],
            [false, false, false, false],
            [false, false, false, false],
        ],
        size : 2,
        color: YELLOW, // yellow
        // kind : MinoKind::O,
    };
    static S: Mino = Mino {
        shape: [
            [false, true , true , false],
            [true , true , false, false],
            [false, false, false, false],
            [false, false, false, false],
        ],
        size : 3,
        color: LIME, // lime
        // kind : MinoKind::S,
    };
    static Z: Mino = Mino {
        shape: [
            [true , true , false, false],
            [false, true , true , false],
            [false, false, false, false],
            [false, false, false, false],
        ],
        size : 3,
        color: RED, // red
        // kind : MinoKind::Z,
    };
    static J: Mino = Mino {
        shape: [
            [true , false, false, false],
            [true , true , true , false],
            [false, false, false, false],
            [false, false, false, false],
        ],
        size : 3,
        color: BLUE, // blue
        // kind : MinoKind::J,
    };
    static L: Mino = Mino {
        shape: [
            [false, false, true , false],
            [true , true , true , false],
            [false, false, false, false],
            [false, false, false, false],
        ],
        size : 3,
        color: ORANGE, // orange
        // kind : MinoKind::L,
    };
    static T: Mino = Mino {
        shape: [
            [false, true , false, false],
            [true , true , true , false],
            [false, false, false, false],
            [false, false, false, false],
        ],
        size : 3,
        color: PURPLE, // purple
        // kind : MinoKind::T,
    };

    #[derive(Debug, Copy, Clone)]
    pub struct Block {
        filled   : bool,
        color    : [f32; 4],
        clearing : bool,
    }

    impl Block {
        fn new(filled: bool, color: [f32; 4]) -> Block {
            Block {
                filled, color,
                clearing: false,
            }
        }

        pub fn is_filled(&self) -> bool {
            self.filled
        }

        pub fn get_color(&self) -> [f32; 4] {
            self.color
        }

        pub fn is_clearing(&self) -> bool {
            self.clearing
        }
    }

    #[derive(Debug)]
    enum Dir {
        North, East, South, West,
    }

    impl Dir {
        fn next(&self) -> Dir {
            match self {
                Dir::North => Dir::East, Dir::East => Dir::South,
                Dir::South => Dir::West, Dir::West => Dir::North,
            }
        }

        fn pre(&self) -> Dir {
            match self {
                Dir::North => Dir::West, Dir::East => Dir::North,
                Dir::South => Dir::East, Dir::West => Dir::South,
            }
        }

        // SRS
        // url: https://tetrisch.github.io/main/srs.html
        fn next_sequences(&self, mino: &'static Mino) -> Vec<(i32, i32)> {
            // match mino.kind {
            match mino.size {
                /* MinoKind::T => match self {
                    Dir::East  => vec![( 0, -1), (-1, -1),
                                       ( 2,  0), ( 2, -1)],
                    Dir::South => vec![( 0,  1), ( 1,  1),
                                       (-2,  0), (-2,  1)],
                    Dir::West  => vec![( 0,  1), (-1,  1),
                                       ( 2,  0), ( 2,  1)],
                    Dir::North => vec![( 0, -1), ( 1, -1), // サイトはここだけ変だった...?
                                       (-2,  0), (-2, -1)],
                }, */
                // MinoKind::I => match self {
                4 => match self {
                    Dir::East  => vec![( 0, -2), ( 0,  1),
                                       ( 1, -2), (-2,  1)],
                    Dir::South => vec![( 0, -1), ( 0,  2),
                                       (-2, -1), ( 1,  2)],
                    Dir::West  => vec![( 0,  2), ( 0, -1),
                                       (-1,  2), ( 2, -1)],
                    Dir::North => vec![( 0, -2), ( 0,  1),
                                       ( 2,  1), (-1, -2)],
                },
                _ => {
                    let d = match self {
                        Dir::East | Dir::South => 1,
                        _ => -1,
                    };
                    match self {
                        Dir::East | Dir::West =>
                            vec![( 0, -d), (-1, -d),
                                 ( 2,  0), ( 2, -d)],
                        _  =>
                            vec![( 0,  d), ( 1,  d),
                                 (-2,  0), (-2,  d)],
                    }
                }
            }
        }

        fn pre_sequences(&self, mino: &'static Mino) -> Vec<(i32, i32)> {
            // match mino.kind {
            match mino.size {
                /* MinoKind::T => match self {
                    Dir::West  => vec![( 0,  1), (-1,  1),
                                       ( 2,  0), ( 2,  1)],
                    Dir::North => vec![( 0,  1), ( 1,  1),
                                       (-2,  0), (-2,  1)],
                    Dir::East  => vec![( 0, -1), (-1, -1),
                                       ( 2,  0), ( 2, -1)],
                    Dir::South => vec![( 0, -1), ( 1, -1),
                                       (-2,  0), (-2, -1)],
                }, */
                // MinoKind::I => match self {
                4 => match self {
                    Dir::West  => vec![( 0, -1), ( 0,  2),
                                       (-2, -1), ( 1,  2)],
                    Dir::North => vec![( 0,  2), ( 0, -1),
                                       (-1,  2), ( 2, -1)],
                    Dir::East  => vec![( 0,  1), ( 0, -2),
                                       ( 2,  1), (-1, -2)],
                    Dir::South => vec![( 0,  1), ( 0, -2),
                                       ( 1, -2), (-2,  1)],
                },
                _ => {
                    let d = match self {
                        Dir::West | Dir::North => 1,
                        _ => -1,
                    };
                    match self {
                        Dir::West | Dir::East =>
                            vec![( 0,  d), (-1,  d),
                                 ( 2,  0), ( 2,  d)],
                        _ =>
                            vec![( 0,  d), ( 1,  d),
                                 (-2,  0), (-2,  d)],
                    }
                }
            }
        }
    }

    struct ControlledMino {
        pos   : (i32, i32),
        mino  : &'static Mino,
        dir   : Dir,
    }

    impl ControlledMino {
        fn new(mino: &'static Mino) -> ControlledMino {
            ControlledMino {
                pos  : (0, 4),
                mino : mino,
                dir  : Dir::North,
            }
        }

        fn rend_mino(&self) -> Vec<Vec<bool>> {
            let size  = self.mino.size;
            if size < 1 { return vec![]; }
            let shape: [[bool; 4]; 4] = self.mino.shape;
            let mut method: Box<dyn FnMut(usize, usize) -> bool> = match self.dir {
                Dir::North => Box::new(|i, j| shape[       i][       j]),
                Dir::East  => Box::new(|i, j| shape[size-1-j][       i]),
                Dir::South => Box::new(|i, j| shape[size-1-i][size-1-j]),
                Dir::West  => Box::new(|i, j| shape[       j][size-1-i]),
            };
            (0..size).map(|i| {
                (0..size).map(|j| method(i, j)).collect()
            }).collect()
        }

        fn pos_verify(&mut self, field: &[[Block; 12]; 22]) -> bool {
            let r = self.rend_mino();
            let size = self.mino.size;
            for i in 0..size {
                for j in 0..size {
                    if !r[i][j] { continue; }

                    let (x, y) = self.pos;
                    // 範囲チェック
                    let (x, y) = ((i as i32 + x) as i32, (j as i32 + y) as i32);
                    if x < 0 || 21 < x || y < 0 || 11 < y { return false; }
                    let (x, y) = (x as usize, y as usize);
                    // 設置可能チェック
                    if field[x][y].filled { return false; }
                }
            }
            true
        }

        fn move_mino(&mut self, field: &[[Block; 12]; 22], i: i32, j: i32) -> bool {
            let pre = self.pos;
            self.pos = (self.pos.0 + i, self.pos.1 + j);
            if !self.pos_verify(field) {
                self.pos = pre;
                return false;
            }
            true
        }

        fn spin_right(&mut self, field: &[[Block; 12]; 22]) {
            self.dir = self.dir.next();
            let mino = self.mino;
            if !self.pos_verify(field) &&
                self.dir.next_sequences(mino).iter()
                .all(|(i, j)| !self.move_mino(field, *i, *j))
            {
                self.dir = self.dir.pre();
            }
        }

        fn spin_left(&mut self, field: &[[Block; 12]; 22]) {
            self.dir = self.dir.pre();
            let mino = self.mino;
            if !self.pos_verify(field) &&
                self.dir.pre_sequences(mino).iter()
                .all(|(i, j)| !self.move_mino(field, *i, *j))
            {
                self.dir = self.dir.next();
            }
        }
    }

    struct MinoGenerator {
        dropped: Vec<(usize, u32)>,
        count  : u32,
        rng    : rand::rngs::ThreadRng,
    }

    impl MinoGenerator {
        fn new() -> MinoGenerator {
            let dropped = (0..7).map(|i| (i as usize, 0) ).collect();

            MinoGenerator {
                dropped,
                count  : 0,
                rng    : rand::thread_rng(),
            }
        }

        fn generate(&mut self) -> &'static Mino {
            if self.dropped.iter().all(|m| m.1 >= self.count) { self.count += 1 };
            let cands = self.dropped.iter().filter(|m| m.1 < self.count).collect::<Vec<_>>();
            let m = match cands.choose(&mut self.rng) {
                Some(v) => v,
                None    => panic!("Error in Mino Generating"),
            };
            let i: usize = m.0;
            self.dropped[i].1 += 1;
            [&I, &O, &S, &Z, &J, &L, &T][i]
        }
    }

    enum DroppingState {
        Dropping,
        Interval(u32),
    }

    struct Gravity {
        reciprocal : u32,
        drop_limit : u32,
        fix_flag   : bool,
        // frames     : u32,
    }

    impl Gravity {
        fn new() -> Gravity {
            let reciprocal = 45;
            let drop_limit = 20*reciprocal;

            Gravity {
                reciprocal,
                drop_limit,
                fix_flag: false,
                // frames : 0,
            }
        }
    }

    enum Hold {
        Holding(&'static Mino),
        None
    }

    pub struct Game {
        field          : [[Block; 12]; 22],
        total_frames   : u32,
        current_frames : u32,
        contmino       : ControlledMino,
        holdmino       : Hold,
        holdflag       : bool,
        nextminos      : VecDeque<&'static Mino>,
        minogen        : MinoGenerator,

        // States
        score          : u32,
        clearlines     : u32,
        cleartargets   : [bool; 21],
        dropping       : DroppingState,
        game_over      : bool,
        gravity        : Gravity,
    }

    impl Game {
        pub fn new() -> Game {
            let mut field = [[Block::new(false, TRANS); 12]; 22];

            for i in 0..21 {
                field[i][0].filled  = true;
                // field[i][0].color   = TRANS;
                field[i][11].filled = true;
                // field[i][11].color  = TRANS;
            }

            for block in &mut field[21] {
                block.filled = true;
                block.color  = TRANS;
            }

            let mut minogen   = MinoGenerator::new();
            let mut nextminos = VecDeque::new();
            for _ in 0..(NEXTS+1) {
                nextminos.push_front(minogen.generate());
            }

            Game {
                field,
                total_frames: 0,
                current_frames: 0,
                contmino: ControlledMino::new(nextminos.pop_back().unwrap()),
                holdmino: Hold::None,
                holdflag: false,
                nextminos,
                minogen,

                score: 0,
                clearlines: 0,
                cleartargets: [false; 21],
                dropping: DroppingState::Dropping,
                game_over: false,
                gravity : Gravity::new(),
            }
        }

        pub fn get_score(&self) -> u32 {
            self.score
        }

        pub fn get_clearlines(&self) -> u32 {
            self.clearlines
        }

        pub fn is_gameover(&self) -> bool {
            self.game_over
        }

        pub fn can_use_hold(&self) -> bool {
            !self.holdflag
        }

        pub fn rend_field(&mut self) -> [[Block; 10]; 20] {

            let mut cln = self.field.clone();
            let mino = self.contmino.mino;
            let size = mino.size;

            if let DroppingState::Dropping = self.dropping {
                // ゴースト用の座標
                let pre_pos = self.contmino.pos;
                while self.contmino.move_mino(&self.field, 1, 0) {}
                let (g_x, g_y) = self.contmino.pos;
                self.contmino.pos = pre_pos;

                let (x, y) = self.contmino.pos;
                let r = self.contmino.rend_mino();
                for i in 0..size {
                    for j in 0..size {
                        if r[i][j] {
                            let (i, j) = (i as i32, j as i32);
                            let (x, y) = ((i+x) as usize, (j+y) as usize);
                            let (g_x, g_y) = ((i+g_x) as usize, (j+g_y) as usize);
                            cln[g_x][g_y] = Block::new(false, GRAY);
                            cln[x][y] = Block::new(true, mino.color);
                        }
                    }
                }
            }

            let mut res: [[Block; 10]; 20] = [[Block::new(false, TRANS); 10]; 20];
            for i in 1..21 {
                for j in 1..11 {
                    res[i-1][j-1] = cln[i][j];
                }
            }
            res
        }

        pub fn rend_next(&self, i: usize) -> Vec<Vec<Block>> {
            let m = match self.nextminos.get(i) {
                Some(&mino) => mino,
                None        => &D,
            };
            (0..4).map(|i| {
                (0..4).map(move |j| Block::new(m.shape[i][j], if m.shape[i][j] { m.color } else { TRANS })).collect()
            }).collect()
        }

        pub fn rend_hold(&self) -> Vec<Vec<Block>> {
            let m = match self.holdmino {
                Hold::Holding(mino) => mino,
                Hold::None          => &D,
            };
            (0..4).map(|i| {
                (0..4).map(move |j| Block::new(m.shape[i][j], if m.shape[i][j] { m.color } else { TRANS })).collect()
            }).collect()
        }

        pub fn get_interval_ratio(&self) -> f32 {
            match self.dropping {
                DroppingState::Interval(f) => (f as f32) / (INTERVAL as f32),
                DroppingState::Dropping    => 0.0,
            }
        }

        // 固定と落下開始にワンクッション入れるために分離
        fn drop_start(&mut self, mino: &'static Mino) -> bool {
            self.contmino = ControlledMino::new(mino);
            // memo: self.nextminos.pop_back().unwrap()
            // self.nextminos.push_front(self.minogen.generate());
            self.current_frames = 0;
            // self.gravity.frames = 0;
            self.contmino.pos_verify(&self.field)
        }

        fn lines_clear(&mut self) {
            let mut cls: usize = 0;
            for i in 0..21 {
                if self.field[i][1..11].iter().all(|b| b.filled) {
                    cls += 1;
                    self.cleartargets[i] = true;
                    for j in 1..11 {
                        self.field[i][j].clearing = true;
                    }
                }
            }
            self.score += SCORE_TABLE[if cls > 4 { 4 } else { cls }];
            self.clearlines += cls as u32;
            if cls > 0 && self.clearlines % LEVEL_UP_LINE_NUM == 0 {
                self.gravity.reciprocal = std::cmp::max(
                    (self.gravity.reciprocal as f32 * SPEEDUP_RATE).floor() as u32,
                    MAX_GRAVITY_RECIP
                );
            }
            self.dropping = DroppingState::Interval(0);
        }

        fn lines_close(&mut self) {
            for i in 0..21 {
                if self.cleartargets[i] {
                    for j in (1..(i+1)).rev() {
                        self.field[j] = self.field[j-1];
                    }
                    let mut new_line = [Block::new(false, TRANS); 12];
                    new_line[0].filled = true;
                    new_line[11].filled = true;
                    self.field[0] = new_line;
                }
            }
            self.cleartargets = [false; 21];
            // self.dropping = DroppingState::Dropping;
        }

        fn fix_mino(&mut self) {

            if self.contmino.move_mino(&self.field, 1, 0) {
                self.contmino.move_mino(&self.field, -1, 0);
                return;
            }

            let mino = self.contmino.mino;
            let size = mino.size;
            let (x, y) = self.contmino.pos;
            let r = self.contmino.rend_mino();
            for i in 0..size {
                for j in 0..size {
                    if r[i][j] {
                        let (x, y) = ((i as i32 + x) as usize, (j as i32 + y) as usize);
                        self.field[x][y] = Block::new(true, mino.color);
                    }
                }
            }
            // レンダリング回避のためにダミーミノを挿入
            // バグの原因かもしれない...
            self.contmino = ControlledMino::new(&D);
            self.gravity.fix_flag = false;
            self.lines_clear();
        }

        fn mino_down_with_score(&mut self) -> bool {
            let res = self.contmino.move_mino(&self.field, 1, 0);
            if res { self.score += 1 }
            res
        }

        pub fn clock(&mut self, keys: [bool; 7]) {
            if let DroppingState::Interval(f) = self.dropping {
                self.dropping = if f > INTERVAL {
                    self.lines_close();
                    let mino = self.nextminos.pop_back().unwrap();
                    self.game_over = !self.drop_start(mino);
                    self.holdflag  = false;
                    self.nextminos.push_front(self.minogen.generate());
                    DroppingState::Dropping
                } else {
                    DroppingState::Interval(f+1)
                };
                return;
            }

        // pub fn key_input(&mut self, keys: [bool; 7]) {
            /*
            keys[0] : A button
            keys[1] : B button
            keys[2] : hard drop
            keys[3] : down
            keys[4] : right
            keys[5] : left
            keys[6] : hold
             */
            if keys[0] || keys[1] { self.gravity.fix_flag = false; }

            // spin
            if keys[0] {
                self.contmino.spin_right(&self.field);
            } else if keys[1] {
                self.contmino.spin_left(&self.field);
            }

            // hard drop
            if keys[2] {
                while self.mino_down_with_score() {}
                self.fix_mino();
                // if !self.drop_start() { self.game_over = true; }
            }

            // move
            if keys[3] {
                self.mino_down_with_score();
            } else if keys[4] {
                let _ = self.contmino.move_mino(&self.field, 0, 1);
            } else if keys[5] {
                let _ = self.contmino.move_mino(&self.field, 0,-1);
            }

            if keys[6] && !self.holdflag {
                let mino = match self.holdmino {
                    Hold::Holding(mino) => mino,
                    Hold::None => {
                        self.nextminos.push_front(self.minogen.generate());
                        self.nextminos.pop_back().unwrap()
                    }
                };
                self.holdmino = Hold::Holding(self.contmino.mino);
                self.game_over = !self.drop_start(mino);
                self.holdflag = true;
            }

            // free fall
            if self.current_frames % self.gravity.reciprocal == 0 {

                let moved = self.contmino.move_mino(&self.field, 1, 0);

                let fix_cond = self.current_frames > SAFTY_FRAMES
                    && (self.gravity.fix_flag
                        || self.current_frames > self.gravity.drop_limit);

                if fix_cond && !moved {
                    self.fix_mino();
                // moved == true
                } else if self.contmino.move_mino(&self.field, 1, 0) {
                    self.contmino.move_mino(&self.field, -1, 0);
                } else {
                    self.gravity.fix_flag = true;
                }
            }

            self.total_frames += 1;
            self.current_frames += 1;
            // self.gravity.frames += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn new_game() {
        use super::game::Game;
        let _ = Game::new();
    }
}

