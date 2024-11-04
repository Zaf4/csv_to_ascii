use csv_to_ascii::some::CSV;
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

fn read_csv(filename: &str) -> String {
    let contents = fs::read_to_string(filename).unwrap();
    contents
}

fn main() {
    let csv = CSV::new(read_csv("iris.csv"));
    let table = csv.to_table(); // convert csv to table

    table.show();
}
