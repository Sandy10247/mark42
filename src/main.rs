use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::stdin;

// Types
type ExecuteFn = fn() -> ();

// Utils
fn reverse_string(curr: String) -> String {
    curr.chars().rev().collect::<String>()
}

// Ops

fn linear_search() {
    println!("Linear Search : 🔍");
    println!("Enter text :- ");
    let user_input = read_string();

    println!("Enter Search Item :- ");
    let search_item = read_string();

    match user_input
        .split(" ")
        .map(|x| x.to_string())
        .find(|x| *x == search_item)
    {
        Some(_) => println!("Found :- {search_item:?}"),
        None => println!("Not Found"),
    };
}

fn delete_string_chars() {
    println!("Check for Palindrome : ");
    println!("Enter text :- ");
    let user_input = read_string();

    println!("Enter postion :- ");
    let user_desired_pos = read_number();

    println!("Enter delete count :- ");
    let delete_count = read_number();

    let mut res_string = String::new();

    for (i, item) in user_input.chars().enumerate() {
        if i < user_desired_pos as usize || i > (user_desired_pos as usize + delete_count as usize)
        {
            res_string.push_str(item.to_string().as_str());
        }
    }

    println!("Result String :- {res_string}");
}

fn check_for_palindrome() {
    println!("Check for Palindrome : ");
    println!("Enter text :- ");
    let user_input = read_string();
    let reverded = reverse_string(user_input.trim().to_string());
    match user_input.trim().cmp(&reverded) {
        Ordering::Less => println!("Not Pallindrome"),
        Ordering::Greater => println!("Not Pallindrome"),
        Ordering::Equal => {
            println!("Pallindrome");
        }
    }
}

fn binary_to_decimal() {
    println!("Decimal to Binary OP : ");
    println!("Enter a Binary :- ");
    let mut num: i32 = read_string()
        .trim()
        .parse()
        .expect("Failed to Read Binary to i32");
    let mut result = 0;
    let mut multiplier = 1;
    while num != 0 {
        let last = num % 10;
        result += last * multiplier;
        multiplier *= 2;
        num /= 10;
    }

    println!("result :- {}", result);
}

fn decimal_to_binary() {
    println!("Decimal to Binary OP : ");
    println!("Enter an Integer :- ");
    let mut num = read_number();
    let mut result = String::from("");

    while num > 0 {
        result.push_str((num % 2).to_string().as_str());
        num = num / 2;
    }

    println!("Result :- {}", reverse_string(result));
}

fn read_string() -> String {
    let mut user_input = String::new();

    stdin()
        .read_line(&mut user_input)
        .expect("reading input failed");

    user_input.trim().to_string()
}

fn read_number() -> i32 {
    let mut user_input = String::new();

    stdin()
        .read_line(&mut user_input)
        .expect("reading input failed");

    user_input.trim().parse().expect("Failed to read a Number")
}

fn main() {
    println!(":::::::::::::::::: KSPK's Programmes :::::::::::::::::::::::::");
    println!("1. Decimal to Binary");
    println!("2. Binary to Decimal");
    println!("3. Check for Palindrome");
    println!("4. Delete String chars");
    println!("5. Linear Search");

    let mut op_fn_map: HashMap<String, ExecuteFn> = HashMap::new();

    op_fn_map.insert(String::from("1"), decimal_to_binary);
    op_fn_map.insert(String::from("2"), binary_to_decimal);
    op_fn_map.insert(String::from("3"), check_for_palindrome);
    op_fn_map.insert(String::from("4"), delete_string_chars);
    op_fn_map.insert(String::from("5"), linear_search);

    let user_input = read_number();
    let execute = op_fn_map.get(&user_input.to_string()).unwrap();

    execute();
}
