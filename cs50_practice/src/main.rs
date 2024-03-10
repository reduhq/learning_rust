fn main() {
    let mut fila = String::new();
    println!("Cuantas filas quiere?: ");
    std::io::stdin().read_line(&mut fila).unwrap();
    let fila = fila.trim().parse::<i8>().unwrap();
    let column = fila*2+1;
    let center = column/2;

    for row in 1..fila+1{
        for col in 0..column{
            if col == center{
                print!(" ");
            }else if col >= center-row && col <= center+row{
                    print!("#");
            }else{
                print!(" ");
            }
        }
        println!("")
    }
}
