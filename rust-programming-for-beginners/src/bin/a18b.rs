// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employee that can access the building are:
//   * Maintenance crews
//   * Marketing department Employee
//   * Manager
// * Other Employee that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated Employee cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of Employee
// * Use a struct to store the employee type and whether they are
//   still employed
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this

#[derive(Debug)]
enum Position {
    MaintenanceCrew,
    Marketing,
    Manager,
    LineSupervisor,
    KitchenStaff,
    AssemblyTechnician,
}

enum Status {
    Active,
    Terminated,
}

struct Employee {
    position: Position,
    status: Status,
}

fn check_access(employee: &Employee) -> Result<(), String> {
    match employee.status {
        Status::Terminated => return Err("The employee is terminated".to_owned()),
        _ => (),
    }

    match employee.position {
        Position::MaintenanceCrew => Ok(()),
        Position::Marketing => Ok(()),
        Position::Manager => Ok(()),
        _ => Err("The access is not available for this position".to_owned()),
    }
}

fn print_access(employee: &Employee) -> Result<(), String> {
    let attempt_access = check_access(employee)?;
    println!("access is ok for {:?}", employee.position);
    Ok(())
}

fn main() {
    let employee1 = Employee {
        position: Position::MaintenanceCrew,
        status: Status::Active,
    };

    let employee2 = Employee {
        position: Position::KitchenStaff,
        status: Status::Active,
    };

    let employee3 = Employee {
        position: Position::Manager,
        status: Status::Terminated,
    };

    match print_access(&employee3) {
        Err(e) => println!("access denied: {:?}", e),
        _ => (),
    }
}
