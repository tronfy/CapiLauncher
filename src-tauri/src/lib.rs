use base64::prelude::*;
use homedir::my_home;
use open_launcher::{auth, version, Launcher};
use rand::Rng;
use std::{
    fs::File,
    io::{Read, Write},
    path::{self, Path, PathBuf},
    process::Command,
};
use tauri::{AppHandle, Emitter};

use flate2::read::GzDecoder;

fn get_home_dir() -> PathBuf {
    let home = match my_home() {
        Ok(home) => match home {
            Some(home) => home,
            None => {
                panic!("Failed to get home directory");
            }
        },
        Err(e) => {
            panic!("Failed to get home directory: {}", e);
        }
    };

    home
}

fn get_launcher_dir() -> PathBuf {
    let home = get_home_dir();

    let launcher_dir = home.join(".capilauncher").join("X");

    launcher_dir
}

fn get_game_dir() -> PathBuf {
    let launcher_dir = get_launcher_dir();

    let game_dir = launcher_dir.join("minecraft");

    game_dir
}

#[tauri::command]
fn log(message: String) {
    println!("[log] {}", message);
}

#[tauri::command]
fn save_nick(nickname: String) {
    println!("saving nickname: {}", nickname);
    let game_dir = get_game_dir();

    let nick_file = game_dir.join(".user");
    if nick_file.exists() {
        return;
    }

    let mut file = File::create(nick_file).unwrap();

    // let nick_base64 = base64::encode(nickname);
    let nick_base64 = BASE64_STANDARD.encode(nickname.as_bytes());

    file.write_all(nick_base64.as_bytes()).unwrap();
}

// #[tauri::command]
// fn authorize(nickname: String, token: String) -> bool {
//     // get https://auth.capivaramanca.com.br/link?nick=NICKNAME&token=TOKEN
//     let url = format!(
//         "https://auth.capivaramanca.com.br/link?nick={}&token={}",
//         nickname, token
//     );

//     let response = reqwest::blocking::get(&url).unwrap();

//     if response.status().is_success() {
//         save_nickname(nickname);
//         // return Ok(());
//         return true;
//     }

//     // Err("Falha ao autorizar".to_string())
//     false
// }

#[tauri::command]
fn get_nick() -> String {
    println!("getting nickname");
    let game_dir = get_game_dir();

    let nick_file = game_dir.join(".user");

    if !nick_file.exists() {
        println!("Nickname not found");
        return "".to_string();
    }
    let mut file = File::open(nick_file).unwrap();

    let mut nick_base64 = String::new();
    file.read_to_string(&mut nick_base64).unwrap();

    let nick = BASE64_STANDARD.decode(nick_base64.as_bytes()).unwrap();

    println!("Nickname: {}", String::from_utf8(nick.clone()).unwrap());

    // Ok(String::from_utf8(nick).unwrap())
    String::from_utf8(nick).unwrap()
}

#[tauri::command]
async fn launch(app: AppHandle) {
    app.emit("msg", "iniciando").unwrap();

    let launcher_dir = get_launcher_dir();

    let game_dir = get_game_dir();

    let java_exec = get_java_exec(&app, launcher_dir.clone()).await;

    // if .sl_password does not exist, create it
    let sl_file = game_dir.join(".sl_password");
    if !sl_file.exists() {
        let mut file = File::create(sl_file).unwrap();
        // generate a 32 char random password
        let password = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(32)
            .map(char::from)
            .collect::<String>();

        file.write_all(password.as_bytes()).unwrap();
    }

    // get instance
    // http://api.capivaramanca.com.br/csmp/X/CSMPX.zip
    let mods_dir = game_dir.join("mods");
    let instance_file = launcher_dir.join("CSMPX.zip");
    if !mods_dir.exists() {
        app.emit("msg", "obtendo instância do modpack").unwrap();
        let url = "http://api.capivaramanca.com.br/csmp/X/CSMPX.zip";
        let response = reqwest::get(url).await.unwrap();
        let mut file = std::fs::File::create(&instance_file).unwrap();
        let bytes = response.bytes().await.unwrap();
        let mut cursor = std::io::Cursor::new(bytes);
        std::io::copy(&mut cursor, &mut file).unwrap();

        // unzip
        let zip = File::open(&instance_file).unwrap();
        let mut archive = zip::ZipArchive::new(zip).unwrap();
        archive.extract(launcher_dir.clone()).unwrap();

        // remove zip
        std::fs::remove_file(&instance_file).unwrap();
    }

    // packwiz
    app.emit("msg", "atualizando mods").unwrap();
    Command::new(java_exec.clone())
        .arg("-jar")
        .arg("packwiz-installer-bootstrap.jar")
        .arg("https://api.capivaramanca.com.br/csmp/X/pack.toml")
        .current_dir(game_dir.clone())
        .output()
        .expect("failed to execute process");

    let mut launcher = Launcher::new(
        game_dir.to_str().unwrap(),
        java_exec.as_str(),
        version::Version {
            minecraft_version: "1.20.1".to_string(),
            loader: Some("forge".to_string()),
            loader_version: Some("47.3.0".to_string()),
        },
    )
    .await;

    let nick = get_nick();

    if nick.is_empty() {
        println!("Nickname not found");
        app.emit("msg", "Nickname não encontrado").unwrap();
        return;
    }

    launcher.auth(auth::OfflineAuth::new(&nick));
    launcher.custom_resolution(1280, 720);
    // launcher.fullscreen(true);
    // launcher.quick_play("multiplayer", "hypixel.net");

    app.emit("msg", "iniciando instalação").unwrap();

    let mut progress = launcher.on_progress();
    tokio::spawn(async move {
        loop {
            match progress.recv().await {
                Ok(progress) => {
                    let percent = match progress.total {
                        0 => 0,
                        _ => (progress.current as f64 / progress.total as f64 * 100.0 * 100.0)
                            .round() as u64,
                    } as f32
                        / 100.0;

                    app.emit("msg", format!("{} ({}%)", progress.task, percent))
                        .unwrap();
                    println!(
                        "Progress: {} {}/{} ({}%)",
                        progress.task, progress.current, progress.total, percent
                    );
                }
                Err(_) => {
                    println!("Progress channel closed");
                    break;
                }
            }
        }
    });

    match launcher.install_version().await {
        Ok(_) => println!("Version installed successfully."),
        Err(e) => println!("An error occurred while installing the version: {}", e),
    };

    match launcher.install_assets().await {
        Ok(_) => println!("Assets installed successfully."),
        Err(e) => println!("An error occurred while installing the assets: {}", e),
    };

    match launcher.install_libraries().await {
        Ok(_) => println!("Libraries installed successfully."),
        Err(e) => println!("An error occurred while installing the libraries: {}", e),
    };

    let _process = match launcher.launch() {
        Ok(p) => p,
        Err(e) => {
            println!("An error occurred while launching the game: {}", e);
            std::process::exit(1);
        }
    };

    std::process::exit(0);

    // let _ = process.wait();
    // println!("Game closed.");
}

