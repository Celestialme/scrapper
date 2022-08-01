#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
mod proxies;
#[macro_use]
extern crate lazy_static;
use std::io::Write;
use std::fs::File;
use tauri::Window;
use std::time::Duration;
use std::sync::Mutex;
lazy_static! {
  static   ref   WORKING_PROXIES:Mutex<Vec<String>> = Mutex::new(vec![]);
}

const  PROXIES_FILE:&str = "proxies.txt";
const URLS_FILE:&str = "urls.txt";
fn search_google(query:&str) ->String{

  let client = reqwest::blocking::Client::builder()
  .build().unwrap();
  client.get(&format!("https://www.google.com/search?q={}",query)).header("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/103.0.0.0 Safari/537.36").send().unwrap().text().unwrap().to_string()
}


#[tauri::command]
fn read_html() ->String{

  let contents = std::fs::read_to_string("html.html")
  .expect("Something went wrong reading the file");

  contents
}

#[tauri::command]
fn start_crawling(window:Window,keywords:Vec<String>){
  std::thread::spawn(move ||{
    let mut count = 0;
    for keyword in &keywords {
      println!("{:?}",keywords);
      count+=1;
      let html = search_google(&keyword);
      window.emit("parse_html",html).unwrap();
      let json = format!("{{\"keyword\":\"{}\",\"progress\":{{\"count\":\"{}\" , \"maxValue\": \"{}\"      }}        }}",keyword,count,keywords.len());
      window.emit("progress",json).unwrap();
      std::thread::sleep(Duration::from_millis(5000));
    }
  });
}

// round integer




#[tauri::command]
fn test_proxies(proxies:Vec<String>)->Vec<String>{
  let working_proxies  = proxies::test(proxies);
  *WORKING_PROXIES.lock().unwrap() = working_proxies.clone();
  working_proxies
}




#[tauri::command]
fn append_urls_to_file(list:Vec<String>){
// check if urls.txt exists if not create one;

if !std::path::Path::new(URLS_FILE).exists(){
   File::create(URLS_FILE).unwrap();
}
    let mut file = std::fs::OpenOptions::new()
    .append(true)
    .open(URLS_FILE)
    .unwrap();
    for url in list {
      file.write_all(url.as_bytes()).unwrap();
      file.write_all("\n".as_bytes()).unwrap();
    }
  


}

#[tauri::command]
fn get_proxies_from_file()->String{

if !std::path::Path::new(PROXIES_FILE).exists(){
  // create proxy file
  File::create(PROXIES_FILE).unwrap();
}
//read proxy file
std::fs::read_to_string(PROXIES_FILE).unwrap()

}







fn main() {
  tauri::Builder::default()
  .invoke_handler(tauri::generate_handler![start_crawling,read_html,append_urls_to_file,test_proxies,get_proxies_from_file])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
