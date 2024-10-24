pub struct Employee{
    pub name: String,
    pub salary: f64,
    pub fulltime: bool
}

struct Worker{
    name: String,
    salary: f64,
    fulltime: bool
}

impl Worker {
    fn print(&self) {
        
    }
}

fn main() {
    let emp1 = Employee{
        name: String::from("John"),
        salary: 1000.0,
        fulltime: true
    };

    println!("{0} {2} {1}", emp1.name, emp1.salary, emp1.fulltime);

    let mut emp2 = Employee{
        name: String::from("Jane"),
        salary: 2000.0,
        fulltime: false
    };
    println!("{0} {2} {1}", emp2.name, emp2.salary, emp2.fulltime);
    emp2.name = String::from("Jill");
    println!("{0} {2} {1}", emp2.name, emp2.salary, emp2.fulltime);

    let emp3 = build_emp(String::from("Jim"), 3000.0, true);
    print_emp(&emp3);
    println!("{0} {2} {1}", emp3.name, emp3.salary, emp3.fulltime);
    
}

fn build_emp(name: String, salary: f64, fulltime: bool) -> Employee {
    return Employee{
        name: name,
        salary: salary,
        fulltime: fulltime
    }
}

fn print_emp(emp: &Employee) {
    println!("{0} {2} {1}", (*emp).name, (*emp).salary, (*emp).fulltime);
}