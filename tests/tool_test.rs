use s4n::tool::{
    cli_tool::Tool,
    input::{Input, OptionType},
    parser::parse_command_line,
};

pub fn test_cases() -> Vec<(String, Tool)> {
    vec![
        (
            "python script.py".to_string(),
            Tool {
                base_command: vec!["python".to_string(), "script.py".to_string()],
                inputs: vec![],
                outputs: vec![],
            },
        ),
        (
            "python script.py --option1 value1".to_string(),
            Tool {
                base_command: vec!["python".to_string(), "script.py".to_string()],
                inputs: vec![Input::new_with(
                    "option1",
                    Some("value1"),
                    OptionType::Option,
                    Some("--option1"),
                    None,
                )],
                outputs: vec![],
            },
        ),
        (
            "python script.py --option1 \"value with spaces\"".to_string(),
            Tool {
                base_command: vec!["python".to_string(), "script.py".to_string()],
                inputs: vec![Input::new_with(
                    "option1",
                    Some("value with spaces"),
                    OptionType::Option,
                    Some("--option1"),
                    None,
                )],
                outputs: vec![],
            },
        ),
        (
            "python script.py positional1 --option1 value1".to_string(),
            Tool {
                base_command: vec!["python".to_string(), "script.py".to_string()],
                inputs: vec![
                    Input::new_with(
                        "positional1",
                        Some("positional1"),
                        OptionType::Positional,
                        None,
                        Some(0),
                    ),
                    Input::new_with(
                        "option1",
                        Some("value1"),
                        OptionType::Option,
                        Some("--option1"),
                        None,
                    ),
                ],
                outputs: vec![],
            },
        ),
        (
            "python script.py --flag1".to_string(),
            Tool {
                base_command: vec!["python".to_string(), "script.py".to_string()],
                inputs: vec![Input::new_with(
                    "flag1",
                    None,
                    OptionType::Flag,
                    Some("--flag1"),
                    None,
                )],
                outputs: vec![],
            },
        ),
        (
            "python script.py -o value1".to_string(),
            Tool {
                base_command: vec!["python".to_string(), "script.py".to_string()],
                inputs: vec![Input::new_with(
                    "o",
                    Some("value1"),
                    OptionType::Option,
                    Some("-o"),
                    None,
                )],
                outputs: vec![],
            },
        ),
        (
            "Rscript script.R".to_string(),
            Tool {
                base_command: vec!["Rscript".to_string(), "script.R".to_string()],
                inputs: vec![],
                outputs: vec![],
            },
        ),
        (
            "echo \"Hello World\"".to_string(),
            Tool {
                base_command: vec!["echo".to_string()],
                inputs: vec![Input::new_with(
                    "hello-world",
                    Some("Hello World"),
                    OptionType::Positional,
                    None,
                    Some(0),
                )],
                outputs: vec![],
            },
        ),
        (
            "ls -la".to_string(),
            Tool {
                base_command: vec!["ls".to_string()],
                inputs: vec![Input::new_with(
                    "la",
                    None,
                    OptionType::Flag,
                    Some("-la"),
                    None,
                )],
                outputs: vec![],
            },
        ),
    ]
}

#[test]
fn test_command_line_parser() {
    for (input, expected) in test_cases() {
        let args = shlex::split(input.as_str()).expect("Parsing test case failed");
        let result = parse_command_line(args.iter().map(|x| x.as_ref()).collect());
        assert_eq!(result, expected);
        println!("{:?}", result);
    }
}

#[test]
fn test_execution() {
    let command = "ls -la";
    let args = shlex::split(command).expect("parsing failed");
    let result = parse_command_line(args.iter().map(|x| x.as_ref()).collect());
    let status = result.execute();
    assert!(status.success())
}