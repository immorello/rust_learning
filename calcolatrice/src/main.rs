use calcolatrice::choose_operator;
use calcolatrice::parse_operator;
use calcolatrice::saving_number;
use std::io;
fn main() {
    println!("Insert the number of elements involved:");
    let elements_number: i32 = saving_number();
    let mut arr: Vec<i32> = Vec::new();

    for _ in 0..elements_number {
        arr.push(saving_number());
    }

    println!("Elements number: {}, Elements: {:?}", elements_number, arr);

    // Anche qui usiamo un loop perché l'utente potrebbe non inserire
    // subito un operatore valido.
    let operator = loop {
        println!("Choose the operator");
        let mut character: String = String::new();
        match io::stdin().read_line(&mut character) {
            Ok(_) => {}
            Err(_) => {
                println!("Errore nella lettura del valore");
                continue;
            }
        }
        match character.trim().chars().next() {
            Some(operator) => break operator,
            None => {
                println!("Not a valid character");
                continue;
            }
        }
    };

    // Qui trasformiamo il char letto da stdin in un enum Operator.
    // Da questo punto in poi non lavoriamo più con input "grezzo".
    let op_result = parse_operator(operator);
    match op_result {
        Ok(res) => {
            // Passiamo &arr: main resta proprietario del Vec
            // e choose_operator prende solo un prestito in lettura.
            let result = choose_operator(res, &arr);
            match result {
                Ok(value) => println!("Your result is {}", value),
                Err(message) => println!("Error while processing: {}", message),
            }
        }
        Err(_) => {
            println!("Error while parsing the operator")
        }
    }
}
