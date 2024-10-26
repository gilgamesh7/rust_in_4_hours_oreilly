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
    fn print(self: &Worker) {
        println!("{0} earns {1} and it is {2} that he is a fulltime employee", self.name, self.salary, self.fulltime);
    }

    fn payrise(self: &mut Worker, amount: f64) {
        self.salary += amount;
    }

    fn new(name: String, salary: f64, fulltime: bool) -> Worker {
        Worker{
            name,
            salary,
            fulltime
        }
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

    // Use Worker struct
    let mut emp4 = Worker::new(String::from("Joe"), 4000.0, false);
    emp4.print();
    emp4.payrise(500.0);
    emp4.print();
    
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