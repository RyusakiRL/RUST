fn main () {
    let mut numero = 6;
    let mut soma = 0;

    while numero<=100 {
        println!("[{}]", numero);
        numero = numero + 2;
        soma = soma + numero;
    }
        println!("The some is {}", soma)

}