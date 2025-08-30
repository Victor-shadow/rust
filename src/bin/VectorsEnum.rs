use crate::SpreadsheetCell::Float;

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String,)
}

fn main(){
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(100.5),
        SpreadsheetCell::Text(String::from("Blue")),

    ];

    println!("The values of items in the spreadsheet are: {}", row.len());

}