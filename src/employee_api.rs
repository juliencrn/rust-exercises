// Using a hash map and vectors, create a text interface
// to allow a user to add employee names to a department in a company.
//
// For example, “Add Sally to Engineering” or “Add Amir to Sales.”
// Then let the user retrieve a list of all people in a department
// or all people in the company by department, sorted alphabetically.
//
// Don’t develop real API, instead use TDD
// Don’t use database, an in-memory hashmap is enough for this exercise.
use regex::Regex;
use std::collections::HashMap;
use std::fmt;

pub struct Company {
    departments: HashMap<String, Vec<String>>,
}

impl fmt::Display for Company {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut content = String::new();
        for (department, employees) in &self.departments {
            content += &format!("Department: {}\n", department);
            for employee in employees {
                content += &format!(" - {}\n", employee);
            }
        }
        write!(f, "Company: \n--------\n{}", content)
    }
}

impl Company {
    pub fn new() -> Company {
        Company {
            departments: HashMap::new(),
        }
    }

    pub fn add_employee_to(&mut self, input: &str) {
        match Company::parse_input(input) {
            Some((employee, department)) => {
                self.set_employee(employee, department);
            }
            None => panic!("Unable to parse input. Format:\nAdd employee-name to department-name"),
        };
    }

    fn parse_input(input: &str) -> Option<(&str, &str)> {
        let re = Regex::new(r"[Aa]dd (?P<employee>[\w|\-]+).+to (?P<department>[\w|\-]+)").unwrap();
        let caps = re.captures(input).unwrap();
        let employee = caps.name("employee").map_or("", |m| m.as_str());
        let department = caps.name("department").map_or("", |m| m.as_str());
        if employee == "" || department == "" {
            return None;
        }
        return Some((employee, department));
    }

    pub fn get_all_employees(&self) -> Vec<String> {
        let mut all_employees: Vec<String> = vec![];

        for (_, employees) in &self.departments {
            for employee in employees {
                all_employees.push(String::from(employee));
            }
        }

        all_employees
    }

    pub fn get_employees_by_department(&self, department: &str) -> Option<&Vec<String>> {
        self.departments.get(department)
    }

    fn set_employee(&mut self, employee: &str, department: &str) {
        self.departments
            .entry(String::from(department))
            .and_modify(|vec| vec.push(String::from(employee)))
            .or_insert(vec![String::from(employee)]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_employee() {
        let mut company = Company::new();
        assert_eq!(company.get_all_employees().len(), 0);
        company.set_employee("Julien", "Tech");
        assert_eq!(company.get_all_employees().len(), 1);
        company.set_employee("Steven", "Tech");
        company.set_employee("Alex", "Marketing");
        assert_eq!(company.get_all_employees().len(), 3);
    }

    #[test]
    fn test_set_text_employee() {
        let mut company = Company::new();

        company.add_employee_to("Add Steven to Data");
        company.add_employee_to("Add Alex to Sales.");

        assert_eq!(company.get_all_employees().len(), 2);
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(
            Company::parse_input("Add Julien to Tech"),
            Some(("Julien", "Tech"))
        );
        assert_eq!(
            Company::parse_input("Please, add Juliette to Design."),
            Some(("Juliette", "Design"))
        );
        assert_eq!(
            Company::parse_input("add Jean-paul to Sales, please"),
            Some(("Jean-paul", "Sales"))
        );
    }

    #[test]
    fn test_get_employees_by_department() {
        let mut company = Company::new();

        company.add_employee_to("Add Julien to Tech");
        company.add_employee_to("Add Mateo to Tech");
        company.add_employee_to("Add Juliette to Design");

        let existing = company.get_employees_by_department("Tech");
        let nonexisting = company.get_employees_by_department("Luxe");

        assert_eq!(
            existing,
            Some(&vec![String::from("Julien"), String::from("Mateo")])
        );
        assert_eq!(nonexisting, None)
    }
}
