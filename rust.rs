use std::collections::HashSet;

struct Employee(char, i32, i32, i32);

fn flatten(a: &Vec<char>, b: &Vec<char>, c: &Vec<char>, d: &Vec<char>) -> Vec<char> {
    let mut abcd: Vec<char> = [a.as_slice(), b.as_slice(), c.as_slice(), d.as_slice()].concat();
    abcd.shrink_to_fit();
    abcd
}

fn duplicates(sq: &Vec<char>) -> bool {
    let index = 16;
    let set: HashSet<_> = sq.into_iter().collect();
    if set.len() < index {
        return false;
        } else {
        true
    }
}

fn can_work(assign:&Vec<Vec<char>>, worker:&Vec<Employee>) -> bool{
    for i in assign.iter(){
        for empl in worker.iter() {
            if i[0] == empl.0 || i[1] == empl.0{
                if empl.1 == 0{
                    return false;
                }
            }
            if i[2] == empl.0 {
                if empl.2 == 0{
                    return false;  
                }
            }
            if i[2] == empl.0 {
                if empl.2 == 0{
                    return false;  
                }
            }
        }
    }
    true
}

fn main(){
    let assignments = vec![vec!['a', 'b', 'c', 'd'],
                        vec!['e', 'f', 'g', 'h'],
                        vec!['i', 'j', 'k', 'l'],
                        vec!['m', 'n', 'o', 'p']];
    let employees:Vec<Employee> = vec![Employee('a', 1, 1, 1),
                                        Employee('b', 1, 1, 1),
                                        Employee('c', 1, 1, 1),
                                        Employee('d', 1, 1, 1),
                                        Employee('e', 1, 1, 1),
                                        Employee('f', 1, 1, 1),
                                        Employee('g', 1, 1, 1),
                                        Employee('h', 1, 1, 1),
                                        Employee('i', 1, 1, 1),
                                        Employee('j', 1, 1, 1),
                                        Employee('k', 1, 1, 1),
                                        Employee('l', 1, 1, 1),
                                        Employee('m', 1, 1, 1),
                                        Employee('n', 1, 1, 1),
                                        Employee('o', 1, 1, 1),
                                        Employee('p', 1, 1, 1),
                                        Employee('q', 1, 1, 1)];
    let combine = flatten(&assignments[0], &assignments[1], &assignments[2], &assignments[3]);
    assert!(duplicates(&combine), "Not acceptable, you have duplicates");
    assert!(can_work(&assignments, &employees), "Not acceptable, at least one employee does not have the proper skill.");
    println!("Acceptable");
    
}
