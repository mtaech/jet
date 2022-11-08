extern crate sys_info;

enum Package {
    Fedora{
        manager:String ,
    },
    APT,
    ZYPPER,
    SCOOP,
}

#[cfg(target_os = "linux")]
pub fn detect_distro() -> String {
    let info = sys_info::linux_os_release().expect("get os release error");
    let id = info.id.expect("get os name error");
    id
}

#[cfg(test)]
mod test {
    use crate::package::detect_distro;

    #[test]
    fn test() {
        detect_distro();
    }
}
