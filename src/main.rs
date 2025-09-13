fn main() {
    let array: [i32 ; 4] = [1, 2, 3, 4];
    println!("Array: {:?}", array);

    let fruits: [&str ; 4] = ["apples", "bananas", "pears", "blueberries"];
    println!("Fruits array: {:?}", fruits);
    println!("Fruits array first element: {}", fruits[0]);

    let human: (String, i32, bool) = ("Alice".to_string(), 30, false); // "Alice" is not a string. 
    println!("Human: {:?}", human);

    let my_mix_tuples = ("Alice", 30, false, [1, 2, 23, 4, 5]);
    println!("Mixed tuple with a compound data type: {:?}", my_mix_tuples);
    println!("here: {}", my_mix_tuples.3[2]);

    let number_slices: &[i32; 3] = &[1, 4, 7];
    println!("number_slice: {:?}", number_slices);

    let pokemon_slices: &[&str; 3] = &["bulbasaur", "charmander", "squirtle"];
    println!("pokemon_slices: {:?}", pokemon_slices);

    let book_slices: &[&String; 2] = &[&"Eragon".to_string(), &"Attracting through honesty".to_string()];
    println!("book_slices: {:?}", book_slices);

    let mut inspiration: String = String::from("I'm gonna make it");
    inspiration.push_str("\nWhy not me?");
    println!("inspiration: {}", inspiration);

    let string: String = String::from("Hello World!");
    let slice: &str = &string[6..11];
    println!("string slice: {}", slice);
}
