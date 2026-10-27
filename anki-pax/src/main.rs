fn main() {
    let (executable, args) = {
        let mut args = std::env::args();
        let executable = args.next().unwrap();

        (executable, args)
    };

    if args.len() == 0 {
        eprintln!("Usage: {executable} <file.apkg>");
    }

    for arg in args {
        let path = std::path::PathBuf::from(arg);
        let file = std::fs::File::open(&path).unwrap();
        let data = anki_pax::extract_archive(file).unwrap();

        std::fs::write(path.with_extension("sqlite"), &data).unwrap();
    }
}
