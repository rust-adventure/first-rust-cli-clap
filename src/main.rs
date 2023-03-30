use std::{fs::File, io::Write};

use camino::Utf8PathBuf;
use clap::{error::ErrorKind, CommandFactory, Parser};
use serde::Serialize;
use slug::slugify;

#[derive(
    clap::ValueEnum, Clone, Default, Debug, Serialize,
)]
#[serde(rename_all = "kebab-case")]
enum Layout {
    /// blog post
    #[default]
    Post,
    /// image gallery
    Gallery,
    /// code example
    Code,
}
#[derive(
    clap::ValueEnum, Clone, Default, Debug, Serialize,
)]
#[serde(rename_all = "kebab-case")]
enum PostStatus {
    /// Draft, don't publish
    #[default]
    Draft,
    /// Needs Review
    NeedsReview,
    /// Publishable
    Publish,
}

#[derive(Serialize)]
struct Frontmatter {
    layout: Layout,
    tags: Vec<String>,
    status: PostStatus,
    title: String,
    slug: String,
}
impl From<Args> for Frontmatter {
    fn from(args: Args) -> Self {
        let slug = if !args.title.is_empty() {
            slug::slugify(&args.title)
        } else {
            let today =
                time::OffsetDateTime::now_local().unwrap();
            let format = time::format_description::parse(
                "[year]-[month]-[day]",
            )
            .unwrap();
            today.format(&format).unwrap()
        };
        Frontmatter {
            layout: args.layout,
            tags: args.tags,
            status: args.status,
            title: args.title,
            slug,
        }
    }
}
/// Scaffold a new post for your blog
#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The layout the post should use
    #[clap(short, long, value_enum, default_value_t)]
    layout: Layout,

    /// Tags to include
    #[clap(short, long = "tag")]
    tags: Vec<String>,
    /// The title of the post.
    ///
    /// If not provided, the filename will be generated
    #[clap(short = 'T', long, default_value = "A Post")]
    title: String,
    /// Should this post be published?
    #[clap(short, long, value_enum, default_value_t)]
    status: PostStatus,
    /// Where to put the file
    #[clap(short, long, default_value_t = Utf8PathBuf::from("content"))]
    output_dir: Utf8PathBuf,
}
fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();

    if !args.output_dir.exists() {
        let mut cmd = Args::command();
        cmd.error(
            ErrorKind::InvalidValue,
            format!(
                "output directory `{}/` does not exist",
                args.output_dir.as_str()
            ),
        )
        .exit();
    }

    let frontmatter = Frontmatter::from(args.clone());
    let new_file_contents = format!(
        "{frontmatter}
---

# {title}

    ",
        frontmatter =
            serde_yaml::to_string(&frontmatter).unwrap(),
        title = args.title
    );
    let filename = format!("{}.md", slugify(args.title));
    let output_file = args.output_dir.join(filename);

    let mut file = File::create(output_file)?;
    file.write_all(new_file_contents.as_bytes())?;

    Ok(())
}
