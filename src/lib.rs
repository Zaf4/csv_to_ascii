pub mod some {
    pub struct CSV {
        csv: String,
    }

    pub struct Table<'a> {
        table: Vec<Vec<&'a str>>,
    }



    impl CSV {
        pub fn new(csv: String) -> CSV {
            CSV { csv }
        }

        pub fn to_table(&self) -> Table {
            let delimiter = ",";
            let mut table = Vec::new();

            for line in self.csv.lines() {
                let line_vec = line.split(delimiter).collect();
                table.push(line_vec);
            }
            Table { table }
        }
    }

    impl<'a> Table<'a> {
        pub fn nrow(&self) -> usize {
            self.table.len()
        }

        pub fn ncol(&self) -> usize {
            self.table[0].len()
        }

        pub fn max_len_cols(&self) -> Vec<usize> {
            /* find the length of longest cell for each column*/
            let mut max_lens = Vec::new();
            // init max_lens with 0
            for _i in 0..self.ncol() {
                max_lens.push(0);
            }
            // find max len for each column and overwrite previous max_lens
            for row in &self.table {
                for (i, cell) in row.iter().enumerate() {
                    if cell.len() > max_lens[i] {
                        max_lens[i] = cell.len();
                    }
                }
            }
            max_lens
        }

        pub fn width(&self) -> usize {
            /* find the length of the longest row (line) */
            let sum_max_cols: usize = self.max_len_cols().iter().sum();
            let max_len = sum_max_cols + self.ncol() + 1; // ncol for the delimiters
            max_len
        }

        pub fn col_ends(&self) -> Vec<usize> {
            let numbers = self.max_len_cols();
            let mut col_ends = Vec::with_capacity(numbers.len());

            let mut sum = 0;
            for &number in &numbers {
                sum += number + 1;
                col_ends.push(sum);
            }
            col_ends
        }

        pub fn header(&self) -> String {
            let mut header = String::new();
            header.push_str("╋");
            for i in 1..self.width() - 1 {
                if self.col_ends().contains(&i) {
                    header.push_str("╋");
                } else {
                    header.push_str("─");
                }
            }
            header.push_str("╋\n");
            header
        }

        pub fn row_to_ascii(&self, row: &Vec<&str>) -> String {
            let mut row_ascii = String::new();
            row_ascii.push_str("│");
            for (i, cell) in row.iter().enumerate() {
                let diff = self.max_len_cols()[i] - cell.len();

                row_ascii.push_str(cell);
                row_ascii.push_str(&" ".repeat(diff));
                row_ascii.push_str("│");
            }
            row_ascii.push_str("\n");
            row_ascii
        }

        pub fn to_acsii(&self) -> String {
            let mut ascii = String::new();
            ascii.push_str(&self.header());
            for (i, row) in self.table.iter().enumerate() {
                ascii.push_str(&self.row_to_ascii(row));
                if i == 0 || i == self.nrow() - 1 {
                    ascii.push_str(&self.header());
                }
            }
            ascii
        }

        pub fn show(&self) {
            println!("Shape: {} x {} ", self.nrow(), self.ncol());
            println!("{}", self.to_acsii());
        }
    }

    pub fn read_csv(filename: &str) -> CSV {
        let contents = std::fs::read_to_string(filename).unwrap();
        let table = CSV::new(contents);
        table
    }
}
pub use some::{Table, CSV};

#[cfg(test)]
mod tests {
    use some::read_csv;

    use super::*;

    #[test]
    fn test_csv_to_ascii() {
        let mut csv = String::new();
        csv.push_str("title1,title_2,title_02\n");
        csv.push_str("1121,12,3\n");
        csv.push_str("4,5,6\n");
        let file  = CSV::new(csv);
        let table = file.to_table();
        assert_eq!(table.nrow(), 3);
        assert_eq!(table.ncol(), 3);
    }

    #[test]
    fn file_printing() {
        let file = read_csv("starbucks.csv");
        let table = file.to_table();
        table.show();
    }

}

