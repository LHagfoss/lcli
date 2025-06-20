use owo_colors::OwoColorize;
use tabled::{
    builder::Builder,
    settings::{
        object::{Columns, Rows},
        Alignment, Modify, Padding, Panel, Style,
    },
};

pub fn handle_about_command() {
    let version = format!("Version: {}", env!("CARGO_PKG_VERSION"));
    let author = format!("Author: {}", env!("CARGO_PKG_AUTHORS"));
    let website = "Website: https://lhagfoss.com";
    let github = "Github: https://github.com/lhagfoss";

    let content = format!(
        "{}\n{}\n\n{}\n{}",
        version, author, website, github
    );

    print_boxed_content("Lagos Command Line Interface - lcli", &content);
}

fn print_boxed_content(title: &str, content: &str) {
    let mut builder = Builder::default();
    builder.push_record([content]);

    let mut table = builder.build();
    table
        .with(Style::rounded())
        .with(Panel::header(title.cyan().bold().to_string()))
        .with(Modify::new(Rows::first()).with(Padding::new(1, 1, 1, 1)))
        .with(Modify::new(Columns::first()).with(Alignment::left()));

    println!("\n{}\n", table);
}