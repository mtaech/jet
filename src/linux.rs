use std::error::Error;
use tracing::log::info;
use crate::config::Config;

pub fn app_install(config: Config)->Result<(),Box<dyn Error>> {
    let profile = config.info.profile;
    let map = config.profiles.expect("get profiles error");
    let profile = map.get(&profile).expect("get profile error");
    let apps = &profile.apps;
    let distro_info = &config.distro_info.expect("get distro info error");
    let distro = distro_info.get(&profile.distro).expect("get distro info");
    if apps.is_some() {
        let app_vec: &Vec<String> = apps.as_ref().unwrap();
        let shell = format!("sudo {} {} {}",distro.package_manager,distro.install, app_vec.join(" \
        "));
        info!("start exec install script:");
        println!("bash -c {:?}",&shell);
        cmd_lib::run_cmd!(bash -c $shell)?;
    }
    Ok(())
}

pub fn app_uninstall(config: Config) {
    todo!()
}

pub fn font(config: Config) {
    todo!()
}

pub fn repo_add(config: Config) {
    todo!()
}

pub fn repo_delete(config: Config) {
    todo!()
}

pub fn flatpak(config: Config) {
    todo!()
}

pub fn app(config: Config) {
    todo!()
}

pub fn repo(config: Config) {
    todo!()
}
