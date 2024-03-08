fn main() {
    a()
}

fn a(){
    b()
}

fn b(){
    c(27)
}

fn c (x: u8){
    if x >= 25 {
        panic!("No more money mismanagement!")
    }else{
        println!("You're still young but you need to manage money better!")
    }
}
