
// Imports


// Macros

// See the following post from kaj, on the Rust user foruns.
// https://users.rust-lang.org/t/3-things-that-the-rust-standard-library-should-have/68825/9
//
// Macro visibility rules.
// https://users.rust-lang.org/t/3-things-that-the-rust-standard-library-should-have/68825/11
//
macro_rules! graph {
    [$($k:expr => $($v:expr),*);*] => {
        ::std::collections::HashMap::from([
            $(($k, vec![ $($v),* ])),*
        ])
    }
}

pub(crate) use graph;



pub struct Grid2D {
    grid: Vec<i32>,
    rows_len: i32,
    cols_len: i32,
}

impl Grid2D {
    pub fn new(rows_len: i32, cols_len: i32) -> Self {
        assert!(rows_len >= 0);
        assert!(cols_len >= 0);
        Grid2D {
            grid: vec![0; (rows_len * cols_len) as usize],
            rows_len: rows_len,
            cols_len: cols_len,
        }
    }

    pub fn new_ar(rows_len: i32, cols_len: i32, ar: & [i32]) -> Self {
        assert!(rows_len >= 0);
        assert!(cols_len >= 0);
        Grid2D {
            grid: Vec::from(ar),
            rows_len: rows_len,
            cols_len: cols_len,
        }
    }

    pub fn new_with_val(rows_len: i32, cols_len: i32, val: i32) -> Self {
        assert!(rows_len >= 0);
        assert!(cols_len >= 0);
        Grid2D {
            grid: vec![val; (rows_len * cols_len) as usize],
            rows_len: rows_len,
            cols_len: cols_len,
        }
    }
    
    pub fn get(& self, row: i32, col: i32) -> i32 {
        assert!(row >= 0 && row < self.rows_len);
        assert!(col >= 0 && col < self.cols_len);
        self.grid[(row * self.cols_len + col) as usize]
    }

    pub fn set(& mut self, row: i32, col: i32, val: i32) {
        assert!(row >= 0 && row < self.rows_len);
        assert!(col >= 0 && col < self.cols_len);
        self.grid[(row * self.cols_len + col) as usize] = val;
    }

    pub fn rows_len(& self) -> i32 {
        self.rows_len
    }

    pub fn cols_len(& self) -> i32 {
        self.cols_len
    }

    pub fn print(& self) {
        for row in 0..self.rows_len {
            for col in 0..self.cols_len {
                print!("{} ", self.get(row, col));
            }
            print!("\n");
        }
        print!("\n");
    }
}