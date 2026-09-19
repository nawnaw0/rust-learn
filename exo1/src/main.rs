fn main() {
    println!("Hello, world!");
    let prix: f64 = 25.0; //f64 c pour le type double
    let quantite: u32 = 4; //u32 c pour pos ou nul
    let reduction: f64 = 10.0;

    println!("Prix : {}", prix);
    println!("Quantité : {}", quantite);
    println!("Réduction : {}", reduction);
    println!("{} + {} = {}", 2, 3, 5); 

    let total = prix * quantite as f64; //on converti u32 en f64 pour pas avoir d'erreur au niveau du compilateur

    println!("Total : {}", total);

    let montant_reduc = prix * reduction /100.0;
    println!("Réduction : {}", montant_reduc);

    let prix_final: f64 = total - montant_reduc;

    println!("Prix final : {}", prix_final);

    


    
}
