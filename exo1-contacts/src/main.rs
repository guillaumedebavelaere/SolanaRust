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
        nom: nom,
        telephone: numero
    };
    contacts.push(new_contact);
}

// méthode afficher_contacts
fn afficher_contacts(contacts: &Vec<Contact>){
    for c in contacts {
        println!("Nom: {} Telephone: {}", c.nom, c.telephone);
    }
}   

fn main() {
    let mut contacts: Vec<Contact> = Vec::new();

    loop {
        println!("\n=== MENU ===");
        println!("1. Ajouter un contact");
        println!("2. Afficher tous les contacts");
        println!("3. Quitter");

        let mut choix = String::new();
        std::io::stdin().read_line(&mut choix).expect("Erreur de lecture");
        let choix = choix.trim();

        // matching selon le choix, d'appeler la bonne méthode.
        match choix {
            "1" => ajouter_contact(&mut contacts),
            "2" => afficher_contacts(&contacts),
            _ => {
                println!("Echec")
            }
        }
    }
}
