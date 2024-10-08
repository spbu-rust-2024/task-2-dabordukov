use std::io;

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    let chars: Vec<u8> = input.chars().map(|x| x as u8).collect();
    let mut borders: (usize, usize) = (0, 0);
    if chars.len() == 0 {
        println!("");
        return;
    }

    find_longest_palindrome(&chars, &mut borders, true);
    find_longest_palindrome(&chars, &mut borders, false);

    for i in borders.0..borders.1 + 1 {
        print!("{}", chars[i] as char);
    }
}

fn find_longest_palindrome(string: &Vec<u8>, borders: &mut (usize, usize), even: bool) {
    let mut max_length = borders.1 - borders.0 + 1;
    let mut left_saved = borders.0;
    let mut right_saved = borders.1;

    let mut i = 0;
    while i < string.len() - 1 {
        let mut left = i;
        let mut right = i;
        if even {
            right += 1;
        }
        loop {
            if string[left] != string[right] {
                if right - left >= 2 {
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

        if right - left + 1 > max_length && string[right] == string[left] {
            max_length = right - left + 1;
            left_saved = left;
            right_saved = right;
        }

        i += 1;
    }

    borders.0 = left_saved;
    borders.1 = right_saved;
}
