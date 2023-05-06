use std::path::Path;
use strum::EnumIter;

#[allow(unused_imports)]
use uname::Info;

#[derive(EnumIter)]
pub enum Folder {
    Kernels,
    KernelModules,
    KernelSrcFolders,
    PortageCache,
    PortagePackagesSrc,
    GenkernelCache,
}

impl Folder {
    pub fn folder_path(&self) -> &Path {
        let path = match self {
            Self::Kernels => "/boot",
            Self::KernelModules => "/lib/modules",
            Self::KernelSrcFolders => "/usr/src",
            Self::PortageCache => "/var/tmp/portage",
            Self::PortagePackagesSrc => "/var/cache/distfiles",
            Self::GenkernelCache => "/var/tmp/genkernel",
        };
        Path::new(path)
    }

    pub fn is_cache_file(&self, file: &Path) -> bool {
        match self {
            Self::Kernels => is_old_kernel(file, true),
            Self::KernelModules => is_old_kernel(file, false),
            Self::KernelSrcFolders => is_old_kernel(file, false) && !file.is_symlink(),
            Self::PortageCache | Self::PortagePackagesSrc | Self::GenkernelCache => true,
        }
    }

    pub const fn pretty_name(&self) -> &str {
        match self {
            Self::Kernels => "kernels",
            Self::KernelModules => "kernel modules",
            Self::KernelSrcFolders => "kernel sources",
            Self::PortageCache => "portage cache",
            Self::PortagePackagesSrc => "portage packages source code",
            Self::GenkernelCache => "genkernel cache",
        }
    }

    pub fn exists(&self) -> bool {
        self.folder_path().exists()
    }
}

#[cfg(not(test))]
fn get_kernel_version() -> String {
    let kernel_name = Info::new().unwrap().release;
    kernel_name.split('-').last().unwrap().to_owned()
}

fn is_old_kernel(file: &Path, check_prefixes: bool) -> bool {
    let valid_kernel_prefixes = vec!["config-", "initramfs-", "vmlinuz-", "System.map-"];

    let path = file.to_string_lossy();

    let is_current_kernel = path.contains(&get_kernel_version());
    let contains_valid_prefix = valid_kernel_prefixes
        .iter()
        .any(|prefix| path.contains(prefix));

    if check_prefixes {
        !is_current_kernel && contains_valid_prefix
    } else {
        !is_current_kernel
    }
}

#[cfg(test)]
fn get_kernel_version() -> String {
    "1.0.1-gentoo-dist".to_owned()
}

#[cfg(test)]
mod tests {
    use crate::folder::is_old_kernel;
    use std::path::Path;

    #[test]
    fn test_kernels_folder() {
        assert!(is_old_kernel(Path::new("config-1.0.0-gentoo-dist"), true));
        assert!(!is_old_kernel(Path::new("config-1.0.1-gentoo-dist"), true));
        assert!(is_old_kernel(Path::new("config-1.0.0-gentoo-dist"), false));
        assert!(is_old_kernel(Path::new("/boot/1.0.0-gentoo-dist"), false));
        assert!(!is_old_kernel(Path::new("/boot/grub"), true));
        assert!(!is_old_kernel(Path::new("/boot/linux"), true));
    }

    #[test]
    fn test_kernel_modules_folder() {
        assert!(is_old_kernel(Path::new("1.0.0-gentoo-dist"), false));
        assert!(!is_old_kernel(Path::new("1.0.1-gentoo-dist"), false));
    }

    #[test]
    fn test_kernel_sources_folder() {
        assert!(is_old_kernel(Path::new("linux-1.0.0-gentoo-dist"), false));
        assert!(!is_old_kernel(Path::new("linux-1.0.1-gentoo-dist"), false));
    }
}
