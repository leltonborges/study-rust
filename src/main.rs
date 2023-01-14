fn main() {
    let idade:i32 = 31;
    println!("valor = {}, tamanho = {}", idade, std::mem::size_of_val(&idade));

    let decimal  = 2.5;
    println!("Decimal = {}, tamanho = {}", decimal, std::mem::size_of_val(&decimal));

    let boleana =  false;
    println!("Boleana = {}, tamanho = {}", boleana, std::mem::size_of_val(&boleana));

    let letra = 'C';
    println!("Char = {}, tamanho = {}", letra, std::mem::size_of_val(&letra));

}
