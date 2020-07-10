use std::iter::FromIterator;

fn main() {
    let hs_class_students: Vec<&HighSchoolStudent> = Vec::new();
    let hs_class_students1: Vec<&HighSchoolStudent> = Vec::new();
    let college_class_students: Vec<&CollegeStudent> = Vec::new();
    
    let mut hs_class: Class<HighSchoolStudent> = Class::new(
        "biology".to_string(), 
        "charles darwin".to_string(), hs_class_students,
    );
    let mut college_class: Class<CollegeStudent> = Class::new(
        "computing sciences".to_string(),
        "dijkstra".to_string(), college_class_students,
    );
    let mut hs_class1: Class<HighSchoolStudent> = Class::new(
        "maths".to_string(), 
        "whatever".to_string(), hs_class_students1,
    );

    let mut student = HighSchoolStudent::new(3.6, "oscar".to_string(), "hs 1".to_string());
    hs_class.enroll_student(&mut student);

    let mut student1 = HighSchoolStudent::new(4.5, "oscar1".to_string(), "hs 1".to_string());
    hs_class.enroll_student(&mut student1);
    
    let mut student2 = HighSchoolStudent::new(3.55, "ivan".to_string(), "hs 2".to_string());
    hs_class.enroll_student(&mut student2);
    
    let mut student3 = CollegeStudent::new(3.6, "gutierrez".to_string(), "systems eng".to_string());
    college_class.enroll_student(&mut student3);

    let mut student4 = CollegeStudent::new(3.55, "rincon".to_string(), "architecture".to_string());
    college_class.enroll_student(&mut student4);

    let mut student5 = CollegeStudent::new(5.0, "rincon1".to_string(), "architecture".to_string());
    college_class.enroll_student(&mut student5);

    let mut student6 = HighSchoolStudent::new(3.6, "oscar".to_string(), "hs 1".to_string());
    hs_class1.enroll_student(&mut student6);

    let mut student7 = HighSchoolStudent::new(4.5, "oscar1".to_string(), "hs 1".to_string());
    hs_class1.enroll_student(&mut student7);

    assert!(!college_class.all_on_same());
    assert!(!hs_class.all_on_same());
    assert!(hs_class1.all_on_same());

    println!("{:#?}", college_class.rank_students());
    println!("{:#?}", hs_class.rank_students());

    println!("");
    println!("");
    println!("");
    println!("");
    println!("");

    println!("{:#?}", college_class);
    println!("{:#?}", hs_class);

    assert!(student4 == student5);
    assert!(!(student3 == student5));
}

#[derive(Debug)]
pub struct Student {
    name: String,
    grade: f32
}

impl Student {
    pub fn new(grade: f32, name: String) -> Self {
        Student {
            grade,
            name
        }
    }

    pub fn grade(&self) -> f32 {
        self.grade
    }
}


#[derive(Debug)]
pub struct CollegeStudent {
    student: Student,
    program: String
}

impl CollegeStudent {
    pub fn new(grade: f32, name: String, program: String) -> CollegeStudent {
        CollegeStudent {
            student: Student::new(grade, name),
            program
        }
    }
}

impl PartialEq for CollegeStudent {
    fn eq(&self, other: &Self) -> bool {
        self.program == other.program
    } 
}

impl Eq for CollegeStudent{}

impl PartialOrd for CollegeStudent {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.student.grade().partial_cmp(&other.student.grade())
    }
}

impl Ord for CollegeStudent {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.student.grade().partial_cmp(&other.student.grade()) {
            Some(v) => v,
            None => panic!("bad element")
        }
    }
}

#[derive(Debug)]
pub struct HighSchoolStudent {
    student: Student,
    high_school: String
}

impl HighSchoolStudent {
    pub fn new(grade: f32, name: String, high_school: String) -> Self {
        HighSchoolStudent {
            student: Student::new(grade, name),
            high_school
        }
    }
}

impl PartialEq for HighSchoolStudent {
    fn eq(&self, other: &Self) -> bool {
        self.high_school == other.high_school
    } 
}

impl Eq for HighSchoolStudent{}

impl PartialOrd for HighSchoolStudent {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.student.grade().partial_cmp(&other.student.grade())
    }
}

impl Ord for HighSchoolStudent {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.student.grade().partial_cmp(&other.student.grade()) {
            Some(v) => v,
            None => panic!("bad element")
        }
    }
    
}


#[derive(Debug)]
pub struct Class<'a, T> {
    name: String,
    professor: String,
    students: Vec<&'a T>
}

impl<'a, T> Class<'a, T> where T: PartialEq + Eq + PartialOrd + Ord {
    pub fn new(name: String, professor: String, students: Vec<&'a T>) -> Self {
        Class {
            name,
            professor,
            students
        }
    }
    pub fn enroll_student(&mut self, student: &'a mut T) {
        self.students.push(student);
    }
    
    pub fn all_on_same(&self) -> bool {
        self.students.iter().filter(|&x| &self.students[0] != x).count() == 0
    }

    pub fn rank_students(&mut self) -> Vec<&T> {
        // let mut var = self.students.clone();
        // let mut var: Vec<_> = self.students.iter().map(|&x| x).collect();
        let mut var = Vec::from_iter(self.students.iter().cloned());
        var.sort_by(|a, b| b.cmp(a));
        var
    }
}
