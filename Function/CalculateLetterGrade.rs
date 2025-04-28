fn CalculateGrade (score: u32 ) -> &'static str {
    if score >= 90 {
        return  "A+";
    } else if score >= 80{
        return  "A";
    } else if score >= 70 {
        return  "B";
    } else if score >= 60 {
       return "C";
    } else {
        return "F";
    }
}

fn main() {
    println!("Grade : {}", CalculateGrade(50));
}
