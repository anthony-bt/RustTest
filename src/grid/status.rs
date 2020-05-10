use std::fmt;

/*
 * Enum to define download statuses
 */
#[allow(dead_code)]
#[derive(PartialEq)]
pub enum Status {
	ToDownload, // download added to the download list
	InProgress, // download in progress
	Pause,      // download paused
	Stop,       // download stopped
	Success,    // download successed
	Cancelled   // download cancelled
}

impl fmt::Debug for Status {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
			write!(f, "{:?}", self)
	}
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