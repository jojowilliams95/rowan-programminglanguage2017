struct Employee {
        name: char,
        phone: i32,
        network: i32,
        computer: i32,
}

struct EmployeeList{ [Employee; 26]: array,}

impl Copy for EmployeeList{}

fn flatten(assign: [[char; 4]; 4], mut squish: [char; 16]) -> [char; 16] {
	let mut i: usize = 1;
		for row in 1..4 {
			for col in 1..4 {
				squish[i] = assign[row][col];
				i = i+1;
			}
		}
	squish
}

fn duplicates(squish: [char; 16]) -> bool {
	let mut i: usize = 15;
	for employee in 1..15 {
		for index in 1..i {
			if squish[employee] == squish[employee + index] {
				return true;
			}
		}
		i = i-1;
	}
	false
}

fn get_employee(num_employees: usize, x: char, sk: EmployeeList) -> Employee {
	let dummy: char = ' ';
	for empl in 1..num_employees {
		if x == sk[empl].name {
			return sk[empl];
		}
	}
	Employee{name: dummy, phone: 0, network: 0, computer: 0}
}

fn can_work(num_employees:usize, sk: [Employee; 26], worker: [[char; 4]; 4]) -> bool{
	for row in 1..4{
		for col in 1..2{
			if get_employee(num_employees, worker[row][col], sk).phone == 0{
				return false;
			}
		}
	} 
	for row in 1..4{
		if get_employee(num_employees, worker[row][3], sk).network == 0{
			return false;
	 	}
	}
	for row in 1..4{
		if get_employee(num_employees, worker[row][4], sk).computer == 0{
	      		return false;
      		}
	}
	true
}

fn main(){
}
