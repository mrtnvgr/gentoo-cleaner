use nix::unistd::Uid;
use sys_info::linux_os_release;

fn is_gentoo() {
    assert!(
        linux_os_release().unwrap().name() == "Gentoo",
        "You must run this executable only on Gentoo Linux"
    );
}

fn is_root() {
    assert!(
        Uid::effective().is_root(),
        "You must run this executable with root permissions"
    );
}

pub fn check_all() {
    is_gentoo();
    is_root();
}
