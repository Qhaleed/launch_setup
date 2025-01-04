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

fn open_vscode () {
    let _vscode = opener::open("C:\\Users\\Qhal\\AppData\\Local\\Programs\\Microsoft VS Code\\code.exe");
}

fn open_spotify () {
    println!("Open Spotify");
    let _spotify = opener::open
    ("C:\\Users\\Qhal\\AppData\\Roaming\\Spotify\\Spotify")
    .expect("Error");
}
