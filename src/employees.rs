use std::collections::HashMap;

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct Employee {
    name: String,
}

impl Employee {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
        }
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub enum Department {
    #[default]
    None,
    Engineering,
    Sales,
}

#[derive(Debug)]
pub struct EmployeeRegistry {
    registry: HashMap<Employee, Department>,
}

impl EmployeeRegistry {
    pub fn new(registry: HashMap<Employee, Department>) -> Self {
        Self { registry }
    }

    pub fn get_employees(&self, department: Department) -> Vec<&Employee> {
        let compare_employees = |emp1: &&Employee, emp2: &&Employee| {
            (*emp1.name)
                .to_lowercase()
                .cmp(&(*emp2.name).to_lowercase())
        };

        match department {
            Department::None => {
                let mut employees: Vec<&Employee> =
                    self.registry.iter().map(|(emp, _)| emp).collect();
                employees.sort_by(compare_employees);
                employees
            }
            _ => {
                let mut employees: Vec<&Employee> = self
                    .registry
                    .iter()
                    .filter(|(_, dep)| **dep == department)
                    .map(|(emp, _)| emp)
                    .collect();
                employees.sort_by(compare_employees);
                employees
            }
        }
    }

    pub fn add_employee(&mut self, employee: &Employee, department: &Department) {
        self.registry.insert(employee.clone(), *department);
    }

    pub fn add_employees(&mut self, to_add: &[(Employee, Department)]) {
        for (emp, dep) in to_add {
            self.add_employee(emp, dep);
        }
    }
}
