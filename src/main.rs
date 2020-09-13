extern crate clap;
extern crate colored;
extern crate pest;
#[macro_use]
extern crate pest_derive;

use clap::App;
use colored::*;
use pest::iterators::Pair;
use pest::Parser;
use std::fs;

#[derive(Parser)]
#[grammar = "makefile.pest"]
struct MakefileParser;

#[derive(Debug)]
struct Targets {
    targets: Vec<TargetWithHelpMessage>,
}

#[derive(Debug, Clone)]
struct TargetWithHelpMessage {
    target_name: String,
    help_messages: Vec<String>,
}

const INDENT_WIDTH: usize = 4;
const DEFAULT_DESCRIPTION: &str = "This project uses Make for commands.";

fn main() {
    let matches = App::new("Pretty Make")
        .version("1.0")
        .author("David A. <aweaoftheworld@gmail.com>")
        .about("Make make pretty")
        .arg("<makefile> 'Makefile to be pretty'")
        .get_matches();

    let makefile = matches.value_of("makefile").unwrap();
    let unparsed_file = fs::read_to_string(makefile).expect("cannot read file");
    let file =
        MakefileParser::parse(Rule::makefile, &unparsed_file).unwrap_or_else(|e| panic!("{}", e));
    let mut targets = Targets {
        targets: Vec::new(),
    };
    let mut project_name: String = "".to_string();
    let mut project_description: String = "".to_string();

    for record in file {
        match record.as_rule() {
            Rule::target_with_help => {
                let target_name = get_text(&record, Rule::target_name);
                let help_messages = record
                    .clone()
                    .into_inner()
                    .filter(|x| x.as_rule() == Rule::text)
                    .map(|x| {
                        let links: Vec<String> = x
                            .clone()
                            .into_inner()
                            .filter(|x| x.as_rule() == Rule::link)
                            .map(|x| String::from(x.as_str()))
                            .collect();

                        let mut result = String::from(x.as_str());

                        for link in links.iter() {
                            let colored_link = format!("{}", link.truecolor(119, 168, 217));

                            result = result.replace(link, &colored_link);
                        }

                        result
                    })
                    .collect();

                let target_with_help_messages = TargetWithHelpMessage {
                    target_name,
                    help_messages,
                };
                targets.targets.push(target_with_help_messages)
            }
            Rule::name => {
                project_name = get_text(&record, Rule::text);
            }
            Rule::description => {
                project_description = get_text(&record, Rule::text);
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    let help_message_offset = help_message_offset(&targets);

    // primary:
    // - #a6cc70
    //  - `RGB::new(166, 204, 112)`
    // - bold
    // title:
    // - #ffcc66
    //  - `RGB::new(255, 204, 102)`
    // link:
    // - #77a8d9
    //  - `RGB::new(119,168,217)`

    println!("{}", project_name.truecolor(166, 204, 112).bold());
    if project_description.len() > 0 {
        println!("{} {} \n", project_description, DEFAULT_DESCRIPTION);
    } else {
        println!("{} \n", DEFAULT_DESCRIPTION);
    }
    println!("{}", "USAGE".truecolor(255, 204, 102));
    println!("    {}\n", "make <SUBCOMMAND>");
    println!("{}", "SUBCOMMANDS".truecolor(255, 204, 102));

    for target in targets.targets {
        print!("{: <1$}", "", INDENT_WIDTH);
        print!(
            "{target_name: <col$}",
            target_name = target.target_name.truecolor(166, 204, 112).bold(),
            col = help_message_offset
        );

        let mut i = 0;
        for help_message in target.help_messages {
            if i > 0 {
                print!("{: <1$}", "", INDENT_WIDTH);
                print!("{: <1$}", "", help_message_offset);
            }
            println!("{}", help_message);

            i = i + 1;
        }
    }
}

fn get_text(record: &Pair<Rule>, rule_type: Rule) -> String {
    let line = record
        .clone()
        .into_inner()
        .find(|x| x.as_rule() == rule_type)
        .unwrap();

    String::from(line.as_str())
}

fn help_message_offset(targets: &Targets) -> usize {
    let longest_target_name =
        targets
            .targets
            .iter()
            .fold(targets.targets[0].clone(), |acc, item| {
                if item.target_name.len() > acc.target_name.len() {
                    item.clone()
                } else {
                    acc
                }
            });

    longest_target_name.target_name.len() + INDENT_WIDTH
}
