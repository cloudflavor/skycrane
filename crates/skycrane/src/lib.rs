use std::env;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Cli {
    #[structopt(short, long, default_value = "~/.config/skycrane", parse(from_str = expand_path))]
    config_path: PathBuf,
}

fn expand_path(path_str: &str) -> PathBuf {
    if path_str.starts_with('~') {
        let home_var = if cfg!(windows) { "USERPROFILE" } else { "HOME" };
        if let Some(home) = env::var_os(home_var) {
            let without_tilde = &path_str.strip_prefix('~').unwrap();
            let normalized_path = without_tilde
                .trim_start_matches('/')
                .trim_start_matches('\\')
                .replace('\\', "/"); // Normalize path separators
            return PathBuf::from(home).join(normalized_path);
        }
    }
    PathBuf::from(path_str)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_expand_path() {
        let home_var = if cfg!(windows) { "USERPROFILE" } else { "HOME" };
        let home = env::var_os(home_var).unwrap();
        let home_str = home.to_str().unwrap();

        assert_eq!(
            expand_path("~/foo/bar"),
            PathBuf::from(format!("{}/foo/bar", home_str))
        );
        assert_eq!(
            expand_path("~\\foo\\bar"),
            PathBuf::from(format!("{}/foo/bar", home_str))
        );
        assert_eq!(
            expand_path("~/foo\\bar"),
            PathBuf::from(format!("{}/foo/bar", home_str))
        );
        assert_eq!(
            expand_path("~\\foo/bar"),
            PathBuf::from(format!("{}/foo/bar", home_str))
        );
        assert_eq!(
            expand_path("~/.config/skycrane"),
            PathBuf::from(format!("{}/.config/skycrane", home_str))
        );
        assert_eq!(
            expand_path("~/.config\\skycrane"),
            PathBuf::from(format!("{}/.config/skycrane", home_str))
        );
    }
}
