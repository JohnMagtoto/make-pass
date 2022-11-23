use clap::{Parser, ValueEnum};
// use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

/// This is the string used to check if a glyph is lowercase
const LOWERCASES: &str = "a b c d e f g h i j k l m n o p q r s t u v w x y z";
/// This is the string used to check if a glyph is uppercase
const UPPERCASES: &str = "A B C D E F G H U J K L M N O P Q R S T U V W X Y Z";
/// This is the string used to check if a glyph qualifies as a symbol
const SYMBOLS: &str = "! \" ; # $ % & ' ( ) * + , - . / : ; < = > ? @ [ ] ^ _ ` { | } ~";
/// This is the string used to check if a glyph qualifies as a number
const NUMBERS: &str = "1 2 3 4 5 6 7 8 9 0";
/// This is the string used to check if a glyph is one which could be mistaken as another
const SIMILAR_CHARS: &str = "i l o I O 1 0 | ' ` \"";

#[derive(ValueEnum, Clone, Debug)]
enum Length {
    Short,
    Long,
}

fn generate_dictionary(no_similar: bool) -> String {
    let mut dictionary = [LOWERCASES, UPPERCASES, NUMBERS, SYMBOLS].join(" ");

    if no_similar == true {
        dictionary = dictionary.replace(SIMILAR_CHARS, "");
    }

    return dictionary;
}

fn generate_character(dictionary: &String) -> String {
    // Collect valid password characters
    let dict: Vec<&str> = dictionary.split_whitespace().collect();
    // Get random character
    let rand = thread_rng().gen_range(0..dict.len());
    dict[rand].to_owned()
}

fn generate_password(pass_len: usize, no_similar: bool) -> String {
    let mut pass_string = String::new();

    let pass_dict = generate_dictionary(no_similar);

    for _ in 0..pass_len {
        pass_string.push_str(&generate_character(&pass_dict))
    }

    return pass_string;
}

/// Command line arguments
#[derive(Parser, Debug)]
#[command(author="John Magtoto", version="0.0.1", about="A password generator", long_about = None)]
struct Args {
    /// Password size
    #[arg(short, long, value_enum, default_value_t=Length::Long)]
    length: Length,
    /// No similar characters allowed
    #[arg(short, long, default_value_t = false)]
    no_similar: bool,
}

fn main() {
    let arguments = Args::parse();

    const SHORT: usize = 8;

    const LONG: usize = 16;

    let pass_len: usize;

    if matches!(arguments.length, Length::Short) {
        pass_len = SHORT
    } else {
        pass_len = LONG
    }

    // let rand_string: String = thread_rng()
    //     .sample_iter(&Alphanumeric)
    //     .take(pass_len)
    //     .map(char::from)
    //     .collect();

    let rand_string = generate_password(pass_len, arguments.no_similar);

    println!("Generated password is:");
    println!("{}", rand_string);
}
