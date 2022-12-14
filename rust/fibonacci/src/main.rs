fn main() {
    let number= 12;

    if number <= 1{
        println!("{number}");
    }else{
        let x = fibonacci(number);
        println!("{}", x);
    }
}

fn fibonacci(x: u32) -> u32{ // define que x = u32 e que ira retornar um valor u32 também (-> u32)
    if x == 0 || x == 1{
        return x;
    }else{
        return fibonacci(x - 1) + fibonacci(x - 2);
    }
}