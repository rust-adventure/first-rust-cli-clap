use clap::{error::ErrorKind, CommandFactory, Parser};
use std::fs;

/// Scaffold a new post for your blog
#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    /// The layout the post should use
    #[clap(short, long, default_value = "post")]
    layout: String,

    /// Tags to include
    #[clap(short, long = "tag")]
    tags: Vec<String>,

    /// The title of the post.
    ///
    /// If not provided, the filename will be generated
    #[clap(short = 'T', long, default_value = "A Post")]
    title: String,

    /// Should this post be published?
    #[clap(short, long, default_value = "draft")]
    status: String,

    /// Where to put the file
    #[clap(short, long, default_value = "content")]
    output_dir: String,
}
fn main() {
    let args = Args::parse();
    dbg!(&args);
    let filename =
        format!("{}/{}.md", args.output_dir, args.title);

    if let Err(error) = fs::write(&filename, args.title) {
        let mut cmd = Args::command();
        cmd.error(
            ErrorKind::Io,
            format!(
                "failed to write file at `{filename}`\n\t{}",
                error
            ),
        )
        .exit();
    }
}
