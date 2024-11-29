use std::cmp::Ordering;
use std::io::stdin;

type Matrix = Vec<Vec<i32>>;

// #region Utils
fn reverse_string(curr: String) -> String {
    let mut rev_split = curr.split("").map(String::from).collect::<Vec<String>>();
    rev_split.reverse();
    return rev_split.join("");
}
fn print_mat(mat: &Matrix) {
    for i in 0..mat.len() {
        for j in 0..mat[i].len() {
            print!("{} ", mat[i][j])
        }
        println!();
    }
}

fn fill_mat(source_matrix: &Matrix) -> Matrix {
    let mut result_matrix: Matrix = vec![];
    for i in 0..source_matrix.len() {
        let mut col_vector = vec![];
        for j in 0..source_matrix[i].len() {
            col_vector.push(0);
        }
        result_matrix.push(col_vector);
    }
    result_matrix
}
// #endregion

// Ops

fn get_sum_mul_of_digits(num: i32) -> (i32, i32) {
    let mut n = num;
    let mut sum = 0;
    let mut mul = 1;
    while (n > 0) {
        let last_digit = n % 10;
        sum += last_digit;
        mul *= last_digit;
        n /= 10;
    }

    (sum, mul)
}

fn get_trace_of_matrix(m1: &Matrix) -> i32 {
    let mut trace = 0;
    for i in 0..m1.len() {
        for j in 0..m1[i].len() {
            if i == j {
                trace += m1[i][j];
            }
        }
    }
    trace
}

fn get_matrix_transpose(m1: &Matrix) -> Matrix {
    let mut res_matrix = fill_mat(m1);

    for i in 0..m1.len() {
        for j in 0..m1[i].len() {
            res_matrix[j][i] = m1[i][j];
        }
    }

    res_matrix
}

fn check_matrix_symmetric(m1: &Matrix, m2: &Matrix) -> bool {
    for i in 0..m1.len() {
        for j in 0..m1[i].len() {
            if m1[i][j] != m2[i][j] {
                return false;
            }
        }
    }

    return true;
}

fn get_fib_series(count: usize) -> Vec<i32> {
    fn fib(a: usize) -> i32 {
        if a < 2 {
            return a as i32;
        }

        return fib(a - 1) + fib(a - 2);
    }

    let mut res_vec = vec![];
    for i in 1..=count {
        res_vec.push(fib(i));
    }

    res_vec
}

fn get_reverse_string(source_str: String) -> String {
    reverse_string(source_str)
}

fn get_min_max_vec(arr: Vec<i32>) -> (i32, i32) {
    arr.iter().fold((arr[0], arr[0]), |mut acc, curr| {
        if *curr < acc.0 {
            acc.0 = *curr
        }

        if *curr > acc.1 {
            acc.1 = *curr
        }
        return acc;
    })
}

fn get_upper_triangle_matrix(mut source_matrix: &Matrix) -> Matrix {
    let mut result_matrix: Matrix = fill_mat(source_matrix);
    for i in 0..source_matrix.len() {
        for j in 0..source_matrix[i].len() {
            result_matrix[i][j] = if i >= j { source_matrix[i][j] } else { 0 }
        }
    }
    result_matrix
}

fn get_lower_triangle_matrix(mut source_matrix: &Matrix) -> Matrix {
    let mut result_matrix: Matrix = fill_mat(source_matrix);
    for i in 0..source_matrix.len() {
        for j in 0..source_matrix[i].len() {
            result_matrix[i][j] = if i <= j { source_matrix[i][j] } else { 0 }
        }
    }
    result_matrix
}

fn get_sorted_list(l: Vec<usize>) -> Vec<usize> {
    let mut list = l.clone();
    list.sort_by(|a, b| b.cmp(a));
    list
}

fn get_1s_compliment(num: usize) -> String {
    get_binary(num as i32)
        .chars()
        .fold(String::new(), |acc, curr| {
            return acc + if curr == '0' { "1" } else { "0" };
        })
}
fn get_2s_compliment(num: usize) -> String {
    let mut ones_compliment = get_1s_compliment(num);
    println!("ones compliment :- {ones_compliment}");
    let last = ones_compliment.pop().unwrap();
    ones_compliment + if last == '0' { "1" } else { "0" }
}

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

fn linear_search<T: std::cmp::Ord + std::fmt::Display>(list: &[T], search_term: T) -> Option<&T> {
    list.iter().find(|&x| *x == search_term)
}

fn delete_string_chars(curr_string: String, start: usize, count: usize) -> String {
    let mut res_string = String::new();
    for (i, item) in curr_string.chars().enumerate() {
        if i < start as usize || i > (start as usize + count as usize) {
            res_string.push_str(item.to_string().as_str());
        }
    }
    res_string
}

