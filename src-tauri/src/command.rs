use std::collections::HashMap;
// use tauri::api::http;
// use tauri::api::http::{ClientBuilder, HttpRequestBuilder, ResponseType};
use std::{fmt, fs, time};
use std::any::Any;
use std::fs::File;
use std::io::{Read, Write};
use std::time::UNIX_EPOCH;
use hmac::{Hmac, Mac};
use serde_json::{json, Value};
use serde::{Serialize, Deserialize};
use md5::Md5;
use crate::encrypt::x_encode;
use sha1::{Sha1, Digest};
use urlencoding::encode;
const UA: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_12_6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/76.0.3809.100 Safari/537.36";
#[tauri::command]
pub async fn logout(username: String, ip: String) -> Result<Value> {
    let client = reqwest::ClientBuilder::new().no_proxy().build().unwrap();
    let response = client.get(format!("https://gw.buaa.edu.cn/cgi-bin/srun_portal?callback=jQuery112407419864172676014_1566720734115&action=logout&username={}&ac_id=1&ip={}", username, ip))
        .header("Host", "gw.buaa.edu.cn")
        .header("Accept", "text/javascript, application/javascript, application/ecmascript, application/x-ecmascript, */*; q=0.01")
        .header("DNT", "1")
        .header("X-Requested-With", "XMLHttpRequest")
        .header("User-Agent", UA)
        .header("Sec-Fetch-Mode", "cors")
        .header("Sec-Fetch-Site", "same-origin")
        .header("Referer", "https://gw.buaa.edu.cn/srun_portal_pc?ac_id=$AC_ID&theme=buaa&url=www.buaa.edu.cn")
        .header("Accept-Language", "en-US,en;q=0.9,zh-CN;q=0.8,zh;q=0.7,zh-TW;q=0.6")
        .send().await.unwrap();
    let r = response.text().await.unwrap();
    let r = r.trim_start_matches("jQuery112407419864172676014_1566720734115(").trim_end_matches(")");
    let r: Value = serde_json::from_str(r).unwrap();
    Ok(r)
}

#[tauri::command]
pub async fn login(username: String, password: String) -> Result<LoginResponse> {
    let client = reqwest::ClientBuilder::new().no_proxy().build().unwrap();
    let start = time::SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let timestamp = since_the_epoch.as_millis();
    let response = client.get(format!("https://gw.buaa.edu.cn/cgi-bin/get_challenge?callback=jQuery112403499023691859897_1655879182275&username={}&ip=10.135.150.220&_={}", username, timestamp))
        .header("Host", "gw.buaa.edu.cn")
        .header("Accept", "text/javascript, application/javascript, application/ecmascript, application/x-ecmascript, */*; q=0.01")
        .header("DNT", "1")
        .header("X-Requested-With", "XMLHttpRequest")
        .header("User-Agent", UA)
        .header("Sec-Fetch-Mode", "cors")
        .header("Sec-Fetch-Site", "same-origin")
        .header("Referer", "https://gw.buaa.edu.cn/srun_portal_pc?ac_id=$AC_ID&theme=buaa&url=www.buaa.edu.cn")
        .header("Accept-Language", "en-US,en;q=0.9,zh-CN;q=0.8,zh;q=0.7,zh-TW;q=0.6")
        .send().await.unwrap();
    let response = response.text().await.unwrap();
    let response = response.trim_start_matches("jQuery112403499023691859897_1655879182275(").trim_end_matches(")");
    let challenge: Challenge = serde_json::from_str(response).unwrap();
    println!("{}", challenge.challenge);
    let login_url = build_login_url(username, password, challenge.client_ip.clone(), "Mac+OS".to_string(), format!("{}", since_the_epoch.as_millis()), challenge.challenge.clone());
    let response = client.get(login_url)
        .header("Host", "gw.buaa.edu.cn")
        .header("Accept", "text/javascript, application/javascript, application/ecmascript, application/x-ecmascript, */*; q=0.01")
        .header("DNT", "1")
        .header("X-Requested-With", "XMLHttpRequest")
        .header("User-Agent", UA)
        .header("Sec-Fetch-Mode", "cors")
        .header("Sec-Fetch-Site", "same-origin")
        .header("Referer", "https://gw.buaa.edu.cn/srun_portal_pc?ac_id=$AC_ID&theme=buaa&url=www.buaa.edu.cn")
        .header("Accept-Language", "en-US,en;q=0.9,zh-CN;q=0.8,zh;q=0.7,zh-TW;q=0.6")
        .send().await.unwrap();
    let r = response.text().await.unwrap();
    let r = r.trim_start_matches("jQuery112407419864172676014_1566720734115(").trim_end_matches(")");
    println!("{}", r);
    let r: LoginResponse = serde_json::from_str(r).unwrap();
    Ok(r)
}

