use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();
    let content = std::fs::read_to_string("test.txt").unwrap();

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line)
        }
    }

    let result = std::fs::read_to_string("test.txt");
    let content = match result {
        Ok(content) => { content },
        Err(error) => { panic!("Can't deal with {}, just exit here", error); }
    };
    println!("file content: {}", content);
}
