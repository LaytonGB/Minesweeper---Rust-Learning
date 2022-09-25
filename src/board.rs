use rand::Rng;
use std::{cmp::max, collections::HashSet, fmt};

#[derive(Copy, Clone, Debug)]
pub struct Square {
    x: usize,
    y: usize,
    is_flagged: bool,
    is_mine: bool,
    is_triggered: bool,
    adjascent_mines: usize,
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_triggered {
            if self.is_mine {
                write!(f, "☢")
            } else {
                write!(f, "{}", self.adjascent_mines)
            }
        } else if self.is_flagged {
            write!(f, "🏳")
        } else {
            write!(f, "▣")
        }
    }
}

#[derive(Debug)]
pub struct Board {
    pub mines: usize,
    pub triggered: usize,
    pub size: usize,
    pub width: usize,
    pub height: usize,
    grid: Vec<Vec<Square>>,
}

impl Board {
    pub fn new(width: usize, height: usize, mines: usize) -> Self {
        if mines >= width * height {
            panic!("Mines must be less than gridspace.");
        }
        let mut b = Board {
            width,
            height,
            mines,
            triggered: 0,
            size: width * height,
            grid: (0..height)
                .map(|y| {
                    (0..width)
                        .map(|x| Square {
                            x,
                            y,
                            is_flagged: false,
                            is_mine: false,
                            is_triggered: false,
                            adjascent_mines: 0,
                        })
                        .collect::<Vec<Square>>()
                })
                .collect::<Vec<Vec<Square>>>(),
        };
        let mut mine_coordinates: HashSet<(usize, usize)> = HashSet::with_capacity(mines);
        let mut rng = rand::thread_rng();
        while mine_coordinates.len() < mines {
            mine_coordinates.insert((rng.gen_range(0..width), rng.gen_range(0..height)));
        }
        for (x, y) in mine_coordinates {
            b.grid[y][x].is_mine = true;
        }
        b
    }

    fn get_all_adjascent(&self, x: usize, y: usize) -> Vec<Square> {
        let low_x: usize = if x > 0 { x - 1 } else { x };
        let low_y: usize = if y > 0 { y - 1 } else { y };
        let high_x: usize = if x < self.width - 1 { x + 2 } else { x + 1 };
        let high_y: usize = if y < self.height - 1 { y + 2 } else { y + 1 };
        let mut out = Vec::<Square>::with_capacity(8);
        for new_y in low_y..high_y {
            for new_x in low_x..high_x {
                if !(new_y == y && new_x == x) {
                    out.push(self.grid[new_y][new_x]);
                }
            }
        }
        out
    }

    fn count_surrounding_mines(&mut self, x: usize, y: usize) -> usize {
        let mut count = 0usize;
        let adj = self.get_all_adjascent(x, y);
        for s in adj {
            if s.is_mine {
                count += 1;
            }
        }
        self.grid[y][x].adjascent_mines = count;
        count
    }

    pub fn toggle_flag(&mut self, x: usize, y: usize) {
        if self.grid[y][x].is_triggered {
            println!("ERR: Cannot flag a triggered square");
        }
        self.grid[y][x].is_flagged = !self.grid[y][x].is_flagged;
    }

    //* Returns true if the most recently triggered square is mine */
    pub fn trigger(&mut self, x: usize, y: usize) -> bool {
        if self.grid[y][x].is_triggered {
            return false;
        }
        if self.grid[y][x].is_flagged {
            println!("ERR: Cannot trigger a flagged square");
            return false;
        }
        self.triggered += 1;
        self.grid[y][x].is_triggered = true;
        if self.grid[y][x].is_mine {
            return true;
        }
        if self.count_surrounding_mines(x, y) == 0 {
            let adj = self.get_all_adjascent(x, y);
            for s in adj {
                self.trigger(s.x, s.y);
            }
        }
        false
    }

    pub fn display(&self) {
        let number_width: usize = max(
            format!("{}", self.width).len(),
            format!("{}", self.height).len(),
        );
        let mut out = vec![Vec::<String>::with_capacity(self.width + 2); self.height + 2];
        out[0].push('▣'.to_string());
        out[0].push('║'.to_string());
        for x in 1..=self.width {
            out[0].push(x.to_string());
        }
        out[1].push('═'.to_string());
        out[1].push('╬'.to_string());
        for _ in 1..=self.width {
            out[1].push('═'.to_string());
        }
        for y in 1..=self.height {
            out[y + 1].push((y).to_string());
            out[y + 1].push('║'.to_string());
            for x in 0..self.width {
                out[y + 1].push(self.grid[y - 1][x].to_string());
            }
        }

        println!();
        for y in 0..out.len() {
            for x in 0..out[0].len() {
                if y == 1 {
                    print!("═{:═<number_width$}", out[y][x]);
                } else {
                    print!(" {:<number_width$}", out[y][x]);
                }
            }
            println!();
            for _ in 0..((number_width - 1) / 2) {
                println!(" {:>number_width$}", "║");
            }
        }
    }

    pub fn end_game(&mut self) {
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                if self.grid[y][x].is_flagged {
                    self.toggle_flag(x, y);
                }
                if !self.grid[y][x].is_triggered {
                    self.trigger(x, y);
                }
            }
        }
        self.display();
    }
}
