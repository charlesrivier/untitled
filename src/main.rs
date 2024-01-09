fn main() {
    let response = reqwest::blocking::get("https://www.nostalgeek-serveur.com/db/?item=787");
    let data = response.unwrap().text().unwrap();
    println!("{data}");
}