#[tauri::command]
pub async fn update_option(app_handle: tauri::AppHandle, payload: LoginOption) -> Result<LoginOption> {
    let buf = app_handle.path_resolver().app_dir().unwrap();
    let config_path_buf = buf.join("config.txt");
    // let option = File::options().create(true).write(true).truncate(true).read(true);
    let mut config_file = File::options().create(true).write(true).truncate(true).read(true).open(config_path_buf.as_path()).unwrap();
    let result = serde_json::to_string_pretty(&payload).unwrap();
    config_file.write(result.as_bytes()).unwrap();
    Ok(payload)
}

#[tauri::command]
pub async fn fetch_option(app_handle: tauri::AppHandle) -> Result<LoginOption> {
    let buf = app_handle.path_resolver().app_dir().unwrap();
    if !buf.exists() {
        fs::create_dir_all(buf.as_path()).unwrap();
    }
    let config_path_buf = buf.join("config.txt");
    if !config_path_buf.exists() {
        let mut file = File::create(config_path_buf.as_path()).unwrap();
        let default_option = LoginOption::default();
        let result = serde_json::to_string_pretty(&default_option).unwrap();
        file.write(result.as_bytes()).unwrap();
        return Ok(default_option);
    }
    // let option = File::options().create(true).write(true).truncate(true).read(true);
    let mut config_file = File::options().create(true).write(true).read(true).open(config_path_buf.as_path()).unwrap();
    let mut config_str = String::new();
    config_file.read_to_string(&mut config_str).unwrap();
    let config: LoginOption = serde_json::from_str(config_str.as_str()).unwrap();
    Ok(config)
}


type HmacMD5 = Hmac<Md5>;

fn build_login_url(username: String, raw_password: String, client_ip: String, sys_name: String, timestamp: String, challenge: String) -> String {
    // encrypt password
    let mut mac = HmacMD5::new_from_slice(challenge.as_bytes()).unwrap();
    mac.update(raw_password.as_bytes());
    let result = mac.finalize();
    let password = result.into_bytes();
    let encrypted_password = hex::encode(&password);
    // encrypt info
    let info_str = format!(r#"{{"username":"{}","password":"{}","ip":"{}","acid":"1","enc_ver":"srun_bx1"}}"#, username, raw_password, client_ip);
    let info_encoded = format!(r#"{{SRBX1}}{}"#, x_encode(info_str, challenge.clone()));
    // checksum
    let list: Vec<&str> = vec!["", username.as_str(), encrypted_password.as_str(), "1", client_ip.as_str(), "200", "1", info_encoded.as_str()];
    let chksum_str = list.join(challenge.as_str());
    let mut hasher = Sha1::new();
    hasher.update(chksum_str.as_bytes());
    let checksum = hex::encode(&hasher.finalize());
    // let encoded_checksum = encode(checksum.as_str());
    let encrypted_password = encode(encrypted_password.as_str());
    let info_encoded = encode(info_encoded.as_str());
    format!("https://gw.buaa.edu.cn/cgi-bin/srun_portal?callback=jQuery112407419864172676014_1566720734115&action=login&username={}&password={{MD5}}{}&ac_id=1&ip={}&chksum={}&info={}&n=200&type=1&os={}&name=Macintosh&double_stack=0&_={}",
            username, encrypted_password, client_ip, checksum, info_encoded, sys_name, timestamp
    )
}



#[derive(Clone, Serialize, Deserialize)]
struct Challenge {
    challenge: String,
    client_ip: String,
    ecode: i32,
    error: String,
    error_msg: String,
    expire: String,
    online_ip: String,
    res: String,
    srun_ver: String,
    st: i64,
}

#[derive(Debug, Clone,Serialize, Deserialize)]
pub struct LoginResponse {
    ServerFlag: Option<i32>,
    ServicesIntfServerIP: Option<String>,
    ServicesIntfServerPort: Option<String>,
    access_token: Option<String>,
    checkout_date: Option<i32>,
    client_ip: Option<String>,
    error: Option<String>,
    error_msg: Option<String>,
    online_ip: Option<String>,
    ploy_msg: Option<String>,
    real_name: Option<String>,
    remain_flux: Option<i32>,
    remain_times: Option<i32>,
    res: Option<String>,
    srun_ver: Option<String>,
    suc_msg: Option<String>,
    sysver: Option<String>,
    username: Option<String>,
    wallet_balance: Option<i32>,
}

#[derive(Debug, Clone,Serialize, Deserialize, Default)]
pub struct LoginOption {
    pub auto_start: bool,
    pub auto_login: bool,
    pub remember_password: bool,
    pub password: Option<String>,
    pub username: Option<String>
}


type Result<T> = std::result::Result<T, GeneralError>;

#[derive(Debug, Clone,Serialize, Deserialize)]
pub struct GeneralError {
    msg: String
}

impl GeneralError {
    pub fn new(msg: String) -> Self {
        GeneralError {
            msg
        }
    }
    pub fn message(&self) -> &str {
        self.msg.as_str()
    }
}

impl fmt::Display for GeneralError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg.as_str())
    }
}