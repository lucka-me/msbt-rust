fn main() {
    let arguments = std::env::args().collect::<Vec<_>>();

    if arguments.len() < 2 {
        panic!("The input file is missing.");
    }

    let input_file = std::fs::File::open(std::path::PathBuf::from(arguments[1].clone()))
        .expect(&format!("Unable to open {}", arguments[1]));

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
