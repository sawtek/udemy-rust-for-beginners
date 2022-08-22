// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees
// * Use a struct to store the employee type and whether they are
//   still employed
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this

enum EmployeeRole {
    Maintenance,
    Marketing,
    Manager,
    LineSupervisor,
    Kitchen,
    AssemblyTech,
}

struct Employee {
    role: EmployeeRole,
    terminated: bool,
}

impl Employee {
    fn access(&self) -> Result<(), String> {
        if !self.terminated {
            match self.role {
                EmployeeRole::Maintenance => Ok(()),
                EmployeeRole::Marketing => Ok(()),
                EmployeeRole::Manager => Ok(()),
                _ => Err("role has no permission".to_owned()),
            }
        } else {
            Err("terminated".to_owned())
        }
    }
}

fn print_access(employee: &Employee) -> Result<(), String> {
    employee.access()?;
    println!("granted");
    Ok(())
}

fn main() {
    let emp1 = Employee {
        role: EmployeeRole::Maintenance,
        terminated: false,
    };

    let emp2 = Employee {
        role: EmployeeRole::Maintenance,
        terminated: true,
    };

    let emp3 = Employee {
        role: EmployeeRole::AssemblyTech,
        terminated: false,
    };

    match print_access(&emp1) {
        Err(msg) => println!("denied, {:?}", msg),
        _ => (),
    }
    match print_access(&emp2) {
        Err(msg) => println!("denied, {:?}", msg),
        _ => (),
    }
    match print_access(&emp3) {
        Err(msg) => println!("denied, {:?}", msg),
        _ => (),
    }
}
