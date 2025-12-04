use crate::utils::file_utils::read_file_to_list;

#[derive(Copy, Clone, PartialEq)]
pub enum CellType {
    Empty,
    Towel,
}

#[derive(Copy, Clone)]
pub struct Cell {
    pub pos: (usize, usize),
    pub cell_type: CellType,
}

pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Cell>,
}

impl Grid {
    pub const DIRS: [(i32, i32); 8] = [
        (-1, -1), (-1, 0), (-1, 1),
        ( 0, -1),          ( 0, 1),
        ( 1, -1), ( 1, 0), ( 1, 1),
    ];

    pub fn new(input: Vec<Vec<char>>) -> Self {
        let height = input.len();
        let width = input[0].len();

        let cells = input.iter().enumerate().flat_map(|(r, row)| {
            row.iter()
                .enumerate()
                .map(move |(c, &ch)| {
                let cell_type = if ch == '@' { CellType::Towel } else { CellType::Empty };
                Cell { pos: (r, c), cell_type }
            })
        }).collect();

        Grid { width, height, cells }
    }

    pub fn find_accessible(&self) -> Vec<usize> {
        self.cells.iter().enumerate()
            .filter_map(|(idx, cell)| {
                if cell.cell_type != CellType::Towel {
                    return None;
                }

                let (r, c) = cell.pos;

                let neighbors = Self::DIRS.iter()
                    .filter(|&&(dx, dy)| {
                        let nr = r as i32 + dx;
                        let nc = c as i32 + dy;

                        if nr < 0 || nr >= self.height as i32 || nc < 0 || nc >= self.width as i32 {
                            return false;
                        }

                        let nidx = nr as usize * self.width + nc as usize;
                        self.cells[nidx].cell_type == CellType::Towel
                    })
                    .count();

                if neighbors < 4 { Some(idx) } else { None }
            })
            .collect()
    }

    pub fn remove_all_accessible(&mut self) -> usize {
        let mut total_removed = 0;

        while !self.find_accessible().is_empty() {
            let to_remove = self.find_accessible();
            to_remove.iter().for_each(|&idx| self.cells[idx].cell_type = CellType::Empty);
            total_removed += to_remove.len();
        }

        total_removed
    }
}

pub fn run() {
    let input = read_file_to_list("src/Problems/day4.txt");
    let grid_chars: Vec<Vec<char>> = input.iter()
        .map(|line| line.chars().collect())
        .collect();

    let mut grid = Grid::new(grid_chars);
    let removed = grid.remove_all_accessible();
    println!("{}", removed);
}
