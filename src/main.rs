use crate::{
    employees::{Department, Employee, EmployeeRegistry},
    strings::{pig_latin, CanBeVowel},
};
use std::{collections::HashMap, str::FromStr};

mod employees;
mod stats;
mod strings;

fn main() {
    println!(
        "Mean    (i32 Array): {}",
        stats::mean::<i32>(&[1, 4, 2, 3, 5]).unwrap_or_default()
    );
    println!(
        "Mean   (f64 Vector): {}",
        stats::mean::<f64>(&vec![1.5, 4.7, 9.9, 3.5, 7.2, 9.1, 5.3]).unwrap_or_default()
    );
    println!(
        "Mean             (): {}\n",
        stats::mean::<f64>(&vec![]).unwrap_or_default()
    );

    println!(
        "Median (f64 Vector): {}",
        stats::median::<f64>(&vec![1.5, 4.7, 9.9, 3.5, 7.2, 9.1, 101.7, 5.3]).unwrap_or_default()
    );
    println!(
        "Median (u32 Vector): {}",
        stats::median::<i32>(&vec![1, 4, 2, 3, 5, 900_000_000]).unwrap_or_default()
    );
    println!(
        "Median           (): {}\n",
        stats::median::<i32>(&vec![]).unwrap_or_default()
    );

    println!(
        "Mode   (f32 Vector): {}",
        stats::mode::<f32>(&vec![1.0, 4.0, 4.5, 4.5, 7.4, 3.9, 10.2, 5.1]).unwrap_or_default()
    );
    println!(
        "Mode     (i8 Array): {}",
        stats::mode::<i8>(&[1, 4, 2, 3, 5]).unwrap_or_default()
    );
    println!(
        "Mode             (): {}\n",
        stats::mode::<i8>(&[]).unwrap_or_default()
    );

    println!(
        "Char is vowel (a): {}",
        char::from_str("a").unwrap_or_default().is_vowel(),
    );
    println!(
        "Char is vowel (A): {}",
        char::from_str("A").unwrap_or_default().is_vowel(),
    );
    println!(
        "Char is vowel (s): {}",
        char::from_str("s").unwrap_or_default().is_vowel(),
    );
    println!(
        "Char is vowel (I): {}",
        char::from_str("I").unwrap_or_default().is_vowel(),
    );
    println!(
        "Char is vowel  (): {}\n",
        char::from_str("").unwrap_or_default().is_vowel(),
    );

    println!(
        "Pig Latin (Apples to apples): {}",
        pig_latin("Apples to apples").unwrap_or_default(),
    );
    println!(
        "Pig Latin (apples to apples): {}",
        pig_latin("apples to apples").unwrap_or_default(),
    );
    println!(
        "Pig Latin (charging battery): {}",
        pig_latin("charging battery").unwrap_or_default(),
    );
    println!(
        "Pig Latin     (Rust is cool): {}",
        pig_latin("Rust is cool").unwrap_or_default(),
    );
    println!(
        "Pig Latin                 (): {}\n",
        pig_latin("").unwrap_or_default(),
    );

    let mut registry = EmployeeRegistry::new(HashMap::new());
    println!(
        "Empty Employee Registry            : {:?}",
        registry.get_employees(Department::default())
    );

    registry.add_employee(&Employee::new("Alex"), &Department::Engineering);
    println!(
        "Employee Registry after adding     : {:?}",
        registry.get_employees(Department::default())
    );

    registry.add_employees(&[
        (Employee::new("Susan"), Department::default()),
        (Employee::new("Bob"), Department::Sales),
        (Employee::new("Jim"), Department::Engineering),
    ]);
    println!(
        "Employee List after adding multiple: {:?}",
        registry.get_employees(Department::default())
    );

    println!(
        "Employee List         (Engineering): {:?}",
        registry.get_employees(Department::Engineering)
    );
    println!(
        "Employee List               (Sales): {:?}",
        registry.get_employees(Department::Sales)
    );
    println!(
        "Employee List                 (All): {:?}\n",
        registry.get_employees(Department::default())
    );
}
