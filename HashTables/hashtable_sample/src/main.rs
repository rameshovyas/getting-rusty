use std::collections::HashMap;
type Table = HashMap<String, Vec<String>>;

fn show(table: &Table) {
    for (domain, languages) in table {
        println!("Languages used for {} ", domain);
        println!("--------------------------------");
        for language in languages {
            println!("  {}", language);
        }
        println!("--------------------------------");
    }
}

fn sort_languages(table: &mut Table) {
    for (_domain, languages) in table {
        languages.sort();
    }
}

fn main() {
    let mut table = Table::new();
    table.insert(
        "Systems Programming".to_string(),
        vec![
            "Assembly Language".to_string(),
            "C Language".to_string(),
            "Rust Language".to_string(),
        ],
    );

    table.insert(
        "Web Development".to_string(),
        vec![
            "C#".to_string(),
            "Rust Language".to_string(),
            "Java".to_string(),
            "JavaScript".to_string(),
        ],
    );

    show(&table);
    sort_languages(&mut table);
    show(&table);
}
