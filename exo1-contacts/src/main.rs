use std::fs::File;
use std::io::Write;

struct Contact {
    nom: String, 
    telephone: String
}

// méthode ajouter_contact
fn ajouter_contact(contacts: &mut Vec<Contact>){
    println!("Ajouter Contact");

    // nom contact
    println!("Tapez le nom du contact");
    let mut nom = String::new();
    std::io::stdin().read_line(&mut nom).expect("Erreur de lecture");

    // numero contact
    println!("Tapez le numéro de téléphone du contact");
    let mut numero = String::new();
    std::io::stdin().read_line(&mut numero).expect("Erreur de lecture");


    let new_contact = Contact {
        nom: nom.trim().to_string(),
        telephone: numero.trim().to_string(),
    };
    contacts.push(new_contact);
}

// méthode afficher_contacts
fn afficher_contacts(contacts: &mut Vec<Contact>){
    println!("---------------");
    contacts.sort_by(|c1, c2| c1.nom.cmp(&c2.nom));

    for c in contacts.iter() {
        println!("Nom: {}Telephone: {}", c.nom, c.telephone);
    }

    println!("---------------");
    println!("Nombre de contacts: {}", contacts.len())
}   

fn main() {
    let mut contacts: Vec<Contact> = Vec::new();

    loop {
        println!("\n=== MENU ===");
        println!("1. Ajouter un contact");
        println!("2. Afficher tous les contacts");
        println!("3. Exporter contact");
        println!("4. Quitter");

        let mut choix = String::new();
        std::io::stdin().read_line(&mut choix).expect("Erreur de lecture");
        let choix = choix.trim();

        // matching selon le choix, d'appeler la bonne méthode.
        match choix {
            "1" => ajouter_contact(&mut contacts),
            "2" => afficher_contacts(&mut contacts),
            "3" => exporter_contacts(&mut contacts),
            "4" => break,
            _ => {
                println!("Echec")
            }
        }
    }
}

fn exporter_contacts(contacts: &mut Vec<Contact>) {
    // export to csv

   let mut file = match  File::create("contacts.csv") {
       Ok(file) => { file }
       Err(_) => {
           panic!("Erreur exportation contacts");
       }
   } ;

    let columns = "name,telephone\n";
    if let Err(e) = file.write_all(columns.as_bytes()) {
        panic!("Erreur exportation contacts: {}", e);
    }

    for c in contacts {
        let line = format!("{},{}\n", c.nom.trim(), c.telephone.trim());
        if let Err(e) = file.write_all(line.as_bytes()) {
            panic!("Erreur de contact: {}", e);
        }
    }

    println!("Export finished!")
}
