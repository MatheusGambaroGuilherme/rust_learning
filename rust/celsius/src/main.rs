fn main() {
    let mut temp = 76.0;
    let mut typ = 'c';
    let celsius = (temp * 1.8) + 32.0;
    let faren = (temp - 32.0) * 5.0/9.0;
    if typ == 'f'{
        println!("A temperatura em farenheit é: {celsius}");
    }else{
        println!("A temperatura em cesius é igual a: {faren}");
    }
}
