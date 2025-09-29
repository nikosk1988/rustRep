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

//sunarthsh gia thn euresh mesw id
fn searchById(id: i32, idvec: &Vec<i32>, titlevec: &Vec<&str>, authorvec: &Vec<&str>, isbnvec: &Vec<&str>, editionvec: &Vec<i32>, yearvec: &Vec<i32>, kindvec: &Vec<&str>){ //oi parametroi tha einai vectors(oxi pinakes) me ta stoixeia 3exwrista
    
    let mut mid: usize = 0; //deikths mesaias theshs
    let mut l: usize = 0; //aristeros deikths
    let mut r: usize = idvec.len() - 1; //de3ios deikths
    let key: i32 = id;  //to id pou anazhtw
    let mut found: bool = false; 
    
    while (l <= r)&&(found==false){
        mid = (l+r)/2;
        if(idvec[mid] == key){
            found = true;
        }
        else if (idvec[mid] < key){
            l = mid + 1;
        }
        else {
            if mid == 0 {
                break;
            }
            r = mid - 1;
        }
    }
    
    if (found == true){
        println!("{} {} {} {} {} {} {}", idvec[mid], titlevec[mid], authorvec[mid], isbnvec[mid], editionvec[mid], yearvec[mid], kindvec[mid] );
    }
    else {
        println!("Το βιβλίο με id {} δε βρέθηκε!", key);
    }
}


//sunarthsh gia thn euresh mesw titlou
fn searchById(title: &str, idvec: &Vec<i32>, titlevec: &Vec<&str>, authorvec: &Vec<&str>, isbnvec: &Vec<&str>, editionvec: &Vec<i32>, yearvec: &Vec<i32>, kindvec: &Vec<&str>){ //oi parametroi tha einai vectors(oxi pinakes) me ta stoixeia 3exwrista
    
    let mut found: bool = false; 
    let mut i: usize = 0; 
    let mut size: usize = titlevec.len()-1;
    let mut pos: usize = 0;
    
    if (size >=0 )
    {
        while (i <= size)&&(found==false)
        {
            if (titlevec[i] == title)
            {
                found = true;
                pos = i;
                break;
            }
            else {
                i +=1;
            }
        }
    }
    else 
    {
        println!("Δεν υπάρχει κανένα βιβλίο μέσα στη βάση!\n");
    }
    
    
    if (found == true){
        println!("{} {} {} {} {} {} {}", idvec[pos], title, authorvec[pos], isbnvec[pos], editionvec[pos], yearvec[pos], kindvec[pos] );
    }
    else {
        println!("Το βιβλίο με τίτλο {} δε βρέθηκε!", title);
    }
}



fn subMenu1()  {
    
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

     match sel1 {
        1 => {
            println!("Αναζήτηση με το ID του βιβλίου");
            let mut id_input = String::new();
            println!("ID:");
            std::io::stdin().read_line(&mut id_input).expect("ERROR");

            let id: i32 = match id_input.trim().parse() {
                Ok(n) => n,
                Err(_) => {
                    println!("Μη έγκυρο ID!");
                    return;
                }
            };

            println!("Αναζητείται βιβλίο με ID: {}", id);
            //kalw to function gia thn anazhthsh me to antistoixo id
            searchById(id);
        },
        
        2 => {
            println!("Αναζήτηση με τίτλο");
        },
        3 => {
            println!("Αναζήτηση με όνομα συγγραφέα");
        },
        4 => {
            println!("Αναζήτηση με ISBN");
        },
        5 => {
            println!("Αναζήτηση με έτος έκδοσης");
        },
        6 => {
            println!("Αναζήτηση με είδος βιβλίου");
        },
        _ => {
            println!("Μη έγκυρη επιλογή!");
        },
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

