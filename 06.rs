fn longest_common_prefix(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }

    let first_string = &strings[0];
    let mut longest_prefix = String::new();

    'outer: for (i, char) in first_string.chars().enumerate() {
        for string in &strings[1..] {
            if let Some(other_char) = string.chars().nth(i) {
                if char != other_char {
                    break 'outer;
                }
            } else {
                break 'outer;
            }
        }
        longest_prefix.push(char);
    }

    longest_prefix
}

fn main() {
    let strings = vec![
        String::from("flower"),
        String::from("flow"),
        String::from("flight"),
    ];

    println!("Longest common prefix: {}", longest_common_prefix(&strings));
}