fn is_pallindrome(curr_str: String) -> bool {
    let reverded = reverse_string(curr_str.trim().to_string());
    match curr_str.trim().cmp(&reverded) {
        Ordering::Less => false,
        Ordering::Greater => false,
        Ordering::Equal => true,
    }
}

fn binary_to_decimal(num_str: String) -> i32 {
    let mut num: i32 = num_str
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

    result
}

fn get_binary(n: i32) -> String {
    let mut result = String::from("");
    let mut num = n;

    while num > 0 {
        result.push_str((num % 2).to_string().as_str());
        num = num / 2;
    }

    reverse_string(result)
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

fn main() {}

#[cfg(test)]
mod tests {
    use std::vec;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_linear_search() {
        assert_eq!(linear_search(&vec![1, 10, 230, 20, -1, 30], -1), Some(&-1));
    }

    #[test]
    fn test_linear_search_fail() {
        assert_eq!(linear_search(&vec![1, 10, 230, 20, -1, 30], 1001), None);
    }

    #[test]
    fn test_delete_string_chars() {
        assert_eq!(
            delete_string_chars("Hello World".to_string(), 2, 2),
            "He World"
        );
    }

    #[test]
    fn test_is_pallindrome() {
        assert_eq!(is_pallindrome("abba".to_string()), true);
    }

    #[test]
    fn test_binary_to_decimal() {
        assert_eq!(binary_to_decimal("1010".to_string()), 10);
    }

    #[test]
    fn test_get_test() {
        assert_eq!(get_binary(10), "1010");
    }

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
        assert_eq!(get_largest(vec![12, 20, 30, 22, 200, 100, 10, 2]), 200);
    }

    #[test]
    fn test_get_1s_compliment() {
        assert_eq!(get_1s_compliment(10), "0101");
    }

    #[test]
    fn test_get_2s_compliment() {
        assert_eq!(get_2s_compliment(10), "0100");
    }

    #[test]
    fn test_get_sorted_list() {
        assert_eq!(get_sorted_list(vec![0, 1, 20, 4]), vec![20, 4, 1, 0]);
    }

    #[test]
    fn test_get_lower_triangle_matrix() {
        assert_eq!(
            get_lower_triangle_matrix(&vec![
                vec![1, 2, 3, 4],
                vec![5, 6, 7, 8],
                vec![9, 10, 11, 12],
                vec![13, 14, 15, 16],
            ]),
            vec![
                vec![1, 2, 3, 4],
                vec![0, 6, 7, 8],
                vec![0, 0, 11, 12],
                vec![0, 0, 0, 16],
            ]
        );
    }

    #[test]
    fn test_get_upper_triangle_matrix() {
        assert_eq!(
            get_upper_triangle_matrix(&vec![
                vec![1, 2, 3, 4],
                vec![5, 6, 7, 8],
                vec![9, 10, 11, 12],
                vec![13, 14, 15, 16],
            ]),
            vec![
                vec![1, 0, 0, 0],
                vec![5, 6, 0, 0],
                vec![9, 10, 11, 0],
                vec![13, 14, 15, 16],
            ]
        );
    }

    #[test]
    fn test_get_min_max() {
        assert_eq!(
            get_min_max_vec(vec![20, 10, 1, 20, 30, -1, 20, 200]),
            (-1, 200)
        )
    }

    #[test]
    fn test_rev_string() {
        assert_eq!(
            get_reverse_string(String::from("1234 💔🥶🤠")),
            String::from("🤠🥶💔 4321")
        );
    }

    #[test]
    fn test_get_fib_series() {
        assert_eq!(get_fib_series(4), vec![1, 1, 2, 3]);
    }

    #[test]
    fn test_check_matrix_symmetric() {
        assert_eq!(
            check_matrix_symmetric(
                &vec![vec![1, 2, 3], vec![1, 2, 3], vec![1, 2, 3]],
                &vec![vec![1, 2, 3], vec![1, 2, 3], vec![1, 2, 3]]
            ),
            true
        );
    }

    #[test]
    fn test_get_matrix_transpose() {
        //  [[1, 4, 7], [2, 5, 8], [3, 6, 9]]
        assert_eq!(
            get_matrix_transpose(&vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]]
        );
    }

    #[test]
    fn test_get_trace_of_matrix() {
        assert_eq!(
            get_trace_of_matrix(&vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            15
        );
    }

    #[test]
    fn test_get_sum_mul_of_digits() {
        assert_eq!(get_sum_mul_of_digits(1234), (10, 24));
    }
}
