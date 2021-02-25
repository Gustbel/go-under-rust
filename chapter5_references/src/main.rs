use std::collections::HashMap;
type Table = HashMap<String, Vec<String>>;



fn main() {

    // Ownership y References: Shared Reference vs Mutable Reference:
    // En el primer ejemplo estamos dando ownership al dato que pasamos y lo destruimos, 
    // en el segundo estamos pasando una referencia y NO lo destruimos

    fn show_1(table: Table) {
        println!("  Funcion Ownership example");
        for (artist, works) in table {
            println!("    works by {}:", artist);
            for work in works {
                println!("      {}", work);
            }
        }
        println!("");
    }

    fn show_2(table: &Table) {
        println!("  Funcion Shared Reference example");
        for (artist, works) in table {
            println!("    works by {}:", artist);
            for work in works {
                println!("      {}", work);
            }
        }
        println!("");
    }

    fn show_3(table: &mut Table) {
        println!("  Funcion mut Reference example");

        for (artist, works) in table {
            works.sort();   // Ordena los works de los artistas
            println!("    works by {}:", artist);
            for work in works {
                println!("      {}", work);
            }
        }
        println!("");
    }

    println!("Ownership vs shared References vs mut References:");

    let mut table_1 = Table::new();
    table_1.insert("Gesualdo".to_string(),
                 vec!["many madrigals".to_string(),
                      "Tenebrae Responsoria".to_string()]);
    table_1.insert("Caravaggio".to_string(),
                 vec!["The Musicians".to_string(),
                      "The Calling of St. Matthew".to_string()]);
    table_1.insert("Cellini".to_string(),
                 vec!["Perseus with the head of Medusa".to_string(),
                      "a salt cellar".to_string()]);

    let table_2 = table_1.clone();    //Clonamos table 1 antes de destruirla
    let mut table_3 = table_1.clone();    //Clonamos table 1 antes de destruirla
    show_1(table_1);
    show_2(&table_2);
    show_3(&mut table_3);

    //println!("  Print value after funciton 'show_1':\n    {:?}", table_1);  // Error
    println!("  Print value after funciton 'show_2':\n    {:?}", table_2);  // Correct
    println!("  Print value after funciton 'show_3':\n    {:?}", table_3);  // Correct

}
