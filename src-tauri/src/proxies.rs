use std::sync::{Arc,Mutex};
pub fn test(proxies:Vec<String>)->Vec<String> {
let counter = Arc::new(Mutex::new(proxies.len()));
let working_proxies = Arc::new(Mutex::new(vec![]));
for proxy in proxies {

    let counter = counter.clone();
    let _working_proxies = working_proxies.clone();
    std::thread::spawn(move||{

        let client = reqwest::blocking::Client::builder()
        .proxy(reqwest::Proxy::all(&proxy).unwrap())
        .build().unwrap();
       let result = match client.get(&format!("https://www.google.com/search?q=test")).header("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/103.0.0.0 Safari/537.36").send(){
        
        Ok(resp)=>!resp.text().unwrap().to_string().contains("sending requests very quickly"),
        Err(_)=>false
    };
    *counter.lock().unwrap()-=1;
    if result {
        _working_proxies.lock().unwrap().push(proxy);
    }
    println!("{}",result);
    });
  
}
while *counter.lock().unwrap()>0{
    std::thread::sleep(std::time::Duration::from_millis(100));
}

return working_proxies.lock().unwrap().to_vec();
}


