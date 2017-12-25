use std::fs::File;
use std::io::prelude::*;

pub fn solve_captcha(input: &str, cycle_match: &Fn(&Vec<u32>) -> usize) -> u32 {
    //collect all characters as usigned ints from input
    let numbers = input
        .chars()
        .map(|character| character.to_digit(10))
        .filter_map(|option| option)
        .collect::<Vec<u32>>();

    //zip all parsed integers with itself to get a tuple with next value
    //make the zip target cycle so last element will be paired with first per instruction
    let zip = numbers
        .iter()
        .zip(numbers.iter().cycle().skip(cycle_match(&numbers)));
    zip.fold(0, |result, (&current, &next)| match current == next {
        true => current + result,
        false => result,
    })
}

fn get_input(file_name: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(file_name)?;
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)?;
    Result::Ok(file_content)
}


fn main() {
    let file_name = "inputSource.txt";
    let input = get_input(&file_name).expect("Could not read advent input");
    let result_step_1 = solve_captcha(&input, &skip_first);
    let result_step_2 = solve_captcha(&input, &skip_half);

    println!(
        "Captcha result for Advent input in \"{}\" is {} for step1 and {} for step 2",
        file_name, result_step_1, result_step_2
    );
}

fn skip_first(_: &Vec<u32>) -> usize{
    1
}
fn skip_half(vec: &Vec<u32>) -> usize{
    vec.len() / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step1example_1() {
        let result = solve_captcha("1122", &skip_first);
        assert_eq!(3, result);
    }
    #[test]
    fn step1example_2() {
        let result = solve_captcha("1111", &skip_first);
        assert_eq!(4, result)
    }
    #[test]
    fn step1example_3() {
        let result = solve_captcha("1234", &skip_first);
        assert_eq!(0, result);
    }
    #[test]
    fn step1example_4() {
        let result = solve_captcha("91212129", &skip_first);
        assert_eq!(9, result);
    }

    #[test]
    fn step2example_1(){
        let result = solve_captcha("1212", &skip_half);
        assert_eq!(6, result);
    }
    #[test]
    fn step2example_2(){
        let result = solve_captcha("1221", &skip_half);
        assert_eq!(0, result);
    }

    #[test]
    fn step2example_3(){
        let result = solve_captcha("123425", &skip_half);
        assert_eq!(4, result);
    }

    #[test]
    fn step2example_4(){
        let result = solve_captcha("123123", &skip_half);
        assert_eq!(12, result);
    }

    #[test]
    fn step2example_5(){
        let result = solve_captcha("12131415", &skip_half);
        assert_eq!(4, result);
    }

    #[test]
    fn can_read_input_file() {
        let file_content = get_input("inputSource.txt");
        assert!(file_content.is_ok());
    }
}
