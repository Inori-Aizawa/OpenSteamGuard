#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#[macro_use]
extern crate anyhow;
#[path = "./steam/accountmanager.rs"]
mod accountmanager;
use accountmanager::Manifest;
use accountmanager::ManifestEntry;
use steamguard::{ExposeSecret, SteamGuardAccount};
use steamguard::{UserLogin,LoginError,Confirmation};
use tauri::Window;
use log::*;
#[path = "./steam/errors.rs"]
mod errors;
#[path = "./steam/encryption.rs"]
mod encryption;

#[macro_use]
extern crate lazy_static;


lazy_static! {
    static ref MANIFEST: Manifest = get_configs_with_password();
    static ref PASSWORD: String = input("Enter password: ");
}



// remember to call `.manage(MyState::default())`
fn get_auth_codes() -> Vec<(String, String, String)>{
    let configs = &MANIFEST;

    let mut auth_codes = Vec::new();
    //if the accounts are not loaded yet
    if configs.get_all_loaded().is_empty() {
        auth_codes.push(("error".to_string(), "accounts not loaded".to_string(), "-1".to_string()));
        return auth_codes;
    }
    for config in &configs.entries {
        let account = configs.get_account(&config.account_name).unwrap().lock().unwrap().clone();
        let code = generate_code_for_account(account);
        auth_codes.push((config.account_name.to_string(), code,config.steam_id.to_string()));
    }

    return auth_codes;

}
#[tauri::command]
fn heartbeat(window: Window){
    window.emit("auth_codes", get_auth_codes()).unwrap();
} 
#[tauri::command]
fn get_trade_confirmations(account_name: String, password: String) -> Vec<(String, String, String)>
{
    //if either the account name or password is empty, return an error
    if account_name.is_empty() || password.is_empty() {
        return vec![("error".to_string(), "account name or password is empty".to_string(), "-1".to_string())];
    }
    if !does_steamguard_acc_exist(account_name.clone()){
        return vec![("error".to_string(), "Account does not exist".to_string(), "-1".to_string())];
    }
    println!("Getting trade confirmations for {}", account_name.clone());

    let mut confirmations: Vec<(String, String, String)> = Vec::new();
    let mut account = get_steamguard_acc(account_name.to_string());
    let mut session = login_to_steam(account_name.to_string(), password.to_string(), &account);
    // let mut session = account.session.as_ref().unwrap().expose_secret().clone();
    if session.is_err() {
        confirmations.push(("error".to_string(), "login failed".to_string(), "-3".to_string()));
        println!("Login failed: {:?}", session.err().unwrap());
        return confirmations;
    }
    account.set_session(session.unwrap());
    loop {
		match account.get_trade_confirmations() {
			Ok(confs) => {
				for confirm in confs {
                    confirmations.push((confirm.id.to_string(),confirm.description,confirm.creator.to_string()));
                }
                if confirmations.len() > 0 {
                    println!("Found {} confirmations", confirmations.len());
                    break;
                }
                else {
                    println!("No confirmations found");
                    return vec![("error".to_string(), "No confirmations found".to_string(), "-2".to_string())];

                }
			}
			Err(_) => {
                
			    println!("failed to get trade confirmations, asking user to log in");
			}
		}
	}
    return confirmations;
}

fn main() {
    let _ = &PASSWORD;
    //wait for the user to enter their password
    while PASSWORD.is_empty() {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
  
    
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![heartbeat, get_trade_confirmations])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
fn get_configs_with_password() -> accountmanager::Manifest
{
    let path = dirs::home_dir().unwrap().join(".config/steamguard-cli/maFiles/manifest.json");
    println!("path: {:?}", path);
    let mut accounts = accountmanager::Manifest::load(&path).unwrap();
    //send a message to the frontend to get the password
    
    let password = PASSWORD.to_string();
    accounts.submit_passkey(Some(password.to_string()));
    if accounts.load_accounts().is_err() {
        //return empty manifest
        println!("error loading accounts");
        //exit the program
        std::process::exit(1);
        return accounts;
    }
    return accounts;
}
fn does_steamguard_acc_exist(account_name: String) -> bool
{
    let configs = &MANIFEST;
    let account = configs.account_exists(&account_name);
    return account;
}
fn get_steamguard_acc(account: String) -> SteamGuardAccount{
    let configs = &MANIFEST;
    if configs.account_exists(&account.to_string()){
        let account = configs.get_account(&account).unwrap().lock().unwrap().clone();
        return account;
    }
    else {
        println!("Account {} does not exist", &account);
        return SteamGuardAccount::new();
    }
    // let account = configs.get_account(&account).unwrap().lock().unwrap().clone();
    // return account;
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

fn login_to_steam(	username: String,
	password: String,
	account: &SteamGuardAccount
) -> anyhow::Result<steamguard::steamapi::Session> {
	// TODO: reprompt if password is empty
	let mut login = UserLogin::new(username, password);
	let mut loops = 0;
	loop {
		match login.login() {
			Ok(s) => {
				return Ok(s);
			}
			Err(LoginError::Need2FA) => match account {
                a => {
					let server_time = steamguard::steamapi::get_server_time()?.server_time;
					login.twofactor_code = a.generate_code(server_time);
				}
			},
			Err(LoginError::NeedCaptcha { captcha_gid }) => {
				debug!("need captcha to log in");
                println!("need captcha to log in: https://steamcommunity.com/public/captcha.php?gid={}", &captcha_gid);
				login.captcha_text = input(Some("Enter captcha: "));
			}
			Err(LoginError::NeedEmail) => {
				println!("You should have received an email with a code.");
				print!("Enter code: ");
				login.email_code = input(None);
			}
			Err(r) => {
				error!("Fatal login result: {:?}", r);
				bail!(r);
			}
		}
		loops += 1;
		if loops > 2 {
			error!("Too many loops. Aborting login process, to avoid getting rate limited.");
			bail!("Too many loops. Login process aborted to avoid getting rate limited.");
		}
	}
}