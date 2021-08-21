extern crate ferris_says;
extern crate serial_test;

use serial_test::serial;
use ferris_says::{say, think, perform, SpeechModes, Eyes, FerrisConfig, Speaker, set_speaker};

// Default width when running the binary
const DEFAULT_WIDTH: usize = 40;

const SPEECH_BUBBLE: &str = r#"        \
         \"#;
const THOUGHT_BUBBLE: &str = r#"        o
         o"#;

const FERRIS_TOP: &[u8] = br#"
            _~^~^~_
        \) /  "#;
const FERRIS_BOTTOM: &[u8] = br#"  \ (/
          '_   -   _'
          / '-----' \
"#;

const COW_TOP: &[u8] = br#"
            ^__^
            ("#;
        
const COW_BOTTOM: &[u8] = br#"
               )\_______
            (__)\       )\/\
                ||----w |
                ||     ||
"#;

const CLIPPY_TOP: &[u8] = br#"
            __
           /  \
           |  |
           "#;
const CLIPPY_BOTTOM: &[u8] = br#"
           |  |
           || |/
           || ||
           |\_/|
           \___/
"#;

#[test]
#[serial]
fn hello_fellow_rustaceans_width_24() -> Result<(), ()> {
    //Hello fellow Rustaceans!
    let speech = String::from(concat!(
        " __________________________\n",
        "< Hello fellow Rustaceans! >\n",
        " --------------------------\n",
    ));
    let input = b"Hello fellow Rustaceans!";
    let width = 24;
    generic_tests(speech, width, input);
    Ok(())
}

#[test]
#[serial]
fn hello_fellow_rustaceans_width_12() -> Result<(), ()> {
    //Hello fellow Rustaceans!
    let speech = String::from(concat!(
        " ______________\n",
        "/ Hello fellow \\\n",
        "\\ Rustaceans!  /\n",
        " --------------\n"
    ));
    let input = b"Hello fellow Rustaceans!";
    let width = 12;
    generic_tests(speech, width, input);
    Ok(())
}

#[test]
#[serial]
fn hello_fellow_rustaceans_width_6() -> Result<(), ()> {
    let speech = String::from(concat!(
        " ________\n",
        "/ Hello  \\\n",
        "| fellow |\n",
        "| Rustac |\n",
        "\\ eans!  /\n",
        " --------\n"
    ));
    let input = b"Hello fellow Rustaceans!";
    let width = 6;
    generic_tests(speech, width, input);
    Ok(())
}

#[test]
#[serial]
fn hello_fellow_rustaceans_width_3() -> Result<(), ()> {
    //Hello fellow Rustaceans!
    let speech = String::from(concat!(
        " _____\n",
        "/ Hel \\\n",
        "| lo  |\n",
        "| fel |\n",
        "| low |\n",
        "| Rus |\n",
        "| tac |\n",
        "| ean |\n",
        "\\ s!  /\n",
        " -----\n"
    ));

    let input = b"Hello fellow Rustaceans!";
    let width = 3;
    generic_tests(speech, width, input);
    Ok(())
}

#[test]
#[serial]
fn multibyte_string() -> Result<(), ()> {
    //Hello fellow Rustaceans!
    let speech = String::from(concat!(
        " ____________\n",
        "< 突然の死👻 >\n",
        " ------------\n"
    ));
    let input = "突然の死👻";
    let width = DEFAULT_WIDTH;
    generic_tests(speech, width, input.as_bytes());
    Ok(())
}

fn create_ferris(
    speech: String, top_part: &str, eye: &str, eye_gap: &str, bottom_part: &str
) -> (String, String) {
    let expected_say = speech.clone() + SPEECH_BUBBLE + top_part + eye +  eye_gap + eye + bottom_part;
    let expected_think = speech.clone() + THOUGHT_BUBBLE + top_part + eye +  eye_gap + eye + bottom_part;
    (expected_say, expected_think)
}

fn compare_strings_perform(
    input: &[u8], width: usize, expected: &[u8], speaker: Speaker, cfg: &FerrisConfig
) {
    set_speaker(&speaker).unwrap();
    let mut vec = Vec::new();
    perform(input, width, &mut vec, cfg).unwrap();
    let actual = std::str::from_utf8(&vec).unwrap();
    println!("{}", std::str::from_utf8(&expected).unwrap());
    println!("{}", actual);
    assert_eq!(std::str::from_utf8(&expected).unwrap(), actual);
}

fn compare_strings_say_think(
    input: &[u8], width: usize, expected: &[u8],  speaker: Speaker, mode: &SpeechModes,eyes: &Eyes)
{
    set_speaker(&speaker).unwrap();
    let mut vec = Vec::new();
    match mode {
        SpeechModes::Say => {
            say(input, width, &mut vec, eyes).unwrap();
        }
        SpeechModes::Think => {
            think(input, width, &mut vec, eyes).unwrap();
        }
    };
    let actual = std::str::from_utf8(&vec).unwrap();
    println!("{}", std::str::from_utf8(&expected).unwrap());
    println!("{}", actual);
    assert_eq!(std::str::from_utf8(&expected).unwrap(), actual);
}

fn generic_tests(speech: String, width: usize, input: &[u8]) {
    let top_ferris = std::str::from_utf8(FERRIS_TOP).unwrap();
    let bottom_ferris = std::str::from_utf8(FERRIS_BOTTOM).unwrap();
    let top_clippy = std::str::from_utf8(CLIPPY_TOP).unwrap();
    let bottom_clippy = std::str::from_utf8(CLIPPY_BOTTOM).unwrap();
    let (expected_say, expected_think) = create_ferris(
        speech.clone(), top_ferris, "o", " ", bottom_ferris
    );
    let (happy_say, happy_think) = create_ferris(
        speech.clone(), top_ferris, "^", " ", bottom_ferris
    );
    let (expected_say_clippy, expected_think_clippy) = create_ferris(
        speech.clone(), top_clippy, "o", "  ", bottom_clippy
    );
    let (happy_say_clippy, happy_think_clippy) = create_ferris(
        speech.clone(), top_clippy, "^", "  ", bottom_clippy
    );
    let say = FerrisConfig {
        mode: SpeechModes::Say,
        eyes: Eyes::RegularEyes
    };
    let think = FerrisConfig {
        mode: SpeechModes::Think,
        eyes: Eyes::RegularEyes
    };

    compare_strings_perform(input, width, &expected_say.as_bytes(), Speaker::Ferris, &say);
    compare_strings_perform(input, width, &expected_think.as_bytes(), Speaker::Ferris, &think);
    compare_strings_say_think(
        input, width, &happy_say.as_bytes(), Speaker::Ferris, &SpeechModes::Say,
        &Eyes::HappyEyes
    );
    compare_strings_say_think(
        input, width, &happy_think.as_bytes(), Speaker::Ferris, &SpeechModes::Think,
        &Eyes::HappyEyes
    );

    compare_strings_perform(
        input, width, &expected_say_clippy.as_bytes(), Speaker::Clippy, &say)
        ;
    compare_strings_perform(
        input, width, &expected_think_clippy.as_bytes(), Speaker::Clippy, &think
    );
    compare_strings_say_think(
        input, width, &happy_say_clippy.as_bytes(), Speaker::Clippy, &SpeechModes::Say,
        &Eyes::HappyEyes
    );
    compare_strings_say_think(
        input, width, &happy_think_clippy.as_bytes(), Speaker::Clippy, &SpeechModes::Think,
        &Eyes::HappyEyes
    );
}