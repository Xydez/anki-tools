use argh::FromArgs;

mod util;
mod compare;
mod database;

#[derive(FromArgs, PartialEq, Debug)]
#[argh(
    description = "{command_name} is a tool to perform operation on .apkg files"
)]
struct TopLevel {
    #[argh(subcommand)]
    nested: SubcommandEnum,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum SubcommandEnum {
    Extract(SubcommandExtract),
    Compare(SubcommandCompare),
}
#[derive(FromArgs, PartialEq, Debug)]
/// First subcommand.
#[argh(subcommand, name = "extract", short = 'x')]
struct SubcommandExtract {
    #[argh(positional)]
    file: String,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Second subcommand.
#[argh(subcommand, name = "diff", short = 'd')]
struct SubcommandCompare {
    #[argh(positional, description = "left .apkg file")]
    left: String,
    #[argh(positional, description = "right .apkg file")]
    right: String,
}

fn main() {
    let args: TopLevel = argh::from_env();

    match args.nested {
        SubcommandEnum::Extract(SubcommandExtract { file }) => {
            let path = std::path::PathBuf::from(file);
            let file = std::fs::File::open(&path).unwrap();
            let data = anki_pax::extract_archive(file).unwrap();

            std::fs::write(path.with_extension("sqlite"), &data).unwrap();
        }
        SubcommandEnum::Compare(SubcommandCompare { left, right }) => {
            crate::compare::run_compare_files(&left, &right);
        }
    }
}

