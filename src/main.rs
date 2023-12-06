fn main() {
    // Exemplo com String 
    let message: String = String::from("Hello MasterDEV !");
    // println!("{}", message);
    println!("{}", message);


    // Exemplos com Números 
    let valor: i32 = 1000000000; // 10 casas ? 
    println!("{}", valor);

    let valor: i64 = 1000000000000000000; // 19 casas ? 
    println!("{}", valor);

    let valor: i128 = 100000000000000000000000000000000000000; // 39 casas ? 
    println!("{}", valor);


    // exemplos com float 
    let dinheiro: f32 = 100000.2222222222;
    println!("{}", dinheiro);

    let dinheiro: f64 = 100000.2222222222;
    println!("{}", dinheiro);

    // ifs 
    let idade: i32 = 18;
    if idade >= 18 {
        println!("Maior de idade")
    } else {
        println!("Menor de idade")
    }

    // matches 
    let array = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    for x in array {
        match x {
            2|4|6|8 => println!("{} é par", x),
            1|3|5|7|9 => println!("{} é ímpar", x),
            _ => println!("{} é ???", x),
        }
    }
}
