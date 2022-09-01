use std::process;
use docopt::Docopt;
use serde::Deserialize;

mod caesar_suite;

const USAGE: &'static str = "Caesar Cipher Tool.

Usage:
    caesar-cipher (encrypt | decrypt) [--key <int>] (--input-text <string>)
    caesar-cipher brute-force (--dictionary <path>) (--input-text <string>)
    caesar-cipher [--help]

Options:
    -h, --help                          Show this help message and exit.
    -k <int>, --key <int>               Rotation key value as an integer between 1 and 25 [default: 13].
    -i <string>, --input-text <string>  Input text to be operated on. Must contain only English letters and spaces.
    -d <path>, --dictionary <path>      Specify a path to a text file containing valid English words. All non-English or non-alpha characters will be stripped.";

#[derive(Debug, Deserialize)]
struct Args {
    cmd_encrypt: bool,
    cmd_decrypt: bool,
    cmd_brute_force: bool,
    flag_key: usize,
    flag_input_text: String,
    flag_dictionary: String,
}

fn validate_text(string: &String) -> Result<(), &'static str> {
    if !string.chars().all( |c| c.is_ascii_alphabetic() || c.is_ascii_whitespace()){
        Err("The text you provided contained non-ascii-alphabetic characters.")
    } else {
        Ok(())
    }
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| {
            println!("Please refer to the documentation for correct usage.");
            println!("The following error occurred: ");
            e.exit()
        });

    if args.cmd_encrypt {

        validate_text(&args.flag_input_text).unwrap_or_else(|err| {
            println!("{err}");
            process::exit(1);
        });

        println!("{}", caesar_suite::encrypt(args.flag_key as u8, &args.flag_input_text));

    } else if args.cmd_decrypt {

        validate_text(&args.flag_input_text).unwrap_or_else(|err| {
            println!("{err}");
            process::exit(1);
        });

        println!("{}", caesar_suite::decrypt(args.flag_key as u8, &args.flag_input_text));

    } else if args.cmd_brute_force {

        validate_text(&args.flag_input_text).unwrap_or_else(|err| {
            println!("{err}");
            process::exit(1);
        });

        let solutions =
            caesar_suite::brute_force(&args.flag_dictionary, &args.flag_input_text);

        println!("The following were found as potential solutions:\n");
        for solution in solutions {
            println!("Key: {}", solution.0);
            println!("Plain text: {}", solution.1);
            println!();
        }

    } else {
        println!("{USAGE}");
    }


}
