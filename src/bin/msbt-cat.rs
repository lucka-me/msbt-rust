use clap::Parser;

#[derive(Debug, clap::Parser)]
#[command(author, version, about)]
struct Arguments {
    #[arg(
        num_args(1..),
        value_name = "filename",
        value_hint = clap::ValueHint::AnyPath,
        help = "Path of MSBT file"
    )]
    input_path: std::path::PathBuf,
}

fn main() {
    let arguments = Arguments::parse();

    let input_file = std::fs::File::open(arguments.input_path).expect("Unable to open the input file");

    let message = msbt::Message::from_file(input_file).expect("Unable to parse message");

    let labels = message
        .label_section
        .expect("Label section is not available")
        .labels;
    let texts = message
        .text_section
        .expect("Text section is not available")
        .texts;

    let mut localizations =
        std::collections::BTreeMap::<std::string::String, std::string::String>::new();
    for label_entries in labels {
        for label in label_entries {
            if label.index < texts.len() {
                localizations.insert(label.name, texts[label.index].clone());
            } else {
                eprintln!(
                    "Index out of range: The index of {} is {} but only {} items",
                    label.name, label.index, texts.len()
                );
            }
        }
    }
    for (name, text) in localizations {
        println!("{name}: {text}");
    }
}
