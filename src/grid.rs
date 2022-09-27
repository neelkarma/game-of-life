use ::rand::{thread_rng, Rng};
use rand::distributions::Uniform;

pub struct Grid {
    pub width: usize,
    pub height: usize,
    cells: Vec<bool>,
    buffer: Vec<bool>,
}

impl Grid {
    pub fn random(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: thread_rng()
                .sample_iter(Uniform::from(0..=5))
                .take(width * height)
                .map(|i| i == 0)
                .collect(),
            buffer: vec![false; width * height],
        }
    }

    fn coords_to_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn get_cell(&self, x: usize, y: usize) -> bool {
        self.cells[self.coords_to_index(x, y)]
    }

    fn get_cell_neighbors(&self, x: usize, y: usize) -> usize {
        let x = x as isize;
        let y = y as isize;

        let neighbor_coords = [
            (x, y - 1),
            (x + 1, y - 1),
            (x + 1, y),
            (x + 1, y + 1),
            (x, y + 1),
            (x - 1, y + 1),
            (x - 1, y),
            (x - 1, y - 1),
        ];

        let mut count = 0;

        for (nx, ny) in neighbor_coords {
            if !(0 <= nx
                && nx < self.width as isize
                && 0 <= ny
                && ny < self.height as isize
                && self.get_cell(nx as usize, ny as usize))
            {
                continue;
            };
            count += 1;
        }

        count
    }

    pub fn update(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = self.get_cell(x, y);
                let neighbors = self.get_cell_neighbors(x, y);
                let idx = self.coords_to_index(x, y);

                self.buffer[idx] = match cell {
                    true => match neighbors {
                        2 | 3 => true,
                        _ => false,
                    },
                    false => match neighbors {
                        3 => true,
                        _ => false,
                    },
                }
            }
        }

        self.cells = self.buffer.clone();
    }
}
