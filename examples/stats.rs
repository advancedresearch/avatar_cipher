use avatar_cipher::*;

fn main() {
    let mut count_left: u64 = 0;
    let mut count_right: u64 = 0;
    let mut count_both: u64 = 0;
    let mut count: u64 = 0;

    let tries = 20;
    loop {
        let msg = random_msg();
        let mut answer = 0;
        let mut answer_left = !0;
        let mut answer_right = 0;
        for _ in 0..tries {
            let (a, b) = split(msg);
            learn(a, b, &mut answer);
            learn_left(a, &mut answer_left);
            learn_right(b, &mut answer_right);
        }
        if answer_right == msg {count_right += 1};
        if answer == msg {count_both += 1};
        if answer_left == msg {count_left += 1};
        count += 1;

        if count % 1_000_000 == 0 {
            println!("left {}", count_left as f64 / count as f64);
            println!("right {}", count_right as f64 / count as f64);
            println!("both  {}", count_both as f64 / count as f64);
            println!("{}", count);
        }
    }
}
