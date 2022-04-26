use std::{collections::HashMap, io};
mod functions;

fn main() {
    let mut department_list: HashMap<String, Vec<functions::Employee>> = HashMap::new();
    loop {
        println!("Please choose one of the following options");
        println!("1.Add an employee to department");
        println!("2.Add a department");
        println!("3.Retrieve all employees in a department");
        println!("4.Retrieve all employess by a department");
        println!("5.Retrieve all departments");
        println!("6.Exit from interface");
        let mut user_string = String::new();
        io::stdin()
            .read_line(&mut user_string)
            .expect("Failed to read line");

        let user_option = user_string.trim();

        match user_option.parse::<u32>() {
            Ok(i) => {
                if i == 1 {
                    functions::add_employee_prompt(&mut department_list);
                } else if i == 2 {
                    functions::add_department_prompt(&mut department_list);
                } else if i == 3 {
                    functions::list_in_department(&mut department_list);
                } else if i == 4 {
                    functions::list_by_department(&mut department_list);
                } else if i == 5 {
                    functions::list_all_departments(&mut department_list);
                } else if i == 6 {
                    break;
                } else {
                    println!("Number {} is not a valid option", &i);
                }
            }
            Err(..) => println!("Please enter valid input"),
        }
        println!("\n\n")
    }
}