async fn get_java_exec(app: &AppHandle, launcher_dir: PathBuf) -> String {
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;

    println!("{}, {}", os, arch);

    let java_dir = launcher_dir.join("java");
    let base_url = "https://download.oracle.com/java/17/archive/jdk-17.0.12";

    let packed_file: &str;
    let java_exec: PathBuf;
    let mut unpacked_dir = "jdk-17.0.12";

    match arch {
        "x86_64" => match os {
            "linux" => {
                packed_file = "linux-x64_bin.tar.gz";
                java_exec = java_dir.join("bin").join("java");
            }
            "windows" => {
                packed_file = "windows-x64_bin.zip";
                java_exec = java_dir.join("bin").join("java.exe");
            }
            "macos" => {
                unpacked_dir = "jdk-17.0.12.jdk";
                packed_file = "macos-x64_bin.tar.gz";
                java_exec = java_dir
                    .join("Contents")
                    .join("Home")
                    .join("bin")
                    .join("java");
            }
            _ => {
                panic!("unsupported os/arch: {} {}", os, arch);
            }
        },
        "aarch64" => match os {
            "linux" => {
                packed_file = "linux-aarch64_bin.tar.gz";
                java_exec = java_dir.join("bin").join("java");
            }
            "macos" => {
                unpacked_dir = "jdk-17.0.12.jdk";
                packed_file = "macos-aarch64_bin.tar.gz";
                java_exec = java_dir
                    .join("Contents")
                    .join("Home")
                    .join("bin")
                    .join("java");
            }
            _ => {
                panic!("unsupported os/arch: {} {}", os, arch);
            }
        },
        _ => {
            panic!("unsupported architecture: {}", arch);
        }
    };

    println!("java exec path: {}", java_exec.to_str().unwrap());

    // if exec not found, download and extract
    if !java_exec.exists() {
        app.emit("msg", "atualizado versão do java").unwrap();
        println!("downloading java...");
        let url = format!("{}_{}", base_url, packed_file);

        let response = reqwest::get(&url).await.unwrap();
        let dest = launcher_dir.join(packed_file);

        let mut file = std::fs::File::create(&dest).unwrap();
        let bytes = response.bytes().await.unwrap();
        let mut cursor = std::io::Cursor::new(bytes);
        std::io::copy(&mut cursor, &mut file).unwrap();

        match os {
            "linux" => {
                let tar = File::open(&dest).unwrap();
                let decoder = GzDecoder::new(tar);
                let mut archive = tar::Archive::new(decoder);
                archive.unpack(launcher_dir.clone()).unwrap();
            }
            "windows" => {
                let zip = File::open(&dest).unwrap();
                let mut archive = zip::ZipArchive::new(zip).unwrap();
                archive.extract(launcher_dir.clone()).unwrap();
            }
            "macos" => {
                let tar = File::open(&dest).unwrap();
                let decoder = GzDecoder::new(tar);
                let mut archive = tar::Archive::new(decoder);
                archive.unpack(launcher_dir.clone()).unwrap();
            }
            _ => {
                panic!("unsupported os: {}", os);
            }
        }

        // rename unpacked dir
        let unpacked_dir_path = launcher_dir.join(unpacked_dir);
        let java_dir_path = launcher_dir.join("java");
        std::fs::rename(unpacked_dir_path, java_dir_path).unwrap();

        // remove packed file
        std::fs::remove_file(dest).unwrap();
    }

    java_exec.to_str().unwrap().to_string()
}

pub async fn run() {
    let launcher_dir = get_launcher_dir();

    if !launcher_dir.exists() {
        std::fs::create_dir_all(&launcher_dir).unwrap();
    }

    let game_dir = get_game_dir();

    if !game_dir.exists() {
        std::fs::create_dir_all(&game_dir).unwrap();
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![launch, get_nick, save_nick, log])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
