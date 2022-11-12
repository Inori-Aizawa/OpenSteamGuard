#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#[path = "./steam/accountmanager.rs"]
mod accountmanager;
use accountmanager::Manifest;
use accountmanager::ManifestEntry;
use steamguard::{ExposeSecret, SteamGuardAccount};
#[path = "./steam/errors.rs"]
mod errors;
#[path = "./steam/encryption.rs"]
mod encryption;

#[macro_use]
extern crate lazy_static;


lazy_static! {
    static ref MANIFEST: Manifest = get_configs_with_password();
}



// remember to call `.manage(MyState::default())`
#[tauri::command]
fn get_auth_codes() -> Vec<String>{
    let mut configs = &MANIFEST;
    let mut auth_codes = Vec::new();
    for config in &configs.entries {
        let account = configs.get_account(&config.account_name).unwrap().lock().unwrap().clone();
        let code = generate_code_for_account(account);
        println!("code: {:?}", code);
    }
    return auth_codes;

}

fn main() {

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_auth_codes])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
fn get_configs_with_password() -> accountmanager::Manifest
{
    let path = dirs::home_dir().unwrap().join(".config/steamguard-cli/maFiles/manifest.json");
    println!("path: {:?}", path);
    let mut accounts = accountmanager::Manifest::load(&path).unwrap();
    let password = input("Enter password: ");
    accounts.submit_passkey(Some(password.to_string()));
    if accounts.load_accounts().is_err() {
        //return empty manifest
        println!("error loading accounts");
        return accountmanager::Manifest::default();
    }


    return accounts;
}
fn get_configs() -> accountmanager::Manifest
{
    let path = dirs::home_dir().unwrap().join(".config/steamguard-cli/maFiles/manifest.json");
    println!("path: {:?}", path);
    let mut accounts = accountmanager::Manifest::load(&path).unwrap();
    if accounts.load_accounts().is_err() {
        //return empty manifest
        println!("error loading accounts");
        return accountmanager::Manifest::default();
    }
    accounts.load_accounts().unwrap();

    return accounts;
}
fn generate_code_for_account(steamguardAccount:  SteamGuardAccount) -> String {
    let time = get_time();
    let code = steamguardAccount.generate_code(time);
    return code;
}
fn get_time() -> u64 {
    // let time = steamguard::steamapi::get_server_time().unwrap().server_time;
    let time: u64 = chrono::Utc::now().timestamp().try_into().unwrap();
    return time;
}

pub fn input<'a>(s: impl Into<Option<&'a str>>) -> String {
	let s_into = s.into();
	
	if s_into != None {
		println!("{}", s_into.unwrap());
	}
	
	let mut val = String::new();
	std::io::stdin().read_line(&mut val).expect("Something went wrong with the input.");
	
	return val[0..val.len() - 1].to_string();
}