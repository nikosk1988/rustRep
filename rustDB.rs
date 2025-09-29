use std::io;
use std::collections::HashMap;

// dhmiourgia menu epilogwn
fn Menu(){
    println!("---MENU---");
    println!("1. Άνοιγμα Βάσης Δεδομένων");
    println!("2. Προσθήκη νέου βιβλίου");
    println!("3. Διαγραφή Βάσης Δεδομένων");
    println!("4. νημέρωση Βάσης Δεδομένων");
    println!("5. Έξοδος");
}

fn database(){
    //menu pou tha epilegei tupo anazhthseis p.x. ana titlo, ana suggrafea klp
    println!("\n");
    //kalei antistoixh sunarthsh pou tha epistrefei ta antistoixa dedomena kai tha ta emfanizei
    subMenu1();
    
}

fn subMenu1() -> i32 {
    
    println!("---Αναζήτηση στη Βάση Δεδομένων ---");
    println!("1. Με το id");
    println!("2. Με τον τίτλο");
    println!("3. Με το όνομα του συγγραφέα");
    println!("4. Με τον κωδικό isbn");
    println!("5. Με το έτος έκδοσης του βιβλίου");
    println!("6. Με το είδος του βιβλίου");
    
    //dialegw epilogh analoga me to ti thelw na kanw
    
    let mut input = String::new();
    println!("Επέλεξε τι από το παραπάνω menu θες να κάνεις: \t");
    std::io::stdin().read_line(&mut input).expect("ERROR");

    
    let mut sel1: i32 = input.trim().parse().expect("Δώσε μια απ' τις παραπάνω επιλογές: \t");

    match sel1 
    {
    
    1 =>{    
        println!("Αναζήτη με το ID του βιβλίου");
        let mut input = String:: new();
        println!("ID:\t");
        std::io::stdin().read_line(&mut input).expect("ERROR");
        let id: i32 = input.trim().parse();
        //Ok(id)
        }
        
        //let mut id: i32 = input.trim().parse().expect("ID:\t");
    }
    
}


fn main()
{

    println!("Δώσε μια επιλογή από τις παρακάτω:");
    //eisagwgh menu
    Menu();
    println!("\n");
    let mut input = String::new();
    io::stdin()
    .read_line(&mut input);
    //.expect("Αποτυχία ανάγνωσης επιλογής");
    
    let mut selection: i32 = input.trim().parse().expect("Δώσε μια απ' τις παραπάνω επιλογές: \t");
    
    //elegxos epilogwn
    
    if (selection == 1){
        println!("Βάση Δεδομένων");
        database();
    }
    else if (selection == 2){
        println!("Προσθήκη νέου");
    }
    else if (selection == 3){
        println!("Διαγραφή βάσης");
    }
    else if (selection == 4){
        println!("Ενημέρωση βασης");
    }
    else if(selection == 5){
        println!("Αντίο");
        std::process::exit(0);
    }
    else {
        println!("Λάθος επιλογή! Δώσε μια σωστή επιλογή! \n");
        Menu();
    }
}


