use avatar_cipher::*;

fn main() {
    let tries = 20;
    let msg = random_msg();
    let mut answer = 0;
    let mut an = 0;
    for _ in 0..tries {
        let (a, b) = xor_split(msg);
        an &= a;
        learn(a, b, &mut answer);
        println!("{:064b}", an);
        println!("{:064b}", a);
        println!("{:064b}", b);
        println!("{:064b}", answer);
        println!("{:064b}", msg);
        println!("");
        if answer == msg {
            println!("answered");
            break
        }
    }
}
