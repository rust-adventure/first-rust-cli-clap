use camino::Utf8PathBuf;
use clap::{error::ErrorKind, CommandFactory, Parser};
use serde::Serialize;
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
    output_dir: Utf8PathBuf,
}
fn main() {
    let args = Args::parse();
    dbg!(&args);

    if !args.output_dir.exists() {
        let mut cmd = Args::command();
        cmd.error(
            ErrorKind::ValueValidation,
            format!(
                "output directory `{}` doesn't exist",
                args.output_dir
            ),
        )
        .exit();
    }

    let post_slug = slug::slugify(&args.title);
    let mut filename = args.output_dir.join(&post_slug);
    filename.set_extension("md");

    let frontmatter = Frontmatter {
        layout: args.layout,
        tags: args.tags,
        status: args.status,
        title: args.title.clone(),
        slug: post_slug,
    };

    let yaml = serde_yaml::to_string(&frontmatter).unwrap();
    let file_contents = format!(
        "{yaml}
---

# {}",
        args.title
    );

    if let Err(error) = fs::write(&filename, file_contents)
    {
        let mut cmd = Args::command();
        cmd.error(
            ErrorKind::Io,
            format!(
                "failed to write file at `{filename}`\n\t{error}",
            ),
        )
        .exit();
    }
}

#[derive(Debug, Serialize)]
struct Frontmatter {
    layout: String,
    tags: Vec<String>,
    status: String,
    title: String,
    slug: String,
}
