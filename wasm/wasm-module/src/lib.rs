use std::mem;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
//чтобы каждая ячейка была представлена ​​одним байтом
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}
#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}
#[wasm_bindgen]
impl Universe {
    // Чтобы получить доступ к ячейке в данной строке и столбце, мы переводим строку и столбец в индекс вектора ячеек
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }
    // Чтобы вычислить следующее состояние ячейки, нам нужно подсчитать, сколько ее соседей живы.
    // В live_neighbor_count методе используются дельты и модуль,
    // чтобы избежать специального оформления краев вселенной с помощью ifs.
    // Применяя дельту -1, мы добавляем self.height - 1 и позволяем модулю делать свое дело, а не пытаемся вычесть 1.
    // row и column может быть 0, и если бы мы попытались вычесть 1 из них, произошло бы опустошение беззнакового целого числа.
    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }
                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }
    // Теперь у нас есть все необходимое, чтобы вычислить следующее поколение на основе текущего!
    // Каждое из правил игры следует за простым переводом в условие выражения match.
    // Кроме того, поскольку мы хотим, чтобы JavaScript контролировал возникновение тиков,
    // мы поместим этот метод внутри блока #[wasm_bindgen] , чтобы он был доступен JavaScript.
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by under population.
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (Cell::Dead, 3) => Cell::Alive,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };
                next[idx] = next_cell;
            }
        }
        self.cells = next;
    }
    // инициализирует вселенную интересным шаблоном живых и мертвых ячеек
    pub fn new() -> Universe {
        let width = 124 * 4;
        let height = 64 * 4;
        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();
        Universe {
            width,
            height,
            cells,
        }
    }
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn cells(&self) -> js_sys::Uint8Array {
        unsafe {
            let u8_cells = mem::transmute::<&Vec<Cell>, &Vec<u8>>(&self.cells);
            js_sys::Uint8Array::view(&u8_cells)
        }
    }
}