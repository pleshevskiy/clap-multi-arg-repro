use clap::{Arg, ArgAction, Command};

fn main() {
    let matches = args().get_matches();

    println!(
        "input: {:?}",
        matches
            .get_many::<String>("input")
            .unwrap()
            .into_iter()
            .map(String::from)
            .collect::<Vec<_>>()
    );
    println!("output: {:?}", matches.get_one::<String>("output").unwrap());
    println!("one: {:?}", matches.get_one::<String>("one"));
    println!("two: {:?}", matches.get_one::<String>("two"));
    println!("yes: {:?}", matches.get_one::<bool>("yes"));
}

fn args<'a>() -> clap::App<'a> {
    Command::new("Clap test")
        .version("1.0")
        .author("Dmitriy Pleshevskiy <dmitriy@ideascup.me>")
        .about("Error reproduction with multiple parameters")
        .arg(Arg::new("yes").long("yes").action(ArgAction::SetTrue))
        .arg(Arg::new("one").long("one").action(ArgAction::Set))
        .arg(Arg::new("two").long("two").action(ArgAction::Set))
        .arg(Arg::new("input").multiple_values(true).required(true))
        .arg(Arg::new("output").required(true))
}
