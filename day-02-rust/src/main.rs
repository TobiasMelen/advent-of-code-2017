use std::fs::File;
use std::io::prelude::*;

fn main() {
    let file_name = "inputSource.txt";
    let input = read_file(file_name).expect("Could not read input source");
    let formatted_input = format_input(&input);
    let checksum1 = calculate_checksum(&formatted_input, &calculate_row_checksum_step_1).unwrap();

    let checksum2 = calculate_checksum(&formatted_input, &calculate_row_checksum_step_2).unwrap();
    println!(
        "Checksum result from source \"{}\" for step 1 is {}, for step 2 it's {}",
        file_name, checksum1, checksum2
    );
}

fn calculate_checksum(
    source: &Vec<Vec<u32>>,
    row_calculator: &Fn(&Vec<u32>) -> Option<u32>,
) -> Option<u32> {
    source.iter().fold(None, |fold, row| {
        let result = row_calculator(row);
        match result {
            Some(value) => Some(
                value + match fold {
                    Some(value) => value,
                    None => 0,
                },
            ),
            None => fold,
        }
    })
}

fn calculate_row_checksum_step_2(source: &Vec<u32>) -> Option<u32> {
    source.iter().fold(None, |result, &outer| match result {
        Some(_) => result,
        None => {
            let inner_even = source
                .iter()
                .find(|&&inner| outer != inner && outer % inner == 0);
            match inner_even {
                Some(inner) => Some(outer / inner),
                None => None,
            }
        }
    })
}


fn calculate_row_checksum_step_1(source: &Vec<u32>) -> Option<u32> {
    let max = source.iter().max()?;
    let min = source.iter().min()?;
    Some(max - min)
}

fn format_input(source: &str) -> Vec<Vec<u32>> {
    let result = source.lines().map(|line| {
        line.split_whitespace()
            .map(|string_value| string_value.parse::<u32>())
            .filter_map(|parse_result| match parse_result {
                Ok(value) => Some(value),
                _ => None,
            })
            .collect::<Vec<u32>>()
    });
    result.collect::<Vec<_>>()
}

fn read_file(file_path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)?;
    Result::Ok(file_content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_checksum_for_example_step1() {
        let example = vec![vec![5, 1, 9, 5], vec![7, 5, 3], vec![2, 4, 6, 8]];
        let result =
            calculate_checksum(&example, &calculate_row_checksum_step_1).expect("didnt work");
        assert_eq!(18, result)
    }

    #[test]
    fn calculate_checksum_for_example_step2() {
        let example = vec![vec![5, 9, 2, 8], vec![9, 4, 7, 3], vec![3, 8, 6, 5]];
        let result =
            calculate_checksum(&example, &calculate_row_checksum_step_2).expect("didnt work");
        assert_eq!(9, result)
    }

    #[test]
    fn format_input_as_nested_vector() {
        let source = "12 3\n4 56";
        let result = format_input(&source);
        assert_eq!(vec![vec![12, 3], vec![4, 56]], result)
    }
}
