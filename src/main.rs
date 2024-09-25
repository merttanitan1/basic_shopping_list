use std::io;
use std::collections::HashMap;

fn main(){
    let mut shopping_list: HashMap<String, u32> = HashMap::new();
    println!("Welcome to the shopping list");
    loop{
        println!("Add some products in shop list (name and quantity):");
        println!("Write 'exit' to close program.");

        let mut product_name = String::new();
        io::stdin().read_line(&mut product_name).expect("Failed to read line");
        let product_name = product_name.trim().to_string();

        if product_name == "exit"{
            break;
        }

        let mut quantity = String::new();
        println!("Enter quantity:");
        io::stdin().read_line(&mut quantity).expect("Failed to read line");
        let quantity: u32 = match quantity.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        shopping_list.entry(product_name.clone())
            .and_modify(|e| *e += quantity)
            .or_insert(quantity);

        println!("Current shopping list");
        for (product, &amount) in &shopping_list{
            println!("{}: {}", product, amount);
        }
    }

    println!("Your Shopping List: {:?}", shopping_list);
}