use std::process;

fn main() {
    let args = sn::get_args();
    let program = args[0].clone();
    let arguments = sn::Arguments::new(&args).unwrap_or_else(|err| {
        if err.contains("help") {
            process::exit(0);
        } else {
            eprintln!("{} problem parsing arguments: {}", program, err);
            process::exit(0);
        }
    });

    sn::run(&arguments);
}
