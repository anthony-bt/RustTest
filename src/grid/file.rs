use super::status;

/*
 * Struct to define an URL to download as a file
 */
// #[derive(Copy, Clone)]
pub struct File<'a> {
	url: &'a str,    			 // download URL address
	progress: f64,   			 // download progress
	format: &'a str, 			 // download format
	status: status::Status // download status
}

impl File<'_> {

	/*
	 * Constructor
	 */
	pub fn new<'a>(url: &'static str, format: &'static str) -> File<'a> {
		File {
			url: url,
			progress: 0.0,
			format: format,
			status: status::Status::ToDownload
		}
	}

	/*
	 * Display object to string
	 */
	pub fn display(&mut self) {
		println!("File: {}.{}\nProgress: {} %\nStatus: {}", self.url, self.format, self.progress, self.status);
	}

	/*
	 * Get URL
	 */
	pub fn get_url(&mut self) -> &str {
		self.url
	}

	/*
	 * Get format
	 */
	 pub fn get_fomat(&mut self) -> &str {
		 self.format
	 }

	/*
	 * Set progress
	 */
	pub fn set_progress(&mut self, progress: f64) {
		self.progress = progress;
	}

	/*
	 * Get progress
	 */
	pub fn get_progress(&mut self) -> f64 {
		self.progress
	}

}
