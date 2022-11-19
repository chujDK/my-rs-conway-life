#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CellState {
    Alive,
    Dead,
}

#[derive(Clone, Debug)]
pub struct Cells {
    x: usize,
    y: usize,
    map: Vec<CellState>,
}

#[derive(thiserror::Error, Debug)]
enum CellsErr {
    #[error("array index out of bounds!")]
    OOB,
}

impl Cells {
    pub fn new(x: usize, y: usize) -> Self {
        Cells {
            x: x,
            y: y,
            map: vec![CellState::Dead; x * y],
        }
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }

    pub fn set(
        &mut self,
        x: usize,
        y: usize,
        alive: CellState,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if x < self.x && y < self.y {
            let index = self.c2i(x, y);
            self.map[index] = alive;
            Ok(())
        } else {
            Err(Box::new(CellsErr::OOB))
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Result<CellState, Box<dyn std::error::Error>> {
        if x < self.x && y < self.y {
            Ok(self.map[self.c2i(x, y)])
        } else {
            Err(Box::new(CellsErr::OOB))
        }
    }

    fn count_neighbors(&self, (x, y): (usize, usize)) -> usize {
        let mut lives: usize = 0;
        let check = |x, y| match self.get(x, y) {
            Ok(live) => match live {
                CellState::Alive => 1,
                CellState::Dead => 0,
            },
            Err(_) => 0,
        };

        if x > 0 {
            lives += check(x - 1, y);
            lives += check(x - 1, y + 1);
            if y > 0 {
                lives += check(x - 1, y - 1);
            }
        }
        if y > 0 {
            lives += check(x, y - 1);
            lives += check(x + 1, y - 1);
        }

        lives += check(x, y + 1) + check(x + 1, y + 1) + check(x + 1, y + 1);

        lives
    }

    pub fn reduce(&mut self) {
        let old_map = self.to_owned();
        self.map.iter_mut().enumerate().for_each(|(idx, cell)| {
            match old_map.count_neighbors(old_map.i2c(idx)) {
                n if n == 3 => {
                    if *cell == CellState::Dead {
                        *cell = CellState::Alive;
                    }
                }
                n if n == 2 => {
                    // nothing change, dead still dead, live long live
                }
                _ => {
                    if *cell == CellState::Alive {
                        *cell = CellState::Dead;
                    }
                }
            }
        })
    }

    fn i2c(&self, index: usize) -> (usize, usize) {
        (index / self.x, index % self.x)
    }

    /// coordinate to index helper function
    fn c2i(&self, x: usize, y: usize) -> usize {
        y * self.x + x
    }
}

#[cfg(test)]
mod cells_test {

    use super::*;
    #[test]
    fn set_get_test() {
        let mut cells = Cells::new(30, 20);

        assert_eq!(30, cells.x());
        assert_eq!(20, cells.y());

        cells.set(15, 12, CellState::Alive).unwrap();
        cells.set(13, 12, CellState::Alive).unwrap();
        cells.set(10, 11, CellState::Alive).unwrap();
        cells.set(0, 2, CellState::Alive).unwrap();

        assert_eq!(CellState::Alive, cells.get(15, 12).unwrap());
        assert_eq!(CellState::Alive, cells.get(13, 12).unwrap());
        assert_eq!(CellState::Alive, cells.get(10, 11).unwrap());
        assert_eq!(CellState::Alive, cells.get(0, 2).unwrap());

        assert_eq!(cells.get(31, 10).is_err(), true);
        assert_eq!(cells.set(0, 20, CellState::Alive).is_err(), true);

        cells.set(0, 2, CellState::Dead).unwrap();
        assert_eq!(CellState::Dead, cells.get(0, 2).unwrap());
    }

    #[test]
    fn reduce_test() {
        let mut cells = Cells::new(20, 20);
        // [ ] [x] [ ]
        // [x] [x] [x]
        // [ ] [x] [ ]
        // [ ] [ ] [ ]
        cells.set(0, 1, CellState::Alive).unwrap();
        cells.set(1, 0, CellState::Alive).unwrap();
        cells.set(1, 1, CellState::Alive).unwrap();
        cells.set(1, 2, CellState::Alive).unwrap();
        cells.set(2, 1, CellState::Alive).unwrap();

        // [x] [x] [x]
        // [x] [ ] [x]
        // [x] [x] [x]
        // [ ] [ ] [ ]
        cells.reduce();
        assert_eq!(CellState::Dead, cells.get(1, 1).unwrap());
        assert_eq!(CellState::Alive, cells.get(0, 0).unwrap());
        assert_eq!(CellState::Alive, cells.get(0, 1).unwrap());
        assert_eq!(CellState::Alive, cells.get(0, 2).unwrap());
        assert_eq!(CellState::Alive, cells.get(1, 0).unwrap());
        assert_eq!(CellState::Alive, cells.get(1, 2).unwrap());
        assert_eq!(CellState::Alive, cells.get(2, 0).unwrap());
        assert_eq!(CellState::Alive, cells.get(2, 1).unwrap());
        assert_eq!(CellState::Alive, cells.get(2, 2).unwrap());

        // [x] [ ] [x] [ ]
        // [ ] [ ] [ ] [x]
        // [x] [ ] [x] [ ]
        // [ ] [x] [ ] [ ]
        // [ ] [ ] [ ] [ ]
        cells.reduce();
        assert_eq!(CellState::Alive, cells.get(0, 0).unwrap());
        assert_eq!(CellState::Alive, cells.get(0, 2).unwrap());
        assert_eq!(CellState::Alive, cells.get(1, 3).unwrap());
        assert_eq!(CellState::Alive, cells.get(2, 0).unwrap());
        assert_eq!(CellState::Alive, cells.get(2, 2).unwrap());
        assert_eq!(CellState::Alive, cells.get(3, 1).unwrap());
    }
}
