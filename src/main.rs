use std::fs;

/*
fn create_csv()->String{
    let mut csv = String::new();
    csv.push_str("title1,title_2,title_02\n");
    csv.push_str("1121,12,3\n");
    csv.push_str("4,5,6\n");
    csv
}
*/

fn read_csv(filename:&str)->String{
    let contents = fs::read_to_string(filename).unwrap();
    contents
}

struct CSV{
    csv:String, 
}

struct Table<'a> {
    table: Vec<Vec<&'a str>>,
}

impl CSV{

    fn new(csv:String)->CSV{
        CSV{csv}
    }

    fn to_table(&self)->Table{

        let delimiter = ",";
        let mut table = Vec::new();
    
        for line in self.csv.lines(){
            let line_vec = line.split(delimiter).collect();
            table.push(line_vec);
        }
        Table{table}
    } 
}

impl<'a> Table<'a>{

    fn nrow(&self)->usize{
        self.table.len()
    }

    fn ncol(&self)->usize{
        self.table[0].len()
    }

    fn max_len_cols(&self)->Vec<usize>{
        /* find the length of longest cell for each column*/ 
        let mut max_lens = Vec::new();
        // init max_lens with 0
        for _i in 0..self.ncol(){
            max_lens.push(0);
        }
        // find max len for each column and overwrite previous max_lens
        for row in &self.table{
            for (i,cell) in row.iter().enumerate(){
                if cell.len() > max_lens[i]{
                    max_lens[i] = cell.len();
                }
            }
        }
        max_lens
    }

    fn width(&self)->usize{
        /* find the length of the longest row (line) */
        let sum_max_cols: usize = self.max_len_cols().iter().sum();
        let max_len = sum_max_cols + self.ncol()+1; // ncol for the delimiters 
        max_len
    }

    fn col_ends(&self)->Vec<usize>{
        let numbers = self.max_len_cols();
        let mut col_ends = Vec::with_capacity(numbers.len());

        let mut sum = 0;
        for &number in &numbers{
            sum += number+1;
            col_ends.push(sum);
        }
        col_ends
    }


    fn header(&self)->String{
        let mut header = String::new();
        header.push_str("╋");
        for i in 1..self.width()-1{
            if self.col_ends().contains(&i){
                header.push_str("╋");
            }
            else{
                header.push_str("─");
            }
        }
        header.push_str("╋\n");
        header
    }

    fn row_to_ascii(&self, row:&Vec<&str>)->String{
        let mut row_ascii = String::new();
        row_ascii.push_str("│");
        for (i, cell) in row.iter().enumerate(){
            let diff = self.max_len_cols()[i] - cell.len();


            row_ascii.push_str(cell);
            row_ascii.push_str(&" ".repeat(diff));
            row_ascii.push_str("│");
        }
        row_ascii.push_str("\n");
        row_ascii
    }

    fn to_acsii(&self)->String{
        let mut ascii = String::new();
        ascii.push_str(&self.header());
        for (i, row) in self.table.iter().enumerate(){
            ascii.push_str(&self.row_to_ascii(row));
            if i == 0 || i == self.nrow()-1 {
                ascii.push_str(&self.header());
            }
        
        }
        ascii
    }

    fn show(&self){
        println!("Shape: {} x {} ", self.nrow(), self.ncol());
        println!("{}",self.to_acsii());
    }
}

fn main() {
    let csv = CSV::new(read_csv("iris.csv"));
    let table = csv.to_table(); // convert csv to table

    table.show();
}
