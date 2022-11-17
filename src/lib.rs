#[derive(Clone, Debug)]
pub struct Ceil {
    pub life: bool,
    pub change: bool,
}

pub struct PatternSeed {
    pub pattern: Pattern,
    pub x: usize,
    pub y: usize,
}

pub struct LineSeed {
    pub vertical: bool,
    pub length: usize,
    pub x: usize,
    pub y: usize,
}

pub enum Pattern {
    Block,
    Beehive,
    Loaf,
    Boat,
    Tub,

    Blinker,
    Toad,
    Beacon,
    Pulsar,
    PentaDecathlon,

    Glider,
    LightWeightSpaceShip,
    LWSS,
    MiddleWeightSpaceShip,
    MWSS,
    HeavyWeightSpaceShip,
    HWSS,
}

impl Pattern {
    /**
     * Still lifes
     * TODO rota
     */
    fn add_block(board: &mut Board, x0: usize, y0: usize) {
        let x1 = board.offset.right(x0);

        let y1 = board.offset.down(y0);

        board.world[x0][y0].life = true;
        board.world[x0][y1].life = true;

        board.world[x1][y0].life = true;
        board.world[x1][y1].life = true;
    }

    fn add_beehive(board: &mut Board, x0: usize, y0: usize) {
        let x1 = board.offset.right(x0);
        let x2 = board.offset.right(x1);
        let x3 = board.offset.right(x2);

        let y1 = board.offset.down(y0);
        let y2 = board.offset.down(y1);

        board.world[x0][y1].life = true;

        board.world[x1][y0].life = true;
        board.world[x1][y2].life = true;

        board.world[x2][y0].life = true;
        board.world[x2][y2].life = true;

        board.world[x3][y1].life = true;
    }

    fn add_loaf(board: &mut Board, x0: usize, y0: usize) {
        let x1 = board.offset.right(x0);
        let x2 = board.offset.right(x1);
        let x3 = board.offset.right(x2);

        let y1 = board.offset.down(y0);
        let y2 = board.offset.down(y1);
        let y3 = board.offset.down(y2);

        board.world[x0][y1].life = true;

        board.world[x1][y0].life = true;
        board.world[x1][y2].life = true;

        board.world[x2][y0].life = true;
        board.world[x2][y3].life = true;

        board.world[x3][y1].life = true;
        board.world[x3][y2].life = true;
    }

    fn add_boat(board: &mut Board, x0: usize, y0: usize) {
        let x1 = board.offset.right(x0);
        let x2 = board.offset.right(x1);

        let y1 = board.offset.down(y0);
        let y2 = board.offset.down(y1);

        board.world[x0][y0].life = true;
        board.world[x0][y1].life = true;

        board.world[x1][y0].life = true;
        board.world[x1][y2].life = true;

        board.world[x2][y1].life = true;
    }

    fn add_tub(board: &mut Board, x0: usize, y0: usize) {
        let x1 = board.offset.right(x0);
        let x2 = board.offset.right(x1);

        let y1 = board.offset.down(y0);
        let y2 = board.offset.down(y1);

        board.world[x0][y1].life = true;

        board.world[x1][y0].life = true;
        board.world[x1][y2].life = true;

        board.world[x2][y1].life = true;
    }

    /**
     * Oscillators
     */
    fn add_blinker(board: &mut Board, x0: usize, y0: usize) {
        let x1 = board.offset.right(x0);
        let x2 = board.offset.right(x1);

        let y1 = board.offset.down(y0);

        board.world[x0][y1].life = true;

        board.world[x1][y1].life = true;

        board.world[x2][y1].life = true;
    }

    fn add_toad(board: &mut Board, x0: usize, y0: usize) {
        let x1 = board.offset.right(x0);
        let x2 = board.offset.right(x1);
        let x3 = board.offset.right(x2);

        let y1 = board.offset.down(y0);

        board.world[x0][y1].life = true;

        board.world[x1][y0].life = true;
        board.world[x1][y1].life = true;

        board.world[x2][y0].life = true;
        board.world[x2][y1].life = true;

        board.world[x3][y0].life = true;
    }

