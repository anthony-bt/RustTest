extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use std::str;
use std::process::Command;

fn run_command_line(url: String) {
    println!("Download video from URL '{}' in progress", url);
    let output = Command::new("youtube-dl")
                          .arg("--audio-format")
                          .arg("mp3")
                          .arg("--audio-quality")
                          .arg("0")
                          .arg("--no-check-certificate")
                          .arg("-x")
                          .arg(url)
                          .output()
                          .expect("failed to execute process");

    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    // println!("Output: {:?}", readable_output);
    // let readable_output = str::from_utf8(&output.stdout).unwrap();
    assert!(output.status.success());
}

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let glade_src = include_str!("interface.glade");
    let builder = gtk::Builder::new_from_string(glade_src);
    
    let window: gtk::Window = builder.get_object("window1").unwrap();
    let download: gtk::Button = builder.get_object("button1").unwrap();
    let entry: gtk::Entry = builder.get_object("entry1").unwrap();

    download.connect_clicked(move |_| {
        println!("Clicked Download");
        let url = entry.get_text().unwrap().to_string();
        run_command_line(url);
    });

    window.show_all();
    
    gtk::main();
}
