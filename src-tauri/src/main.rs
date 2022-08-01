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
use serde::{Serialize, Deserialize};
use rand::seq::IteratorRandom;
use rand::prelude::SliceRandom;

lazy_static! {
  static   ref   WORKING_PROXIES:Mutex<Vec<String>> = Mutex::new(vec![]);
}
#[derive(Serialize, Deserialize,Debug)]
struct Sleep{
  min:u64,
  max:u64,
}


const  PROXIES_FILE:&str = "proxies.txt";
const URLS_FILE:&str = "urls.txt";
const UAGENTS_FILE:&str = "user_agents.txt";




fn search_google(query:&str,proxies:&mut Vec<String>) ->String{
let proxy_length = proxies.len();
let user_agents =  std::fs::read_to_string(UAGENTS_FILE)
.expect("Something went wrong reading the UAGENTS_FILE");
let user_agents = user_agents.split("\r\n").filter(|s| !s.is_empty()).collect::<Vec<&str>>();
let user_agent = *user_agents.choose(&mut rand::thread_rng()).unwrap();
while proxies.len() > 0 {
  println!("{} left to try from {}",proxies.len(),proxy_length);
    let proxy = proxies.pop().unwrap();
    let client = reqwest::blocking::Client::builder()
    .proxy(reqwest::Proxy::all(&proxy).unwrap())
    .build().unwrap();
  let   result =  match client.get(&format!("https://www.google.com/search?q={}",query)).header("user-agent", user_agent).send(){
      Ok(resp)=>{
        let resp_txt = resp.text().unwrap().to_string();
        if  !resp_txt.contains("sending requests very quickly") {
          resp_txt
      }else{
        "404".to_string()
        }},
      Err(_)=> "404".to_string()
    };
    if result != "404" {
        return result;
    }
}

  let client = reqwest::blocking::Client::builder()
  .build().unwrap();
client.get(&format!("https://www.google.com/search?q={}",query)).header("user-agent", user_agent).send().unwrap().text().unwrap().to_string()
 


}


#[tauri::command]
fn read_html() ->String{

  let contents = std::fs::read_to_string("html.html")
  .expect("Something went wrong reading the file");

  contents
}

#[tauri::command]
fn start_crawling(window:Window,keywords:Vec<String>,proxies:Vec<String>,sleep:Sleep){
  println!("{:?}",sleep);
  std::thread::spawn(move ||{
    let mut count = 0;
    for keyword in &keywords {
      let mut proxies = proxies.clone();
      println!("{:?}",keywords);
      count+=1;
      let html = search_google(&keyword,&mut proxies);
      window.emit("parse_html",html).unwrap();
      let json = format!("{{\"keyword\":\"{}\",\"progress\":{{\"count\":\"{}\" , \"maxValue\": \"{}\"      }}        }}",keyword,count,keywords.len());
      window.emit("progress",json).unwrap();
      //random between numbers between min and max
      let sleep_duration = (sleep.min..=sleep.max).choose(&mut rand::thread_rng()).unwrap();
     println!("sleep_duration: {}",sleep_duration);
      std::thread::sleep(Duration::from_millis(sleep_duration));
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
fn save_proxies(proxies:String){
// write to a file
  let mut file = File::create(PROXIES_FILE).unwrap();
    file.write_all(proxies.as_bytes()).unwrap();
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
  .invoke_handler(tauri::generate_handler![start_crawling,read_html,append_urls_to_file,test_proxies,save_proxies,get_proxies_from_file])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
