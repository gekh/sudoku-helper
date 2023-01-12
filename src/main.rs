use array_tool::vec::Intersect;
use std::env;

#[derive(Debug, Clone)]
struct Args {
    number: i32,
    size: i32,
    required: Vec<i32>,
    excluded: Vec<i32>,
}

impl Args {
    pub fn parse() -> Self {
        let mut required = vec![];
        let mut excluded = vec![];

        let mut args = env::args().skip(1);

        let number = args
            .next()
            .expect("There should be at least two args for number and size.")
            .parse::<i32>()
            .expect("Should be a number.");

        let size = args
            .next()
            .expect("There should be at least two args for number and size.")
            .parse::<i32>()
            .expect("Should be a number.");

        while let Some(arg) = args.next() {
            match &arg[..] {
                "-r" | "--require" => {
                    if let Some(arg_value) = args.next() {
                        required = arg_value
                            .split(',')
                            .map(|x| x.parse::<i32>().unwrap())
                            .collect::<Vec<i32>>();
                    } else {
                        panic!("No value specified for the parameter required.")
                    }
                }
                "-e" | "--exclude" => {
                    if let Some(arg_value) = args.next() {
                        excluded = arg_value
                            .split(',')
                            .map(|x| x.parse::<i32>().unwrap())
                            .collect::<Vec<i32>>();
                    } else {
                        panic!("No value specified for the parameter required.")
                    }
                }
                _ => (),
            }
        }

        Args {
            number,
            size,
            required,
            excluded,
        }
    }
}

fn main() {
    let args = Args::parse();

    print_explanation_message(&args);

    for s in find(&args) {
        println!("{:?}", s.parse::<u128>().unwrap());
    }
}

fn print_explanation_message(args: &Args) {
    print!(
        "These are groups by {} to sum up to {}",
        args.size, args.number
    );
    if args.excluded.len() > 0 {
        print!(", excluding {:?}", args.excluded);
    }
    if args.required.len() > 0 {
        print!(", requiring {:?}", args.required);
    }
    println!();
}

fn find(args: &Args) -> Vec<String> {
    let mut result = vec![];
    let mut window = vec![];

    req(
        args,
        args.number.clone(),
        args.size.clone(),
        &mut window,
        &mut result,
    );

    let mut result = result
        .iter()
        .map(|row| {
            row.iter()
                .map(|&digit| digit.to_string())
                .collect::<String>()
        })
        .collect::<Vec<String>>();

    result.sort_unstable();
    result.dedup();

    result
}

fn req(
    args: &Args,
    n: i32,
    places: i32,
    mut window: &mut Vec<i32>,
    mut result: &mut Vec<Vec<i32>>,
) {
    if places == 0 {
        if is_succeful(window, args) {
            let mut check = window.clone();
            check.sort();
            result.push(check);
        }
        return;
    }

    for i in 1..10 {
        if n - i >= 0 && !window.contains(&i) {
            window.push(i);
            req(args, n - i, places - 1, &mut window, &mut result);
            window.pop();
        }
    }
}

fn is_succeful(window: &mut Vec<i32>, args: &Args) -> bool {
    window.iter().sum::<i32>() == args.number
        && window.intersect(args.excluded.clone()).len() == 0
        && (args.required.len() == 0
            || window.intersect(args.required.clone()).len() == args.required.len())
}
