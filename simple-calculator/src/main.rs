mod maths;

fn main() {
    let result =  maths::add(234, 2122);
    println!("{}", result);

    let result =  maths::sub(500, 200);
    println!("{}", result);

    let result =  maths::prod(35, 40);
    println!("{}", result);

    let result =  maths::div(3000, 50);
    println!("{}", result);
}