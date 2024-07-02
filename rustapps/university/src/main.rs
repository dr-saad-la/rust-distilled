mod models;

// This is the absolute path
use crate::models::professor::Professor;
use crate::models::student::Student;

fn main() {
    // Instantiate a professor object
    let prof_1: Professor = Professor {
        name: String::from("Dr. John Macceny"),
        degree: String::from("Professor"),
        years_of_exeperience: 12,
    };

    println!("{:?}", prof_1);
    println!(
        "{} : {} : {}",
        prof_1.name, prof_1.degree, prof_1.years_of_exeperience
    );

    let student: Student = Student {
        student_id: String::from("#02020"),
        name: String::from("David Adams"),
        student_email: String::from("david.adams@example.com"),
        student_level: 3,
    };

    println!("{:?}", student);
}

// #[derive(Debug)]
// struct Professor {
//     name: String,
//     degree: String,
//     years_of_exeperience: u32,
// }
