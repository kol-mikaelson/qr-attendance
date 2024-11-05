use chrono::{DateTime, Local};
#[warn(dead_code)]
struct Rollno{
    year:i32,
    branch:String,
    number:i32,
}
struct Student{
    name:String,
    roll:Rollno,
}

pub struct Sdata{
    student: Student,
    time:DateTime<Local>
}