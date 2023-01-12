fn main() {
    for s in find(12, 4) {
        println!("{:?}", s.parse::<u128>().unwrap());
    }
}

/** SUDOKU HELPER */
fn find(n: i32, places: i32) -> Vec<String> {
    let mut result = vec![];
    let mut window = vec![];

    req(n, n, places, &mut window, &mut result);
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

fn req(total: i32, n: i32, places: i32, mut window: &mut Vec<i32>, mut result: &mut Vec<Vec<i32>>) {
    if places == 0 {
        if window.iter().sum::<i32>() == total
        && window.contains(&1)
        // && !window.contains(&8)
        // && !window.contains(&9)
        // && !window.contains(&3)
        {
            let mut check = window.clone();
            check.sort();
            result.push(check);
        }
        return;
    }

    for i in 1..10 {
        if n - i >= 0 && !window.contains(&i) {
            window.push(i);
            req(total, n - i, places - 1, &mut window, &mut result);
            window.pop();
        }
    }
}
