fn main() {
    let prix_entrer: f64 = 100.0;
    let prix_actuel: f64 = 115.0;
    let quantite: f64 = 2.0;
    let compte: bool = true;

    let pnl = (prix_actuel - prix_entrer) * quantite;
    println!("Pnl : {}", pnl);

    if pnl > 0.0 {
        println!("Position gagnante");
    } else if pnl < 0.0 {
        println!("Position perdante");
    } else {
        println!("Break even");
    }
    if pnl >= 20.0 && compte {
        println!("Retrait des profits autorisé");
    } else {
        println!("Retrait non autorisé");
    }

}