    fn add_beacon(board: &mut Board, x0: usize, y0: usize) {
        let x1 = board.offset.right(x0);
        let x2 = board.offset.right(x1);
        let x3 = board.offset.right(x2);

        let y1 = board.offset.down(y0);
        let y2 = board.offset.down(y1);
        let y3 = board.offset.down(y2);

        board.world[x0][y0].life = true;
        board.world[x0][y1].life = true;
        board.world[x1][y0].life = true;

        board.world[x2][y3].life = true;
        board.world[x3][y2].life = true;
        board.world[x3][y3].life = true;
    }

    fn add_pulsar(board: &mut Board, x0: usize, y0: usize) {
        let x1 = board.offset.right(x0);
        let x2 = board.offset.right(x1);
        let x3 = board.offset.right(x2);
        let x4 = board.offset.right(x3);
        let x5 = board.offset.right(x4);
        let x6 = board.offset.right(x5);
        let x7 = board.offset.right(x6);
        let x8 = board.offset.right(x7);
        let x9 = board.offset.right(x8);
        let x10 = board.offset.right(x9);
        let x11 = board.offset.right(x10);
        let x12 = board.offset.right(x11);

        let y1 = board.offset.right(y0);
        let y2 = board.offset.right(y1);
        let y3 = board.offset.right(y2);
        let y4 = board.offset.right(y3);
        let y5 = board.offset.right(y4);
        let y6 = board.offset.right(y5);
        let y7 = board.offset.right(y6);
        let y8 = board.offset.right(y7);
        let y9 = board.offset.right(y8);
        let y10 = board.offset.right(y9);
        let y11 = board.offset.right(y10);
        let y12 = board.offset.right(y11);

        board.world[x0][y2].life = true;
        board.world[x0][y3].life = true;
        board.world[x0][y4].life = true;
        board.world[x0][y8].life = true;
        board.world[x0][y9].life = true;
        board.world[x0][y10].life = true;

        board.world[x2][y0].life = true;
        board.world[x2][y5].life = true;
        board.world[x2][y7].life = true;
        board.world[x2][y12].life = true;

        board.world[x3][y0].life = true;
        board.world[x3][y5].life = true;
        board.world[x3][y7].life = true;
        board.world[x3][y12].life = true;

        board.world[x4][y0].life = true;
        board.world[x4][y5].life = true;
        board.world[x4][y7].life = true;
        board.world[x4][y12].life = true;

        board.world[x5][y2].life = true;
        board.world[x5][y3].life = true;
        board.world[x5][y4].life = true;
        board.world[x5][y8].life = true;
        board.world[x5][y9].life = true;
        board.world[x5][y10].life = true;

        board.world[x7][y2].life = true;
        board.world[x7][y3].life = true;
        board.world[x7][y4].life = true;
        board.world[x7][y8].life = true;
        board.world[x7][y9].life = true;
        board.world[x7][y10].life = true;

        board.world[x8][y0].life = true;
        board.world[x8][y5].life = true;
        board.world[x8][y7].life = true;
        board.world[x8][y12].life = true;

        board.world[x9][y0].life = true;
        board.world[x9][y5].life = true;
        board.world[x9][y7].life = true;
        board.world[x9][y12].life = true;

        board.world[x10][y0].life = true;
        board.world[x10][y5].life = true;
        board.world[x10][y7].life = true;
        board.world[x10][y12].life = true;

        board.world[x12][y2].life = true;
        board.world[x12][y3].life = true;
        board.world[x12][y4].life = true;
        board.world[x12][y8].life = true;
        board.world[x12][y9].life = true;
        board.world[x12][y10].life = true;
    }

    fn add_penta_decathlon(board: &mut Board, x0: usize, y0: usize) {
        let x1 = board.offset.right(x0);
        let x2 = board.offset.right(x1);
        let x3 = board.offset.right(x2);
        let x4 = board.offset.right(x3);

        let y1 = board.offset.right(y0);
        let y2 = board.offset.right(y1);
        let y3 = board.offset.right(y2);
        let y4 = board.offset.right(y3);
        let y5 = board.offset.right(y4);
        let y6 = board.offset.right(y5);
        let y7 = board.offset.right(y6);
        let y8 = board.offset.right(y7);
        let y9 = board.offset.right(y8);
        let y10 = board.offset.right(y9);
        let y11 = board.offset.right(y10);

        board.world[x0][y1].life = true;
        board.world[x0][y2].life = true;
        board.world[x0][y9].life = true;
        board.world[x0][y10].life = true;

        board.world[x1][y0].life = true;
        board.world[x1][y3].life = true;
        board.world[x1][y8].life = true;
        board.world[x1][y11].life = true;

        board.world[x2][y0].life = true;
        board.world[x2][y3].life = true;
        board.world[x2][y8].life = true;
        board.world[x2][y11].life = true;

        board.world[x3][y0].life = true;
        board.world[x3][y3].life = true;
        board.world[x3][y8].life = true;
        board.world[x3][y11].life = true;

        board.world[x4][y1].life = true;
        board.world[x4][y2].life = true;
        board.world[x4][y9].life = true;
        board.world[x4][y10].life = true;
    }

