use std::{collections::HashMap, fmt::Display};

pub fn mean<T>(numbers: &[T]) -> Option<f64>
where
    T: Into<f64> + Copy,
{
    let sum = numbers
        .iter()
        .fold(0., |acc: f64, num| acc + Into::<f64>::into(*num));
    if numbers.len() == 0 {
        None
    } else {
        Some(sum / numbers.len() as f64)
    }
}

pub fn median<T>(numbers: &[T]) -> Option<f64>
where
    T: Into<f64> + Copy,
{
    let mut sorted = numbers
        .iter()
        .map(|num| Into::<f64>::into(*num))
        .collect::<Vec<f64>>();
    sorted.sort_unstable_by(|a, b| a.total_cmp(&b));

    if sorted.len() == 0 {
        None
    } else if sorted.len() == 1 {
        Some(sorted[0])
    } else if sorted.len() % 2 == 0 {
        let middle_index = sorted.len() as f64 / 2.;
        Some(
            mean::<f64>(&vec![
                sorted[(middle_index - 1.) as usize],
                sorted[middle_index as usize],
            ])
            .unwrap(),
        )
    } else {
        let middle_index = sorted.len() as f64 / 2.;
        Some(sorted[middle_index.floor() as usize])
    }
}

pub fn mode<T>(numbers: &[T]) -> Option<f64>
where
    T: Display + Copy,
{
    let mut number_count = HashMap::new();

    for num in numbers {
        *number_count.entry(format!("{}", num)).or_insert(0) += 1;
    }

    match number_count.iter().max_by(|a, b| a.1.cmp(&b.1)) {
        Some(tuple) => Some(tuple.0.parse::<f64>().unwrap()),
        None => None,
    }
}
