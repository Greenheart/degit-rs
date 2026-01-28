use clap::{ArgAction, Parser};

/// Download the contents of a git repository without cloning it.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The source repo you want to download.
    #[arg(
        long_help = "The repository you want to download. This can take any of the following forms:

GitHub:
    user/repo
    github:user/repo
    https://github.com/user/repo

GitLab:
    gitlab:user/repo
    https://gitlab.com/user/repo

BitBucket:
    bitbucket:user/repo
    https://bitbucket.org/user/repo

You can clone a specific subdirectory instead of the entire repository:
    user/repo/subdirectory

And you can specify a branch (defaults to HEAD), tag, or commit from any of the above forms using:
    user/repo#branch
    user/repo#v1.0.0
    user/repo#abcd1234

"
    )]
    src: String,

    /// The destination directory. This is where the contents of the repository will be downloaded.
    #[arg(default_value_t = String::from("."))]
    dest: String,

    /// Controls how verbose the log output is.
    #[arg(short, action = ArgAction::Count)]
    v: u8,
}

fn main() {
    let args = Args::parse();

    let src = degit_rs::parse_src(&args.src).unwrap();
    let dest = degit_rs::parse_dest(&args.dest).unwrap();

    degit_rs::degit(src, dest);
}
