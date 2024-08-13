use std::{env, os::unix::fs::PermissionsExt, path::PathBuf, rc::Rc};

// GitX is git context
pub struct GitX {
    pub gitemoji: PathBuf,
    pub original: PathBuf,

    types: Rc<Vec<String>>,
    emojis: Rc<Vec<String>>,
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
    pub fn exec_git(&self, args: Vec<String>) {
        let gx = GitX::new();
        let status = std::process::Command::new(&gx.original)
            .args(args)
            .status()
            .expect("failed to execute git");
        std::process::exit(status.code().unwrap());
    }
    pub fn exec_commit(&self, args: Vec<String>) {
        // parse -feat,-doc,... from args
        let _args = Vec::new();
        for (i, arg) in args.iter().enumerate() {}

        // case 1: git commit -m "msg"
        if args.contains(&"-m".to_string()) {
            let gx = GitX::new();
            let status = std::process::Command::new(&gx.original)
                .args(args)
                .status()
                .expect("failed to execute git commit");
            std::process::exit(status.code().unwrap());
        }

        // case 2: git commit without -m

        let gx = GitX::new();
        let status = std::process::Command::new(&gx.original)
            .args(args)
            .status()
            .expect("failed to execute git commit");
        std::process::exit(status.code().unwrap());
    }
}
