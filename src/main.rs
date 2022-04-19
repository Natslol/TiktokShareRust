use std::io;
use std::thread;

fn main() {
    let mut share = 0;
    let id: String = url();
    loop {
        ureq::agent().post("https://api19.tiktokv.com/aweme/v1/aweme/stats/?channel=tiktok_web&device_type=SM-G9900&device_id=9999999999999999999&os_version=11&version_code=220400&app_name=tiktok_web&device_platform=android&aid=1988").set("Content-Type", "application/x-www-form-urlencoded; charset=UTF-8").query("item_id", id.trim()).query("share_delta", "1").call();
        share = share + 1;
        println!("Shares: {}", share)
    }
}

fn url() -> String {
    println!("Shares link -> ");
    let mut id = String::new();
    io::stdin().read_line(&mut id).expect("Impossible de lire la ligne");
    id.pop();
    if id.contains("\r") { id.pop(); }
    println!("{:?}", id);
    id
}
