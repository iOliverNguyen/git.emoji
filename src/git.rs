use std::{env, os::unix::fs::PermissionsExt, path::PathBuf};

// GitX is git context
pub struct GitX {
    pub gitemoji: PathBuf,
    pub original: PathBuf,
}

impl GitX {
    pub fn new() -> GitX {
        // the abs path of gitemoji
        let gitemoji = env::args().nth(0).unwrap();
        let gitemoji = std::fs::canonicalize(gitemoji).unwrap();

        // loop through $PATH to find the original git
        let path = env::var("PATH").expect("can not find original git: PATH not set");
        let original = path
            .split(':')
            .find_map(|dir| {
                let git = PathBuf::from(dir).join("git");
                if git != gitemoji
                    && git.is_file()
                    && git.metadata().unwrap().permissions().mode() & 0o111 != 0
                {
                    Some(git)
                } else {
                    None
                }
            })
            .expect("can not find original git: git not in PATH");

        GitX { gitemoji, original }
    }
}
