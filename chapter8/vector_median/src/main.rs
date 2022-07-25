use std::collections::HashMap;

fn main() {

    let mut v = vec![2,3,4,4,4,4,4,4,4,6,6,5,9,10,35,68,3,9,8,5,6,6,7,9];
    v.sort();
    let len = v.len();

    // Median
    if len % 2 == 0 {
        let index = len / 2;
        let i_minus_one = index - 1;
        let i_plus_one = index + 1;
        let median = ( &v[i_minus_one] + &v[i_plus_one] ) as f64 / 2.0;
        println!("Median is: {}", median);
    } else {
        let index = ((len as f64) / 2.0).floor() as usize; // floor, because index starts at 0
        println!("Median is: {}",&v[index]);
    }

    // Modus
    let mut mod_table = HashMap::new();
    for i in &v {
        let count = mod_table.entry(i).or_insert(0);
        *count += 1;
    }
    let mut highest = 0;
    let mut modus = 0;
    for (key, value) in &mod_table {
        if value > &highest {
            highest = *value;
            modus = **key;
        }
    }
    println!("Modus is: {}", modus);
    println!("{:?}", mod_table);



}
