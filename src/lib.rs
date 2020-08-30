use std::collections::HashMap;
use std::fmt::{self, };

type DbResult<T> = std::result::Result<T, RhDbError>;

#[derive(Debug)]
pub struct RhDatabase {
	data: HashMap<String, Vec<String>>,
}

#[derive(Debug)]
pub enum RhDbError {
	InvalidDeptName,
	EmptyName,
	MissingDept,
	NotImplemented,
	DeptAlreadyExists,
}


impl fmt::Display for RhDbError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Error {:?} error", self)
	}
}

impl RhDatabase {
	pub fn new() -> RhDatabase {
		RhDatabase { data: HashMap::new() }
	}
	
	pub fn add_dept(&mut self, dept: String) -> DbResult<String> {
		if self.data.contains_key(&dept) {
			return Err(RhDbError::DeptAlreadyExists);
		}
		if dept == "".to_string() || dept.starts_with('*') {
			return Err(RhDbError::InvalidDeptName);
		}

		let employees : Vec<String> = Vec::new();
		self.data.insert(dept.clone(), employees);
		Ok(dept)
	}


	pub fn add_employee(&mut self, employee : String, dept: String) -> DbResult<(String, String)> {
		if employee == "".to_string() {
			return Err(RhDbError::EmptyName);
		}
		if dept == "".to_string() || dept == "*".to_string(){
			return Err(RhDbError::InvalidDeptName);
		}
		/* if !self.data.contains_key(dept.as_str()) {
			return RhDbError::MissingDept;
		} */
		match self.data.get_mut(&dept){
			Some(employee_list) => {
				employee_list.push(employee.clone());
				println!("Success.");
				Ok((employee, dept))
			},
			None => {
				Err(RhDbError::MissingDept)
			}
		}
		
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_creating_db(){
		//entering new version
	    let a = RhDatabase::new();
	    assert!(a.data.is_empty());
	}

	#[test]
	fn test_adding_dept() {
		let mut a = RhDatabase::new();
		match a.add_dept("New Dept".to_string()) {
			Ok(name) => assert_eq!(name, "New Dept".to_string()),
			Err(_) => assert!(false),
		}
		assert!(a.data.contains_key("New Dept"));
		assert!(a.data.get(&"New Dept".to_string()).unwrap().is_empty());
	}

	#[test]
	fn test_adding_duplicate_dept() {
		let mut a = RhDatabase::new();
		a.add_dept("Duplicate".to_string());
		match a.add_dept("Duplicate".to_string()) {
			Ok(_) => assert!(false, "Ok"),
			Err(RhDbError::DeptAlreadyExists) => assert!(true),
			Err(_) => assert!(false, "Err(_)"),
		}
	}

	#[test]
	fn test_adding_invalid_name_dept(){
		let mut a = RhDatabase::new();
		let star_somethng = "*something".to_string();
		match a.add_dept(star_somethng).err(){
			Some(RhDbError::InvalidDeptName) => {
				println!("Already Exists");
				assert!(true);
			},
			None => assert!(false, "No error for dept name starting with *"),
			Some(a) => {
				println!("Xavasca {:?}", a);
				assert!(false, a);
			}
		}
	}

	#[test]
	fn test_adding_empty_name_dept() {
		let mut a = RhDatabase::new();
		match a.add_dept("".to_string()).err(){
			Some(RhDbError::InvalidDeptName) => {
				println!("Empty string Error");
				assert!(true);
			},
			None => assert!(false, "No error for empty string name"),
			Some(a) => assert!(false, a),
		}
	}

	#[test]
	fn test_add_employee() {
		let mut a = RhDatabase::new();
		a.add_dept("Dept".to_string());
		a.add_employee("Daniel".to_string(), "Dept".to_string());
		let employee_list = a.data.get(&"Dept".to_string());
		assert!(employee_list.is_some());
		
	}


}