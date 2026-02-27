fn main() {
    use rand::prelude::*;
    let mut rng = rand::rng();

    let roofrange: Vec<i32> = (6..24).collect();
    let roofopt = roofrange.choose(&mut rng);

    fn opt_to_num(x: Option<&i32>) -> i32 {
        *x.unwrap_or(&0)
    }
    let floor: i32 = 3;
    let roof: i32 = opt_to_num(roofopt);

    fn print_numbers(floor: i32, roof: i32) -> i32 {
        let mut sum = 0;
        println!("Prints numbers from {} to {}", floor, roof - 1);
        for i in floor..roof {
            println!("{}", i);
            sum += i;
        }
        sum
    }

    println!(
        "Total sum of all printed numbers: {}",
        print_numbers(floor, roof)
    )
}
