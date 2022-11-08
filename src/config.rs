use crate::package;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub info: Info,
    pub profiles: Option<HashMap<String, Profile>>,
    pub distro_info:Option<HashMap<String,DistroInfo>>
}
impl Config {
    fn init_new() -> Config {
        Config {
            info: Info::init_new(),
            profiles: Some(HashMap::from([(
                String::from("default"),
                Profile::init_default(),
            )])),
            distro_info:Some(DistroInfo::init_default())
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Info {
    pub profile: String,
    pub version: String
}
impl Info {
    fn init_new() -> Info {
        Info {
            profile: String::from("default"),
            version: get_version()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub distro: String,
    pub font_dir: Option<String>,
    pub apps: Option<Vec<String>>,
    pub repos: Option<Vec<Repo>>,
}
impl Profile {
    fn init_default() -> Profile {
        Profile {
            distro: package::detect_distro(),
            font_dir: Some("".to_string()),
            apps: Some(vec!["git".to_string()]),
            repos: Some(vec![Repo {
                name: "".to_string(),
                link: "".to_string(),
                apps: Some(vec!["".to_string()]),
            }]),
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Repo {
    pub name: String,
    pub link: String,
    pub apps: Option<Vec<String>>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DistroInfo{
    pub package_manager:String,
    pub install:String,
    pub uninstall:String
}
impl DistroInfo{
    fn init_new(package_manager:&str,install:&str,uninstall:&str)->DistroInfo{
        DistroInfo{
            package_manager:package_manager.to_string(),install:install.to_string(),
            uninstall:uninstall.to_string()
        }
    }
    pub fn init_default()->HashMap<String,DistroInfo>{
        HashMap::from([
            ("fedora".to_string(),DistroInfo::init_new("dnf","install","uninstall"))
            ,("debian".to_string(),DistroInfo::init_new("apt","install","uninstall"))
            ,("opensuse".to_string(),DistroInfo::init_new("zypper","install","uninstall"))
            ,("windows".to_string(),DistroInfo::init_new("scoop","install","remove"))
        ])
    }
}

pub fn init() -> Config {
    let home_dir = home::home_dir().expect("get home path error");
    let jet_dir = home_dir.join(".jet");
    read_config_file(&jet_dir)
}

fn read_config_file(jet_dir: &PathBuf) -> Config {
    let config_path = jet_dir.join("config.toml");
    println!("file path{:?}",config_path);
    if !config_path.exists() {
        println!("not exist");
        return init_config_file(jet_dir);
    }
    println!(" exist");
    let config = fs::read_to_string(config_path).expect("read config file error");
    println!("{:?}", config);
    let config: Config = toml::from_str(&config).expect("transfer error");
    config
}

fn init_config_file(dir_path: &PathBuf) -> Config {
    if !dir_path.exists() {
        fs::create_dir_all(dir_path).expect("create dir error");
    }
    let config_path = dir_path.join("config.toml");
    let config = Config::init_new();
    let result = toml::to_string(&config).unwrap();
    let mut file = File::create(config_path).expect("create init config file error");
    file.write_all(result.as_bytes())
        .expect("write default config error");
    config
}

fn get_version()->String{
    println!("pkg version : {}", env!("CARGO_PKG_VERSION"));
    env!("CARGO_PKG_VERSION").to_string()
}

#[cfg(test)]
mod test {
    use crate::config::{init_config_file, read_config_file};
    use std::path::{Path, PathBuf};
    #[test]
    fn test_toml() {
        let buf = PathBuf::from("/home/huang/.jet/");
        let config = read_config_file(&buf);
    }
}
