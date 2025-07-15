#[derive(Debug)]

struct Student {
    name: String,
    marks: Vec<u32>,
}

fn main() {
    let students = vec![
        Student {
            name : "Ayesha".to_string(),
            marks : vec![85, 90, 78],
        },

        Student {
            name : "Rafi ".to_string(),
            marks : vec![88 , 92, 80],
        },
];

for Student in &students {
    let total: u32 = students.marks.iter().sum();
    println!("{} scored total {}", students.name , total);
}
}