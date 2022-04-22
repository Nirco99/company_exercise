use std::io;

struct Employee {
    first_name:String,
    last_name:String,
    age:u32,
    id:u64,
}
fn main() {
    println!("Please chose one of the following options");
    println!("1.Add an employee to department");
    println!("2.Add a department");
    println!("3.Retrieve all employees in a department");
    println!("4.Retrieve all employess by a department");

    let mut user_string = String::new();
    io::stdin()
        .read_line(&mut user_string)
        .expect("Failed to read line");
    
    let user_option = user_string.trim();
    
    match user_option.parse::<u32>() {
        Ok(i) =>{
            if i == 1 {
                add_employee_prompt();
            }
            else if i == 2 {
                // Add_department();
            }
            else if i == 3  {
                // List_in_department();
            }
            else if i == 4 {
                // List_by_department();
            }
            else {
                println!("Number {} is not a valid option", &i);
            }
            
        },
        Err(..) => println!("Please enter valid input")
    }
}
fn add_employee_prompt() -> Employee {
                    println!("Please insert first name");
                let mut first_name = String::new();
                io::stdin()
                    .read_line(&mut first_name)
                    .expect("Failed to first name");
                
                println!("Please insert last name");
                let mut last_name = String::new();
                io::stdin()
                    .read_line(&mut last_name)
                    .expect("Failed to read last name");

                println!("Please enter employee age");
                let mut age_string = String::new();
                io::stdin()
                    .read_line(&mut age_string)
                    .expect("Failed to read age");
                let age = age_string.trim().parse::<u32>();
                match age{
                    Ok(..) => (),
                    Err(..) => println!("Please insert valid age")
                }

                println!("Please enter employee id");
                let mut id_string = String::new();
                io::stdin()
                    .read_line(&mut id_string)
                    .expect("Failed to read age");
                let id = id_string.trim().parse::<u64>();
                match id {
                    Ok(..) => (),
                    Err(..) => println!("Please insert valid id")
                }
                add_employee(first_name, last_name, age.unwrap(), id.unwrap())

}
fn add_employee(first_name:String,last_name:String,age:u32,id:u64) -> Employee {

    Employee {
         first_name,
         last_name,
         age,
         id 
        }
}
