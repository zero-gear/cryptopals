use clap::{load_yaml, App};
mod hex2base64;


fn main() {
    let yaml = load_yaml!("cli.yaml");
    #[warn(unused_variables)]
    let matches = App::from(yaml).get_matches();

    if let Some(challenge) = matches.value_of("challenge") {
        match challenge {
            "1.1" => {
                println!("Challenge #{} 'Convert hex to base64'", challenge);
                if let Some(input) = matches.value_of("INPUT") {
                    println!("\tLet's handle next input: `{}`", input);
                    hex2base64::encode();
                }
            }
            _ => println!("Rest cases"),
        }
    }
}
