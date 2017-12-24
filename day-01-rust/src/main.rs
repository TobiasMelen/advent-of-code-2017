
use std::fs::File;
use std::io::prelude::*;

pub fn solve_captcha(input: &str) -> u32 {
    //collect all characters as usigned ints from input
    let numbers = input
        .chars()
        .map(|character| character.to_digit(10))
        .filter_map(|option| option)
        .collect::<Vec<u32>>();

    //zip all parsed integers with itself to get a tuple with next value
    //make the zip target cycle so last element will be paired with first per instruction
    let zip = numbers.iter().zip(numbers.iter().cycle().skip(1));
    zip.fold(0, |result, (&current, &next)| match current == next {
        true => current + result,
        false => result,
    })
}

fn get_input(file_name: &str) -> Result<String, std::io::Error>{
    let mut file = File::open(file_name)?;
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)?;
    Result::Ok(file_content)
}


fn main() {
    let file_name = "inputSource.txt";
    let input = get_input(&file_name).expect("Could not read advent input");
    let result = solve_captcha(&input[..]);
    println!("Captcha result for Advent input in \"{}\" is {}", file_name, result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn website_example_1() {
        let result = solve_captcha("1122");
        assert_eq!(3, result);
    }
    #[test]
    fn website_example_2() {
        let result = solve_captcha("1111");
        assert_eq!(4, result)
    }
    #[test]
    fn website_example_3() {
        let result = solve_captcha("1234");
        assert_eq!(0, result);
    }
    #[test]
    fn website_example_4() {
        let result = solve_captcha("91212129");
        assert_eq!(9, result);
    }
    #[test]
    fn can_read_input_file(){
        let file_content = get_input("inputSource.txt");
        assert!(file_content.is_ok());
    }
}
