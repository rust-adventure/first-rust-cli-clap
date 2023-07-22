use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long, default_value = "post")]
    layout: String,

    #[clap(short, long = "tag")]
    tags: Vec<String>,

    #[clap(short = 'T', long, default_value = "A Post")]
    title: String,

    #[clap(short, long, default_value = "draft")]
    status: String,

    #[clap(short, long, default_value = "content")]
    output_dir: String,
}
fn main() {
    let args = Args::parse();
    dbg!(args);
}
