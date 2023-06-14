
fn locate(origin:[i32;2],row:i32){
    print!("{}[{};{}H",
    "\u{001b}",
    origin[0]+row, origin[1])
}

pub fn rectangle(origin:[i32;2],height:u32,width:u32){
    for row in 0..height{
        locate(origin,row.try_into().expect("Row count too hight to cast into i32"));
        for _ in 0..width*2{
            print!(" ");
        }
    }
}

pub fn square(origin:[i32;2],sidelength:u32){
    for row in 0..sidelength{
        locate(origin,row.try_into().expect("Row count too hight to cast into i32"));
        for _ in 0..sidelength*2{
            print!(" ");
        }
    }
}