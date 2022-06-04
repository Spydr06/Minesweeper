use std::{collections::HashSet,fmt::{self, Write}};
use crate::random;

pub type Position = (usize, usize);
pub enum OpenResult {
    Mine,
    NoMine(u8)
}

pub struct Minesweeper {
    width: usize,
    height: usize,
    open_fields: HashSet<Position>,
    mines: HashSet<Position>,
    flagged_fields: HashSet<Position>,
    lost: bool
}

impl fmt::Display for Minesweeper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = (x, y);
                f.write_str(
                    if !self.open_fields.contains(&pos) {
                        if self.lost && self.mines.contains(&pos) {
                            "💣 "
                        } else if self.flagged_fields.contains(&pos) {
                            "🚩 "
                        } else {
                            "🟪 "
                        }
                    } else if self.mines.contains(&pos) {
                        "💣 "
                    } else {
                        match self.neighboring_mines(pos) {
                            1 => "1️⃣ ",
                            2 => "2️⃣ ",
                            3 => "3️⃣ ",
                            4 => "4️⃣ ",
                            5 => "5️⃣ ",
                            6 => "6️⃣ ",
                            7 => "7️⃣ ",
                            8 => "8️⃣ ",
                            _ => "⬜ ",
                        }
                    }
                )?;
            }

            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl Minesweeper {
    pub fn new(width: usize, height: usize, mine_count: usize) -> Minesweeper {
        Minesweeper {
            width,
            height, 
            open_fields: HashSet::new(),
            mines: {
                let mut mines = HashSet::new();
                while mines.len() < mine_count {
                    mines.insert((random::range(0, width), random::range(0, height)));
                }
                mines
            },
            flagged_fields: HashSet::new(),
            lost: false
        }
    }

    fn iter_neighbors(&self, (x, y): Position) -> impl Iterator<Item = Position> {
        let width = self.width;
        let height = self.height;
        (x.max(1) - 1 ..= (x + 1).min(width - 1))
            .flat_map(move |i| {
                (y.max(1) - 1 ..= (y + 1).min(height - 1)).map(move |j| (i,j))
            })
            .filter(move |&pos| pos != (x, y))
    }

    fn neighboring_mines(&self, pos: Position) -> u8 {
        self
            .iter_neighbors(pos)
            .filter(|pos| self.mines.contains(pos))
            .count() as u8
    }

    pub fn open(&mut self, pos: Position) -> Option<OpenResult> {
        if self.open_fields.contains(&pos) {
            let mine_count = self.neighboring_mines(pos);
            let flag_count = self
                .iter_neighbors(pos)
                .filter(|neighbor| self.flagged_fields.contains(neighbor))
                .count() as u8;
            
            if mine_count == flag_count {
                for neighbor in self.iter_neighbors(pos) {
                    if !self.flagged_fields.contains(&neighbor) && !self.open_fields.contains(&neighbor) {
                        self.open(neighbor);
                    }
                }
            }

            return None;
        }

        if self.lost || self.flagged_fields.contains(&pos) {
            return None;
        }

        self.open_fields.insert(pos);
        Some(if self.mines.contains(&pos) {
                self.lost = true;
                OpenResult::Mine
            } else {
                let mine_coint = self.neighboring_mines(pos);

                if mine_coint == 0 {
                    for neighbor in self.iter_neighbors(pos) {
                        if !self.open_fields.contains(&neighbor) {
                            self.open(neighbor);
                        }
                    }
                }

                OpenResult::NoMine(0)
        })
    }

    pub fn toggle_flag(&mut self, pos: Position) {
        if self.lost || self.open_fields.contains(&pos) {
            return;
        }

        if self.flagged_fields.contains(&pos) {
            self.flagged_fields.remove(&pos);
        } else {
            self.flagged_fields.insert(pos);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::minesweeper::Minesweeper;

    #[test]
    fn test() {
        let mut ms = Minesweeper::new(10, 10, 5);

        ms.open((5, 5));
        ms.toggle_flag((6, 6));
        ms.open((6, 6));

        println!("{}", ms);
    }
}