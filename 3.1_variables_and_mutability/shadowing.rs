fn main() {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("{x}");
    }

    println!("{x}");
}
