use std::io;
fn main() {
    let mut numero1 = String::new();
    let mut numero2 = String::new();

    println!( "Ingrese primer digito: ");
    io::stdin().read_line(&mut numero1);
    let numero1 = numero1.trim();
    let numero1:i32 = numero1.parse().unwrap();

    println!( "Ingrese segundo digito: ");
    io::stdin().read_line(&mut numero2);
    let numero2 = numero2.trim();
    let numero2:i32 = numero2.parse().unwrap();

    let resultado = numero1 + numero2;
    let resultado2 = numero1 - numero2;
    println!(" el resultado de la suma es: {}",resultado);
    println!(" el resultado de la resta es: {}",resultado2);
}
