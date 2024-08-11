#[derive(Clone)]
pub struct Loc {
	pub row: usize,
	pub col: usize,
	pub filepath: String, 
}

impl std::fmt::Display for Loc {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}:{}:{}", self.filepath, self.row, self.col)
	}
}

impl Loc {
	pub fn new(row: usize, col: usize, filepath: String) -> Self {
		Self{row, col, filepath}
	}
	pub fn error(&self, message: String) {
		eprintln!("{}: error: {}", self, message);
		std::process::exit(1);
	}
	pub fn message(&self, message: String) {
		eprintln!("{}: {}", self, message);
	}
	pub fn null() -> Self {
		Self{row: 0, col: 0, filepath: String::from("")}
	}
}
