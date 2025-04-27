fn main() {
    let employee=("Molly" , 32 , "Marketing");

    let name = employee.0;
    let age = employee.1;
    let department = employee.2;
    println!("Name: {name} , age: {age} , department: {department}");





    println!("{employee:#?}")
}
