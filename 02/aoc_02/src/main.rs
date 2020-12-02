use std::fs;

fn parse_password_db_string<'a>(
  passwords_db_string: &'a String
) -> Vec<(usize, usize, &'a str, &'a str)>
{
  let lines = passwords_db_string.lines();
  let mut passwords: Vec<(usize, usize, &str, &str)> = Vec::new();
  for line in lines {
    let tokens: Vec<&str> = line.split_whitespace().collect();
    assert_eq!(tokens.len(), 3);
    let range_str = tokens[0];
    let char_str = tokens[1];
    let password = tokens[2];

    let range_tokens: Vec<&str> = range_str.splitn(2, "-").collect();
    assert_eq!(range_tokens.len(), 2);
    let range = (
      range_tokens[0].parse::<usize>().unwrap(),
      range_tokens[1].parse::<usize>().unwrap(),
    );

    let target_char = char_str.trim_end_matches(":");

    passwords.push((range.0, range.1, target_char, password));
  }

  passwords
}

fn count_valid_passwords1(passwords: &Vec<(usize, usize, &str, &str)>) -> usize {
  let mut count: usize = 0;
  for password_tuple in passwords {
    let occurrences: usize = password_tuple.3.matches(password_tuple.2).count();
    if occurrences >= password_tuple.0 && occurrences <= password_tuple.1 {
      count += 1;
    }
  }
  count
}

fn count_valid_passwords2(passwords: &Vec<(usize, usize, &str, &str)>) -> usize {
  let mut count: usize = 0;
  for password_tuple in passwords {
    let password_len = password_tuple.3.len();
    let target: char = password_tuple.2.chars().next().unwrap();
    if password_tuple.0 <= password_len && password_tuple.1 <= password_len {
      let first_matches = password_tuple.3.chars().nth(password_tuple.0 - 1).unwrap() == target;
      let second_matches = password_tuple.3.chars().nth(password_tuple.1 - 1).unwrap() == target;
      let mut result = first_matches;
      result ^= second_matches;
      if result {
        count += 1;
      }
    }
  }
  count
}

fn main() {
  let password_db = fs::read_to_string("password_db1.txt").unwrap();

  let passwords = parse_password_db_string(&password_db);

  println!("{:?}", count_valid_passwords1(&passwords));
  println!("{:?}", count_valid_passwords2(&passwords));
}
