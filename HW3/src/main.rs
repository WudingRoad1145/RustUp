// A basic CRUD program for school
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Student {
    id: i32,
    name: String,
    class: i32,
    grade: i32,
    courses: Vec<i32>,
}

#[derive(Clone, Debug)]
pub struct Class {
    id: i32,
    students: Vec<Student>,
}

#[derive(Clone, Debug)]
pub struct Grade {
    id: i32,
    students: Vec<Student>,
}

#[derive(Clone, Debug)]
pub struct Course {
    id: i32,
    name: String,
    students: Vec<Student>,
}

#[derive(Debug)]
pub struct StudentManagementSystem {
    students: HashMap<u32, Student>,
    classes: HashMap<u32, Class>,
    courses: HashMap<u32, Course>,
    grades: HashMap<u32, Grade>,
}

impl StudentManagementSystem {
    pub fn new() -> Self {
        Self{
            students: HashMap::new(),
            classes: HashMap::new(),
            courses: HashMap::new(),
            grades: HashMap::new(),
        }
    }
    // add
    pub fn add_student(&mut self, student: Student) {
        self.students.insert(student.id as u32, student);
    }
    pub fn add_class(&mut self, class: Class) {
        self.classes.insert(class.id as u32, class);
    }
    pub fn add_course(&mut self, course: Course) {
        self.courses.insert(course.id as u32, course);
    }
    pub fn add_grade(&mut self, grade: Grade) {
        self.grades.insert(grade.id as u32, grade);
    }
    // delete
    pub fn delete_student(&mut self, id: u32) {
        self.students.remove(&id);
    }
    pub fn delete_class(&mut self, id: u32) {
        self.classes.remove(&id);
    }
    pub fn delete_course(&mut self, id: u32) {
        self.courses.remove(&id);
    }
    pub fn delete_grade(&mut self, id: u32) {
        self.grades.remove(&id);
    }
    // update
    pub fn update_student(&mut self, id: u32, student: Student) {
        self.students.insert(id, student);
    }
    pub fn update_class(&mut self, id: u32, class: Class) {
        self.classes.insert(id, class);
    }
    pub fn update_course(&mut self, id: u32, course: Course) {
        self.courses.insert(id, course);
    }
    pub fn update_grade(&mut self, id: u32, grade: Grade) {
        self.grades.insert(id, grade);
    }
    // get
    pub fn get_student(&self, id: u32) -> Option<&Student> {
        self.students.get(&id)
    }
    pub fn get_class(&self, id: u32) -> Option<&Class> {
        self.classes.get(&id)
    }
    pub fn get_course(&self, id: u32) -> Option<&Course> {
        self.courses.get(&id)
    }
    pub fn get_grade(&self, id: u32) -> Option<&Grade> {
        self.grades.get(&id)
    }
}


fn main() {
    let mut school = StudentManagementSystem::new();

    school.add_student(Student{
        id: 1,
        name: "John".to_string(),
        class: 1,
        grade: 9,
        courses: vec![101, 102],
    });
    school.add_student(Student{
        id: 2,
        name: "Jane".to_string(),
        class: 1,
        grade: 10,
        courses: vec![101, 102],
    });

    school.add_class(Class{
        id: 1,
        students: vec![school.get_student(1).unwrap().clone(), school.get_student(2).unwrap().clone()],
    });

    school.add_grade(Grade{
        id: 10,
        students: vec![school.get_student(2).unwrap().clone()],
    });

    // Print all students
    println!("Students:");
    for (_, student) in school.students.iter() {
        println!("{:?}", student);
    }

    // Print class 1
    println!("Class 1:");
    for student in school.get_class(1).unwrap().students.iter() {
        println!("{:?}", student);
    }

    // Print grade 10 - got panick cuz grade isn't added before
    println!("Grade 10:");
    for student in school.get_grade(10).unwrap().students.iter() {
        println!("{:?}", student);
    }

    // Better handle implicitly
    if let Some(grade) = school.get_grade(10) {
        for student in grade.students.iter() {
            println!("{:?}", student);
        }
    } else {
        println!("No grade with id 10");
    }
}