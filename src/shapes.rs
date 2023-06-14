use crate::useful::{Dimension, Coordinate};
fn locate(origin:Coordinate,row:i32){
    print!("{}[{};{}H",
    "\u{001b}",
    origin.0+row, origin.1)
}

pub fn rectangle(origin:Coordinate,dimension:Dimension){
    for row in 0..dimension.height{
        locate(origin,row.try_into().expect("Row count too hight to cast into i32"));
        for _ in 0..dimension.width*2{
            print!(" ");
        }
    }
}
