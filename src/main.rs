use std::io;

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    let chars: Vec<char> = input.chars().collect();
    let mut borders: (usize, usize) = (0, 0);
    if chars.len() == 0 {
        println!("");
        return;
    }
    find_longest_palindrome(&chars, &mut borders);
    for i in borders.0..borders.1 + 1 {
        print!("{}", chars[i]);
    }
}

fn find_longest_palindrome(string: &Vec<char>, borders: &mut (usize, usize)) {
    let mut max_length = 1;
    let mut left_saved = 0;
    let mut right_saved = 0;

    let mut i = 0;
    while i < string.len() - 1 {
        let mut left = i;
        let mut right = i;
        loop {
            if string[left] != string[right] {
                if left != right {
                    left += 1;
                    right -= 1;
                }
                break;
            }

            if left == 0 || right == string.len() - 1 {
                break;
            }

            left -= 1;
            right += 1;
        }

        if right - left + 1 > max_length {
            max_length = right - left + 1;
            left_saved = left;
            right_saved = right;
        }

        left = i;
        right = i + 1;
        loop {
            if string[left] != string[right] {
                if right - left != 1 {
                    right -= 1;
                    left += 1;
                }
                break;
            }

            if left == 0 || right == string.len() - 1 {
                break;
            }

            left -= 1;
            right += 1;
        }

        if right - left + 1 > max_length {
            max_length = right - left + 1;
            left_saved = left;
            right_saved = right;
        }

        i += 1;
    }

    borders.0 = left_saved;
    borders.1 = right_saved;
}
