use csv_to_ascii::some::read_csv;

fn main() {
    let csv = read_csv("penguins.csv");
    let table = csv.to_table(); // convert csv to table

    table.show();
}
