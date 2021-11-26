fn mean(vec: &Vec<i32>) -> f64 {
    let mut sum = 0;
    for e in vec {
        sum += e;
    }
    sum as f64 / vec.len() as f64
}
fn median(vec: &mut Vec<i32>) -> i32 {
    vec.sort();
    vec[vec.len()/2]
}
fn mode(vec: &Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let mut map: HashMap<i32, i32> = HashMap::new();

    let mut max: (i32,i32) = (0, 0);
    for i in vec {
        let val = map.entry(*i).or_insert(0);
        *val += 1;
        if max.0 < *val {
            max.0 = *val;
            max.1 = *i;
        }
    }
    max.1
}

fn pig_latin(str: &str) -> String {
    let mut ans: String;

    if is_vowel(&str[0..1]) {
        ans = str.to_string();
        ans.push_str("-hay");
    } else {
        ans = str[1..].to_string();
        ans.push('-');
        ans.push_str(&str[0..1]);
        ans.push_str("ay");
    }
    fn is_vowel(str: &str) -> bool {
        if str == "a" ||
            str == "i" ||
            str == "u" ||
            str == "e" ||
            str == "o" {
            return true;
        } else {
            return false;
        }
    }
    ans
}
use std::collections::HashMap;

struct company {
    employee: HashMap<String, Vec<String>>
}
impl company {
    fn add(&mut self, name: &str, department: &str) {
        let val = self.employee.entry(department.to_string()).or_insert(Vec::new());
        val.push(name.to_string());
    }
    fn list_department(&mut self, department: &str) -> Vec<String> {
        self.employee
            .entry(department.to_string())
            .or_insert(Vec::new())
            .to_vec()
    }
    fn print_department(&mut self, department: &str) {
        for i in self.list_department(department) {
            println!("{}", i);
        }
    }
    fn print_all_member(&mut self) {
        for (key, value) in &mut self.employee {
            value.sort();
            for e in value {
                println!("{}", e);
            }
        }
    }
    fn syori(&mut self) {
        let mut data = String::new();
        std::io::stdin().read_line(&mut data).expect("neeyo");
        let vec: Vec<&str> = data.split_whitespace().collect();

        if vec[0] == "Add" {
            self.add(vec[1], vec[3]);
        }
        else if vec[0] == "All" {
            self.print_all_member();
        }
        else {
            self.print_department(vec[0]);
        }
    }
}

fn main() {
    println!("{}", mean(&vec![1,2,3,4]));
    println!("{}", median(&mut vec![4,3,2,1]));
    println!("{}", mode(&vec![1,1,2]));
    println!("{} {}", pig_latin("good"), pig_latin("apple"));

    let mut employee = company {
        employee: HashMap::new(),
    };
    while true {
        employee.syori();
    }
}
