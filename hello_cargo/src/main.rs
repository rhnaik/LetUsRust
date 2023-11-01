fn main() {
    let mysum = sum(10, 20);
    println!("Hello, world! {}", mysum);
}

fn sum(var1: i32, var2: i32)-> i32
{
    let var3 = var1 + var2;
    var3
}