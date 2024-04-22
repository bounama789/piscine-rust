pub struct Student (pub u32,pub String,pub String,);
   

pub fn id(student: &Student) -> u32 {
    student.0
}

pub fn first_name(student: &Student) -> String {
    student.1.to_string()
}

pub fn last_name(student: &Student) -> String {
    student.2.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id() {
        let student = Student(20, "Pedro".to_string(), "Domingos".to_string());
        let result = id(&student);
        assert_eq!(result, 20);
    }

    #[test]
    fn test_firstname() {
        let student = Student(20, "Pedro".to_string(), "Domingos".to_string());
        let result = first_name(&student);
        assert_eq!(result, "Pedro");
    }

    #[test]
    fn test_lastname() {
        let student = Student(20, "Pedro".to_string(), "Domingos".to_string());
        let result = last_name(&student);
        assert_eq!(result, "Domingos");
    }
}
