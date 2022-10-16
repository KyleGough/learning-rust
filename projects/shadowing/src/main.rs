fn main() {
    let x = 16;

    let x = x + 4;

    {
        let x = x * 2;
        println!("Value of x in inner scope is {x}");
    }

    println!("Value of x is {x}");
}
