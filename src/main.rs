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

fn get_largest(list: Vec<usize>) -> usize {
    *list
        .iter()
        .max_by(|a, b| {
            return a.partial_cmp(b).unwrap();
        })
        .unwrap()
}

fn gen_primes(start: usize, count: usize) -> Vec<usize> {
    let mut primes_found: Vec<usize> = vec![];
    let mut start_iter = start;

    fn is_prime(num: usize) -> bool {
        let num_sqrt = (num as f64).sqrt() as usize;
        for el in 2..=num_sqrt {
            if num % el == 0 {
                return false;
            }
        }

        true
    }
    while primes_found.len() < count {
        if is_prime(start_iter) {
            primes_found.push(start_iter);
        }
        start_iter += 1;
    }

    primes_found
}

fn binary_search<T: std::cmp::Ord + std::fmt::Display>(
    list: &[T],
    search_term: T,
) -> Option<usize> {
    let length: usize = list.len();
    let mut mid = length / 2;
    let mut high = length - 1;
    let mut low = 0;
    let mut curr = &list[mid];

    while low <= high {
        match curr.cmp(&search_term) {
            Ordering::Equal => return Some(mid),
            Ordering::Greater => low = mid + 1,
            Ordering::Less => high = mid - 1,
        }

        mid = (low + high) / 2;
        curr = &list[mid];
    }

    None
}

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

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_binary_search_ints() {
        assert_eq!(binary_search(&vec![1, 2, 3, 4, 5], 3), Some(2));
    }

    #[test]
    fn test_primes_generate() {
        assert_eq!(gen_primes(2, 4), vec![2, 3, 5, 7])
    }

    #[test]
    fn test_primes_generate_max() {
        assert_eq!(
            gen_primes(2, 168),
            vec![
                2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79,
                83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167,
                173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257,
                263, 269, 271, 277, 281, 283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353,
                359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421, 431, 433, 439, 443, 449,
                457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541, 547, 557, 563,
                569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619, 631, 641, 643, 647, 653,
                659, 661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743, 751, 757, 761,
                769, 773, 787, 797, 809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 863, 877,
                881, 883, 887, 907, 911, 919, 929, 937, 941, 947, 953, 967, 971, 977, 983, 991,
                997
            ]
        )
    }

    #[test]
    fn test_get_largest() {
        assert_eq!(get_largest(vec![12, 20, 30, 22, 200, 100, 10, 2]), 1200);
    }
    // #[test]
    // fn test_bad_add() {
    //     // This assert would fire and test will fail.
    //     // Please note, that private functions can be tested too!
    //     assert_eq!(bad_add(1, 2), 3);
    // }
}
