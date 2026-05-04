use std::io;

// Enum = tipo con un insieme finito di varianti valide.
// Qui ci serve per trasformare l'input "grezzo" dell'utente
// in un tipo più sicuro del semplice char.
pub enum Operator{
    Add,
    Sub,
    Mul,
    Div,
}

pub fn sum(arr: &[i32])->Result<i32, String>{
    // &[i32] = slice immutabile: la funzione legge i dati
    // senza diventare proprietaria del Vec creato in main.
    let mut sum:i32 = 0;
    for num in arr{
        sum = sum + num;
    }  
    Ok(sum)
}

pub fn sub(arr: &[i32]) -> Result<i32, String> {
    // split_first separa "primo elemento" e "resto".
    // Evita di usare arr[0], che farebbe panic su slice vuota.
    match arr.split_first() {
        Some((first, rest)) => {
            // first è un &i32, quindi lo dereferenziamo con *
            // per ottenere il valore i32 da usare come accumulatore.
            let mut result = *first;
            for num in rest {
                result -= num;
            }
            Ok(result)
        }
        None => Err("Non posso sottrarre da una lista vuota".to_string()),
    }
}

pub fn mul(arr: &[i32])->Result<i32,String>{
    let mut mul:i32 = 1;
    for num in arr{
        mul = mul * num;
    }
    Ok(mul)
}

pub fn div(arr: &[i32])->Result<i32, String>{
    match arr.split_first(){
        Some((first,rest))=>{
            let mut result:i32 = *first;
            for num in rest{
                // Qui il Result è utile: invece di andare in errore a runtime,
                // restituiamo esplicitamente un caso Err.
                if *num == 0{
                    return Err("Non posso dividere per zero".to_string())
                }
                result /= num;
            }
            Ok(result)
        }
        None => Err("Non posso dividere da un lista vuota".to_string()),
    }
}

pub fn saving_number()->i32{
    // loop qui è comodo perché non sappiamo in anticipo
    // quante volte l'utente sbaglierà input.
    loop{
        println!("Inserisci un numero:");
        // Il buffer è dentro al loop così ogni tentativo parte pulito.
        let mut input_num = String::new();

        // read_line restituisce Result:
        // Ok(usize) se la lettura riesce, Err(...) se fallisce.
        match io::stdin().read_line(&mut input_num){
            Ok(_)=>{},
            Err(_)=>{
                println!("Error while reading line"); 
                continue;
            }
        }

        // trim restituisce &str senza spazi/newline.
        // parse restituisce Result perché la conversione può fallire.
        match input_num.trim().parse(){
            // break num esce dal loop restituendo quel valore alla funzione.
            Ok(num)=> break num,
            Err(_)=>println!("Not a number, try again")
        }
    }
    
}

pub fn choose_operator(operator:Operator, arr: &[i32])->Result<i32,String>{
    // A questo punto l'operatore è già stato validato:
    // qui dobbiamo solo scegliere quale funzione chiamare.
    match operator {
        Operator::Add => sum(arr),
        Operator::Sub => sub(arr),
        Operator::Mul => mul(arr),
        Operator::Div => div(arr),
    }
    
   
}

pub fn parse_operator(operator:char)->Result<Operator, String>{
    // Questa funzione converte un char in un enum del dominio.
    // Se il char non è valido, restituisce Err invece di inventarsi un valore.
    match operator{
        '+' => Ok(Operator::Add),
        '-' => Ok(Operator::Sub),
        '*' => Ok(Operator::Mul),
        '/' => Ok(Operator::Div),
        _ => Err("Operatore non valido".to_string())
    }
}
