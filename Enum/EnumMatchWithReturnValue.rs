enum OperationSystem{
    Windows,
    MacOs,
    Linux,
}

fn main(){
    let MyComputer = OperationSystem::Linux;
    let age = YearsSinceRelease(MyComputer);
    println!("My Computer Operatiing system is {age} years old ");

    let Dads_Computer = OperationSystem::Windows;
    let age = Years_since_release(Dads_Computer);
    println!("MyDads computer");
}

fn Years_since_release (os: OperationSystem) -> u32 {
    match  os  {
        OperationSystem::Windows => {
            println!("Quite an old Operating system ");
            39
        }

        OperationSystem::Linux=>23,
        OperationSystem::Linux => 34,
    }
}
