struct Contact {
    nom: String,
    telephone: String
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

        match choix {
            "1" => ajouter_contact(&mut contacts),
            "2" => afficher_contacts(&contacts),
            _ => println!("echec")
        }
    }
}

fn ajouter_contact(contacts: &mut Vec<Contact>) {
    println!("Ajouter contact");


    let mut nom = String::new();
    std::io::stdin().read_line(&mut nom).expect("Erreur de lecture");

    let mut telephone = String::new();
    std::io::stdin().read_line(&mut telephone).expect("Erreur de lecture");

    contacts.push(Contact{nom: nom.to_string(), telephone: telephone.to_string()})
}

fn afficher_contacts(contacts: &Vec<Contact>) {
    for c in contacts {
        println!("nom: {}, telephone: {}", c.nom, c.telephone);
    }
}