    /**
     * Spaceships
     */
    fn add_glider(board: &mut Board, x0: usize, y0: usize) {
        let x1 = board.offset.right(x0);
        let x2 = board.offset.right(x1);

        let y1 = board.offset.down(y0);
        let y2 = board.offset.down(y1);

        board.world[x0][y0].life = true;

        board.world[x1][y1].life = true;
        board.world[x1][y2].life = true;

        board.world[x2][y0].life = true;
        board.world[x2][y1].life = true;
    }

    fn add_light_weight_space_ship(board: &mut Board, x0: usize, y0: usize) {
        let x1 = board.offset.right(x0);
        let x2 = board.offset.right(x1);
        let x3 = board.offset.right(x2);
        let x4 = board.offset.right(x3);

        let y1 = board.offset.right(y0);
        let y2 = board.offset.right(y1);
        let y3 = board.offset.right(y2);

        board.world[x0][y0].life = true;
        board.world[x0][y2].life = true;

        board.world[x1][y3].life = true;

        board.world[x2][y3].life = true;

        board.world[x3][y0].life = true;
        board.world[x3][y3].life = true;

        board.world[x4][y1].life = true;
        board.world[x4][y2].life = true;
        board.world[x4][y3].life = true;
    }

    fn add_middle_weight_space_ship(board: &mut Board, x0: usize, y0: usize) {
        let x1 = board.offset.right(x0);
        let x2 = board.offset.right(x1);
        let x3 = board.offset.right(x2);
        let x4 = board.offset.right(x3);
        let x5 = board.offset.right(x4);

        let y1 = board.offset.right(y0);
        let y2 = board.offset.right(y1);
        let y3 = board.offset.right(y2);
        let y4 = board.offset.right(y3);

        board.world[x0][y1].life = true;
        board.world[x0][y3].life = true;

        board.world[x1][y0].life = true;

        board.world[x2][y0].life = true;
        board.world[x2][y4].life = true;

        board.world[x3][y0].life = true;

        board.world[x4][y0].life = true;
        board.world[x4][y3].life = true;

        board.world[x5][y0].life = true;
        board.world[x5][y1].life = true;
        board.world[x5][y2].life = true;
    }

    fn add_heavy_weight_space_ship(board: &mut Board, x0: usize, y0: usize) {
        let x1 = board.offset.right(x0);
        let x2 = board.offset.right(x1);
        let x3 = board.offset.right(x2);
        let x4 = board.offset.right(x3);
        let x5 = board.offset.right(x4);
        let x6 = board.offset.right(x5);

        let y1 = board.offset.right(y0);
        let y2 = board.offset.right(y1);
        let y3 = board.offset.right(y2);
        let y4 = board.offset.right(y3);

        board.world[x0][y1].life = true;
        board.world[x0][y3].life = true;

        board.world[x1][y0].life = true;

        board.world[x2][y0].life = true;
        board.world[x2][y4].life = true;

        board.world[x3][y0].life = true;
        board.world[x3][y4].life = true;

        board.world[x4][y0].life = true;

        board.world[x5][y0].life = true;
        board.world[x5][y3].life = true;

        board.world[x6][y0].life = true;
        board.world[x6][y1].life = true;
        board.world[x6][y2].life = true;
    }
}

pub struct Board {
    world: Vec<Vec<Ceil>>,
    width: usize,
    hight: usize,
    offset: Offset,
}

