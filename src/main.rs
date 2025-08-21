use clap::Parser;
use std::collections::HashMap;
use std::fs;
use serde_json;
use std::io;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    text: String,

    #[arg(short, long)]
    capitalize: bool,

    #[arg(short, long)]
    lowercase: bool,

    #[arg(short, long)]
    tobinary: bool,

    #[arg(short, long)]
    reverse: bool,

    #[arg(short, long)]
    frombinary: bool,

    #[arg(short, long)]
    fromfile: bool,

    #[arg(short, long)]
    tofile: bool,

    #[arg(short, long, required_if_eq("tofile", "true"))]
    filepath: Option<String>,

    #[arg(long)]
    keepcontents: bool,

    #[arg(short, long)]
    tomorse: bool,

    #[arg(short, long)]
    frommorse: bool,

    #[arg(short, long, required_if_eq("to_morse", "true"), required_if_eq("from_morse", "true"))]
    morselanguage: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let data;

    let filepath;

    let morse_language = args.morselanguage;

    match morse_language.as_deref() {
        Some(lang) if lang == "en" => data = fs::read_to_string("morse_en.json")?,
        // Some(lang) if lang == "ru" => data = fs::read_to_string("morse_ru.json")?,
        // TODO ^^
        _ => data = fs::read_to_string("morse_en.json")?,
    }


    let mut output: String = args.text.clone();

    if args.capitalize {
        output = output.to_uppercase();
    }

    if args.lowercase {
        output = output.to_lowercase();
    }

    if args.reverse {
        output = reverse(&output);
    }

    if args.tobinary {
        output = to_binary(&output);
    }

    if args.frombinary {
        output = from_binary(&output);
    }

    if args.tomorse {
        output = to_morse(&output, data.clone())?;
    }

    if args.frommorse {
        output = from_morse(&output, data.clone())?;
    }

    if args.fromfile {
        output = from_file(&output)?;
    }

    if args.tofile {
        filepath = args.filepath.clone();
        output = to_file(filepath.as_deref().unwrap(), &output, args.keepcontents)?;
    }

    println!("{}", output);

    Ok(())
}

fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

fn to_binary(s: &str) -> String {
    s.bytes()
        .map(|b| format!("{:08b}", b))
        .collect::<Vec<_>>()
        .join(" ")
}

fn from_binary(s: &str) -> String {
    s.split_whitespace()
        .filter_map(|b| u8::from_str_radix(b, 2).ok()) // binary -> number
        .map(|num| num as char) // number -> char
        .collect()
}

fn to_morse(s: &str, data: String) -> Result<String, Box<dyn std::error::Error>> {
    let morse_map: HashMap<String, String> = serde_json::from_str(&data)?;

    let mut morse_result = Vec::new();
    for c in s.to_uppercase().chars() {
        if c == ' ' {
            morse_result.push(String::from("/"))
        } else if let Some(code) = morse_map.get(&c.to_string()) {
            morse_result.push(code.clone());
        } else {
            morse_result.push("?".to_string());
        }
    }

    Ok(morse_result.join(" "))
}

fn from_morse(s: &str, data: String) -> Result<String, Box<dyn std::error::Error>> {
    let morse_map: HashMap<String, String> = serde_json::from_str(&data)?;

    let mut morse_result = Vec::new();

    let codes = s.split_whitespace();

    for code in codes {
        for (key, value) in &morse_map {
            if value == code {
                morse_result.push(String::from(key));
            }
        }
    }

    Ok(morse_result.join(" "))
}

fn from_file(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(filepath)?;

    println!("{}", contents);

    Ok(contents)
}

fn to_file(filepath: &str, text: &str, keep: bool) -> Result<String, Box<dyn std::error::Error>> {
    let mut result = String::new();

    let contents = fs::read_to_string(filepath)?;

    if !keep {
        if contents != "".to_string() {
            let mut input = String::new();

            println!("{} will be OVERWRITTEN. Are you sure? (YES)", filepath);
            io::stdin()
                .read_line(&mut input)
                .expect("Fail");

            let input = input.trim();

            if input == "YES" {
                fs::write(filepath, text)?;
                result = format!("Done: {filepath}")
            }
        } else {
            fs::write(filepath, text)?;
            result = format!("Done: {filepath}")
        }
    } else {
        let contents = fs::read_to_string(filepath)?;
        fs::write(filepath, contents + text)?;

        result = format!("Done: {filepath}")
    }

    Ok(result)
}