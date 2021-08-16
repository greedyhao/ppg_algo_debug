use std::fs::File;
use std::io::{BufRead, BufReader};

// pub struct File2Data {
//     file: String,
//     cols: i32,
//     pub cols_vec: Vec<Vec<i32>>,
// }

// impl File2Data {
//     pub fn new (self, _file: String) -> File2Data {
//         File2Data {
//             file: _file,
//             cols: -1,
//             cols_vec: _,
//         }
//     }

//     pub fn get_cols_vec_from_file(self) {
//         let f = BufReader::new(File::open(self.file).unwrap());
//         let f_lines = f.lines().skip(1);

//         // println!("{:?}", f_lines.as_ref().unwrap().split(char::is_whitespace));
    
//         let mut arr: Vec<Vec<i32>> = f_lines
//             .filter(|l| l.as_ref().unwrap().is_empty() == false)
//             .map(|l| l.unwrap().split(char::is_whitespace)
//                  .map(|number| number.parse().unwrap())
//                  .collect())
//             .collect();
    
//         arr.pop(); // avoid column0.len() != column1.len()
//         self.cols_vec = arr;
//     }

//     pub fn get_col_from_cols_vec(self) -> Vec<i32> {
//         let col: Vec<i32> = self.cols_vec
//             .iter()  // iterate over rows
//             .map(|x| x[0]) // get the icolumn-th element from each row
//             .collect();  // collect into new vector
//         col
//     }
// }

pub fn get_cols_vec_from_file(file: String) -> (i32, Vec<Vec<i32>>) {
    let f = BufReader::new(File::open(file).unwrap());
    let mut f_lines = f.lines().skip(1);
    let mut col_cnt = 0;

    for _ in f_lines.next().unwrap().unwrap().split_whitespace() {
        col_cnt += 1;
    }

    let mut arr: Vec<Vec<i32>> = f_lines
        .filter(|l| !l.as_ref().unwrap().is_empty())
        .map(|l| l.unwrap().split(char::is_whitespace)
                .map(|number| number.parse().unwrap())
                .collect())
        .collect();

    arr.pop(); // avoid column0.len() != column1.len()

    (col_cnt, arr)
}

pub fn get_col_from_cols_vec(index: usize, cols: &[Vec<i32>]) -> Vec<i32> {
    let col: Vec<i32> = cols
        .iter()  // iterate over rows
        .map(|x| x[index]) // get the icolumn-th element from each row
        .collect();  // collect into new vector
    // println!("{:?}", cols.iter().map(|x| x[index]));\
    col
}
