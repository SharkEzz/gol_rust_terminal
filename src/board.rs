use rand::Rng;

pub struct Board {
    width: usize,
    height: usize,
    cells: Vec<Vec<bool>>,
}

pub fn new_board(width: usize, height: usize) -> Board {
    let mut cells: Vec<Vec<bool>> = Vec::with_capacity(height);

    let mut rnd = rand::thread_rng();

    for _y in 0..height {
        let mut x_cells: Vec<bool> = Vec::with_capacity(width);
        for _x in 0..width {
            let alive: u32 = rnd.gen_range(0..100);
            x_cells.push(alive > 80);
        }
        cells.push(x_cells);
    }

    Board {
        width,
        height,
        cells,
    }
}

impl Board {
    pub fn get_population(&self) -> u32 {
        let mut population: u32 = 0;

        let mut y = 0;
        while y < self.height {
            let mut x = 0;
            while x < self.width {
                if self.cells[y][x] {
                    population += 1;
                }
                x += 1;
            }
            y += 1;
        }

        return population;
    }

    pub fn alive_neighbors(&self, x: i32, y: i32) -> u32 {
        let mut alive = 0;

        for i in -1..=1 {
            for j in -1..=1 {
                if x + i < 0 || x + i >= self.width.try_into().expect("Can't convert value") {
                    continue;
                }
                if y + j < 0 || y + j >= self.height.try_into().expect("Can't convert value") {
                    continue;
                }
                if x + i == x && y + j == y {
                    continue;
                }

                if self.cells[(j + y) as usize][(x + i) as usize] {
                    alive += 1;
                }
            }
        }

        return alive;
    }

    pub fn compute_next_generation(&mut self) {
        let mut new_cells: Vec<Vec<bool>> = Vec::with_capacity(self.height);

        for y in 0..self.height {
            let mut x_cells: Vec<bool> = Vec::with_capacity(self.width);
            for x in 0..self.width {
                let current_cell = self.cells[y][x];
                let neighbors = self.alive_neighbors(x as i32, y as i32);

                if current_cell && (neighbors < 2 || neighbors > 3) {
                    x_cells.push(false);
                } else if !current_cell && neighbors == 3 {
                    x_cells.push(true);
                } else {
                    x_cells.push(current_cell);
                }
            }
            new_cells.push(x_cells);
        }

        self.cells = new_cells;
    }

    pub fn draw(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.cells[y][x] {
                    print!("*");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }
}
