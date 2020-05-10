use std::fmt;

/*
 * Downlaod status
 */
#[allow(dead_code)]
enum Status {
	ToDownload, // download added to the list to download
	InProgress, // download in progress
	Pause,      // download paused
	Stop,       // download stopped
	Success,    // download successed
	Cancelled   // download cancelled
}

impl fmt::Display for Status {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		 match *self {
			Status::ToDownload => write!(f, "TO DOWNLOAD"),
			Status::InProgress => write!(f, "IN PROGRESS"),
			Status::Pause => write!(f, "PAUSE"),
			Status::Stop => write!(f, "STOP"),
			Status::Success => write!(f, "SUCCESS"),
			Status::Cancelled => write!(f, "CANCELLED"),
		 }
	}
}

/*
 * Struct to define an URL to download as a file
 */
// #[derive(Copy, Clone)]
pub struct File<'a> {
	url: &'a str,    // download URL address
	progress: f64,   // download progress
	format: &'a str, // download format
	status: Status   // download status
}

#[allow(dead_code)]
impl File<'_> {

	/*
	 * Constructor
	 */
	pub fn new<'a>(url: &'static str, format: &'static str) -> File<'a> {
		File {
			url: url,
			progress: 0.0,
			format: format,
			status: Status::ToDownload
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
