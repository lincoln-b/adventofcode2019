use std::time::Instant;

fn main() {
    let now = Instant::now();

    let lower = 357253;
    let upper = 892942;

    let mut num = 0;
    let mut num2 = 0;
    for i in lower..upper {
        if meets_criteria(i) {
            num += 1;
        }
        if meets_criteria_2(i) {
            num2 += 1;
        }
    }

    println!("{}, {}", num, num2);

    let total_time = now.elapsed();
    println!("Total time: {}μs", total_time.as_micros());
}

fn meets_criteria_2(n: i32) -> bool {
    let mut num = n;
    let mut does_decrease = false;
    let mut prev = 10;
    let mut groups = Vec::new();
    let mut consecutive = 1;

    while num > 0 {
        let i = num % 10;
        if i == prev {
            consecutive += 1;
        } else {
            groups.push(consecutive);
            consecutive = 1;
        }
        if i > prev {
            does_decrease = true;
        }
        prev = i;
        num /= 10;
    }
    groups.push(consecutive);

    let has_double = groups.iter().any(|&x| x == 2);

    !does_decrease && has_double
}

fn meets_criteria(n: i32) -> bool {
    let mut num = n;
    let mut does_decrease = false;
    let mut has_double = false;
    let mut prev = 10;

    while num > 0 {
        let i = num % 10;
        if i == prev {
            has_double = true;
        }
        if i > prev {
            does_decrease = true;
        }
        prev = i;
        num /= 10;
    }

    !does_decrease && has_double
}