impl Board {
    pub fn add_seed(&mut self, pattern: Pattern, x: usize, y: usize) {
        match pattern {
            Pattern::Block => Pattern::add_block(self, x, y),
            Pattern::Beehive => Pattern::add_beehive(self, x, y),
            Pattern::Loaf => Pattern::add_loaf(self, x, y),
            Pattern::Boat => Pattern::add_boat(self, x, y),
            Pattern::Tub => Pattern::add_tub(self, x, y),
            Pattern::Blinker => Pattern::add_blinker(self, x, y),
            Pattern::Toad => Pattern::add_toad(self, x, y),
            Pattern::Beacon => Pattern::add_beacon(self, x, y),
            Pattern::Pulsar => Pattern::add_pulsar(self, x, y),
            Pattern::PentaDecathlon => Pattern::add_penta_decathlon(self, x, y),
            Pattern::Glider => Pattern::add_glider(self, x, y),
            Pattern::LightWeightSpaceShip | Pattern::LWSS => {
                Pattern::add_light_weight_space_ship(self, x, y)
            }
            Pattern::MiddleWeightSpaceShip | Pattern::MWSS => {
                Pattern::add_middle_weight_space_ship(self, x, y)
            }
            Pattern::HeavyWeightSpaceShip | Pattern::HWSS => {
                Pattern::add_heavy_weight_space_ship(self, x, y)
            }
        };
    }

    pub fn add_vertical_line(&mut self, x: usize, y: usize, length: usize) {
        let length = if length < self.hight {
            length
        } else {
            self.hight
        };
        for i in 0..length {
            self.world[x][i].life = true;
            self.world[y][i].life = true;
        }
    }

    pub fn add_horizontal_line(&mut self, x: usize, y: usize, length: usize) {
        let length = if length < self.width {
            length
        } else {
            self.width
        };
        for i in 0..length {
            self.world[i][x].life = true;
            self.world[i][y].life = true;
        }
    }

    pub fn draw(&self) {
        let mut p = String::from("");
        for hight in 0..self.hight {
            for width in 0..self.width {
                if self.world[width][hight].life {
                    p.push_str("O ");
                } else {
                    p.push_str(". ");
                }
            }
            p.push('\n');
        }

        print!("{}--{esc}[2J{esc}[1;1H", p, esc = 27 as char);
    }

    pub fn next_tick(&mut self) {
        for width in 0..self.width {
            for hight in 0..self.hight {
                let mut neighbor = 0;
                for i in [self.offset.left(width), width, self.offset.right(width)] {
                    for j in [self.offset.up(hight), hight, self.offset.down(hight)] {
                        if i == width && j == hight {
                            continue;
                        }
                        if self.world[i][j].life {
                            neighbor = neighbor + 1;
                        }
                    }
                }

                self.world[width][hight].change = if self.world[width][hight].life
                    && (neighbor < 2 || neighbor > 3)
                    || !self.world[width][hight].life && neighbor == 3
                {
                    true
                } else {
                    false
                }
            }
        }

        for width in 0..self.width {
            for hight in 0..self.hight {
                if self.world[width][hight].change {
                    self.world[width][hight].life = !self.world[width][hight].life
                }
            }
        }
    }
}

struct Offset {
    width: usize,
    hight: usize,
}

impl Offset {
    fn right(&self, point: usize) -> usize {
        if point + 1 == self.width {
            0
        } else {
            point + 1
        }
    }

    fn left(&self, point: usize) -> usize {
        match point.checked_sub(1) {
            Some(i) => i,
            None => self.width - 1,
        }
    }

    fn up(&self, point: usize) -> usize {
        match point.checked_sub(1) {
            Some(i) => i,
            None => self.hight - 1,
        }
    }

    fn down(&self, point: usize) -> usize {
        if point + 1 == self.hight {
            0
        } else {
            point + 1
        }
    }
}

pub mod world {
    use crate::Board;
    use crate::Ceil;
    use crate::Offset;

    pub fn init(width: usize, hight: usize) -> Board {
        // Vec<Vec<Ceil>>
        let block = Ceil {
            life: false,
            change: false,
        };

        // board.world[WIDTH][HIGHT]
        let world: Vec<Vec<Ceil>> = vec![vec![block.clone(); hight]; width];

        let board: Board = {
            Board {
                world,
                width,
                hight,
                offset: { Offset { width, hight } },
            }
        };

        board
    }
}
