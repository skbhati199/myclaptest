use clap::{command, Arg, ArgMatches, Command};

fn main() {
    let match_result: ArgMatches = command!()
        .subcommand(
            Command::new("register-person")
                .arg(
                    Arg::new("firstname")
                        .short('f')
                        .long("first-name")
                        .aliases(["fname", "firstname"])
                        .required(true)
                        .help("This person's first name"),
                )
                .arg(
                    Arg::new("lastname")
                        .short('l')
                        .long("last-name")
                        .aliases(["lname", "lastname"])
                        .required(true)
                        .help("This person's last name"),
                ),
        )
        .subcommand(
            Command::new("register-pet")
                .arg(Arg::new("pet-name").long("pet-name").required(true))
                .about("This application register people with doctor's office"), // .conflicts_with("lastname")
        )
        .arg(
            Arg::new("fluffy")
                .long("fluffy")
                .help("Is the person wearing a fluffy coat 🧥 "),
        )
        .get_matches();

    // println!("{}", match_result.get_one::<String>("pet-name").unwrap_or(&"Not Pet name".to_string()))
    // let pet_args: Option<&ArgMatches> = match_result.subcommand_matches("register");
    // println!("Does pet name exists? {}", pet_args.unwrap().get_one::<String>("pet-name").unwrap());

    let person_args : &ArgMatches = match_result.subcommand_matches("register-person").unwrap();
    let fname: &String = person_args.get_one::<String>("firstname").unwrap();
    let lname: &String = person_args.get_one::<String>("lastname").unwrap();
    println!("First Name : {} Last name: {}", fname, lname);


}
