#[derive(Debug)]
struct Student {
    name: String,
    age: u8,
    grade: char,
}

fn display_students(students: &Vec<Student>) {
    for student in students {
        println!("Name: {}, Age: {}, Grade: {}", student.name, student.age, student.grade);
    }
}


fn update_grade(students: &mut Vec<Student>, name: &str, new_grade: char) {
    for student in students.iter_mut() {
        if student.name == name {
            student.grade = new_grade;
            println!("Updated grade for {}: {}", name, new_grade);
        }
    }
}

fn main_student_records() {
    let mut students = vec![
        Student { name: "John".to_string(), age: 20, grade: 'B' },
        Student { name: "Emma".to_string(), age: 22, grade: 'A' },
    ];
    display_students(&students);
    update_grade(&mut students, "John", 'A');
    display_students(&students);
}

fn main() {
    main_student_records();
}

