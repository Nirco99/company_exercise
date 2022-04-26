use std::{collections::HashMap, io};
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Employee {
    first_name: String,
    last_name: String,
    employee_department: String,
    age: u32,
}
impl std::fmt::Display for Employee {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            fmt,
            "Employee name is {} {}, {} years old,in the {} department",
            self.first_name, self.last_name, self.age, self.employee_department
        )
    }
}

pub fn add_employee_prompt(department_list: &mut HashMap<String, Vec<Employee>>) {
    println!("Please insert first name");
    let mut first_name = String::new();
    io::stdin()
        .read_line(&mut first_name)
        .expect("Failed to first name");
    let first_name = first_name.trim().to_string();

    println!("Please insert last name");
    let mut last_name = String::new();
    io::stdin()
        .read_line(&mut last_name)
        .expect("Failed to read last name");
    let last_name = last_name.trim().to_string();

    println!("Please insert employee department");
    let mut employee_department = String::new();
    io::stdin()
        .read_line(&mut employee_department)
        .expect("Failed to read last name");
    let employee_department = employee_department.trim().to_string();

    println!("Please enter employee age");
    let mut age_string = String::new();
    io::stdin()
        .read_line(&mut age_string)
        .expect("Failed to read age");
    let age = age_string.trim().parse::<u32>();
    match age {
        Ok(..) => (),
        Err(..) => println!("Please insert valid age"),
    }
    match department_list.contains_key(&employee_department) {
        true => {
            department_list
                .get_mut(&employee_department)
                .unwrap()
                .push(add_employee(
                    first_name,
                    last_name,
                    employee_department.clone(),
                    age.unwrap(),
                ));

            department_list
                .get_mut(&employee_department)
                .unwrap()
                .sort();
        }
        false => {
            let mut employee_vec: Vec<Employee> = Vec::new();
            employee_vec.push(add_employee(
                first_name,
                last_name,
                employee_department.clone(),
                age.unwrap(),
            ));
            employee_vec.sort();
            department_list.insert(employee_department.clone(), employee_vec);
        }
    }
}

fn add_employee(
    first_name: String,
    last_name: String,
    employee_department: String,
    age: u32,
) -> Employee {
    Employee {
        first_name,
        last_name,
        employee_department,
        age,
    }
}

pub fn add_department_prompt(department_list: &mut HashMap<String, Vec<Employee>>) {
    let deparment_employees: Vec<Employee> = Vec::new();
    println!("Please insert department name");
    let mut department_name = String::new();
    io::stdin()
        .read_line(&mut department_name)
        .expect("Failed to department name");
    let department_name = department_name.trim().to_string();

    add_department(department_list, department_name, deparment_employees);
}

fn add_department(
    department_list: &mut HashMap<String, Vec<Employee>>,
    department_name: String,
    department_employees: Vec<Employee>,
) {
    department_list.insert(department_name, department_employees);
}

pub fn list_in_department(department_list: &mut HashMap<String, Vec<Employee>>) {
    println!("employees of which department would you like to least?");
    let mut department_name = String::new();
    io::stdin()
        .read_line(&mut department_name)
        .expect("Failed to read department name");
    department_name = department_name.trim_end().to_string();

    if department_list.contains_key(&department_name) == true {
        let mut count = 0;
        for i in department_list.iter() {
            if i.0 == &department_name {
                for t in department_list.get(&department_name).unwrap() {
                    println!("{}", t);
                }
                count = count + 1;
            }
        }
    }
}

pub fn list_all_departments(department_list: &mut HashMap<String, Vec<Employee>>) {
    for i in department_list.keys() {
        println!("Department name is: {}.", i.to_string())
    }
}

pub fn list_by_department(department_list: &mut HashMap<String, Vec<Employee>>) {
    for department in department_list {
        for i in 0..department.1.len() {
            print!("{}\n", department.1[i]);
        }
    }
}
