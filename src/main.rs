
// use std::process::Command;
use webbrowser;
use opener;
fn main () {

    // open_github();

    open_spotify();
    
    open_vscode();

}

// fn open_github () {
//     let my_url = "https://github.com/Qhaleed?tab=repositories";
//     if webbrowser::open(&my_url).is_ok() {
//         println!("Browser opened");
//     }else {
//         eprintln!("Failed to open {}", my_url)
//     }
// }

fn open_vscode() {
    let directory = "C:\\Users\\Qhal\\Desktop\\CS\\Personal\\Rust";  // Replace this with the directory you want to open
    let vscode_path = "C:\\Users\\Qhal\\AppData\\Local\\Programs\\Microsoft VS Code\\code.exe";
    let command = format!("\"{}\" \"{}\"", vscode_path, directory);

    // Open VS Code with the specified directory
    let _vscode = std::process::Command::new("cmd")
        .args(&["/C", &command])  // Use a slice here
        .spawn()
        .expect("Failed to open VS Code");
}


fn open_spotify () {
    println!("Open Spotify");
    let _spotify = opener::open
    ("C:\\Users\\Qhal\\AppData\\Roaming\\Spotify\\Spotify")
    .expect("Error");
}
