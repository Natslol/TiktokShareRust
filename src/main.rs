use std::io;

fn main() {
    println!("Shares id -> ");
    let mut id = String::new();
    io::stdin().read_line(&mut id).expect("Impossible de lire la ligne");
    id.pop();
    if id.contains("\r") { id.pop(); }
    share(id.trim())
}

fn share(id: &str) {
    let mut share = 0;
    loop {
        ureq::agent().post("https://api19.tiktokv.com/aweme/v1/aweme/stats/?channel=tiktok_web&device_type=SM-G9900&device_id=9999999999999999999&os_version=11&version_code=220400&app_name=tiktok_web&device_platform=android&aid=1988").set("Content-Type", "application/x-www-form-urlencoded; charset=UTF-8").query("item_id", id).query("share_delta", "1").call();
        share = share + 1;
        println!("Shares: {}", share)
    }
}
