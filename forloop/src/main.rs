fn main() {
    loop_sans_modif();
    loop_avec_modif();
}

fn loop_sans_modif() {
    let numbers = [1, 4, 7];

    for number in numbers.iter(){
        println!("Nombre : {}", number)
    }
}

fn loop_avec_modif() {
    let mut numbers = [1, 4, 7]; // avant pas de mut

    // .iter avant
    for number in numbers.iter_mut(){
        *number += 1;
        println!("Nombre : {}", number)
    }
}
