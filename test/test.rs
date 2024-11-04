use csv_to_ascii::some::CSV;

fn create_csv()->String{
    let mut csv = String::new();
    csv.push_str("title1,title_2,title_02\n");
    csv.push_str("1121,12,3\n");
    csv.push_str("4,5,6\n");
    csv
}

#[test]
fn test_csv_to_ascii() {
    let csv = CSV::new(create_csv());
    let table = csv.to_table();
    assert_eq!(table.nrow(), 3);
    assert_eq!(table.ncol(), 3);
}