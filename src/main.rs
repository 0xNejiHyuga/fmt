fn main() {
    let value1 = 10;
    let value2 = 200;
    let value3 = 3000;
    // value | value * value
    println!("{:10} | {:20} | {} ", "value", "value * value", "address"); //title
    println!("{value1:_<10} | {:<20} | {:p}", value1 * value1, &value1);
    println!("{value2:_<10} | {:<20} | {:p}", value2 * value2, &value2);
    println!("{value3:_<10} | {:<20} | {:p}", value3 * value3, &value3);

}