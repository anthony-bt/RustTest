extern crate gtk;
extern crate gio;
use gtk::prelude::*;
use std::process::Command;

pub mod grid;

const CSS: &str = include_str!("../ressources/style.css");
const GLADE: &str = include_str!("../ressources/interface.glade");

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
    // Test GRID
    let mut f1 = grid::file::File::new("file1", "mp3");
    let mut f2 = grid::file::File::new("file2", "mp3");
    f1.display();
    f2.display();
    f1.set_progress(10.0);
    f2.set_progress(50.0);
    f1.display();
    f2.display();
    println!("URL: {}", &mut f1.get_url());
    println!("URL: {}", &mut f1.get_url());
    println!("URL: {}", &mut f2.get_url());
    // End Test GRID

    // initialize
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    // build window
    let builder = gtk::Builder::new_from_string(GLADE);
    let window: gtk::Window = builder.get_object("window1").unwrap();
    
    // define CSS provider
    let provider = gtk::CssProvider::new();
    provider.load_from_data(CSS.as_bytes())
            .expect("Failed to load CSS");

    // give CSS provider to the default screen
    gtk::StyleContext::add_provider_for_screen(
        &gdk::Screen::get_default().expect("Error initializing gtk css provider."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_USER
    );
    
    // controls
    // let label: gtk::Label = builder.get_object("tabletitlelabel1").unwrap();
    // let download: gtk::Button = builder.get_object("button1").unwrap();
    // let entry: gtk::Entry = builder.get_object("entry1").unwrap();

    // download.connect_clicked(move |_| {
    //     println!("Clicked Download");
    //     let url = entry.get_text().unwrap().to_string();
    //     run_command_line(url);
    // });

    window.show_all();
    
    gtk::main();
}