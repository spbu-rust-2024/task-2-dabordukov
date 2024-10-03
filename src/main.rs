use std::io;

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    let chars: Vec<char> = input.chars().collect();
    let mut borders: (usize, usize) = (0, 0);
    find_longest_palindrome(&chars, &mut borders);
    let mut i = borders.0;
    loop {
        print!("{}", chars[i]);
        i += 1;
        if i > borders.1 {
            break;
        }
    }
}

fn find_longest_palindrome(string: &Vec<char>, borders: &mut (usize, usize)) {
    let mut max_length = 1;
    let mut left_saved = 0;
    let mut right_saved = 0;

    let mut i = 1;
    let mut current_length = 1;
    while i < string.len() {
        let mut left = i - 1;
        let mut right = i + 1;
        while right < string.len() {
            if string[left] == string[right] {
                current_length += 2;
            } else {
                left += 1;
                right -= 1;
                break;
            }

            if left == 0 {
                break;
            }

            left -= 1;
            right += 1;
        }
        if current_length > max_length {
            max_length = current_length;
            left_saved = left;
            right_saved = right;
        }

        current_length = 0;
        left = i - 1;
        right = i;
        while right < string.len() {
            if string[left] == string[right] {
                current_length += 2;
            } else {
                left += 1;
                right -= 1;
                break;
            }

            if left == 0 {
                break;
            }

            left -= 1;
            right += 1;
        }

        if current_length > max_length {
            max_length = current_length;
            left_saved = left;
            right_saved = right;
        }

        i += 1;
    }

    borders.0 = left_saved;
    borders.1 = right_saved;
}
