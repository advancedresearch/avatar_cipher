use rand;

/// Using a variant of XOR cipher that can be decoded probabilistically.
pub fn xor_split(msg: u64) -> (u64, u64) {
    use rand::Rng;

    let mut r = rand::thread_rng();

    let n1 = r.gen::<u64>();
    (msg ^ n1, !n1)
}

/// Splits a message into two parts.
///
/// The left part keeps 1s while replacing 0s with noise.
/// The right part keeps 0s while replacing 1s with noise.
pub fn split(msg: u64) -> (u64, u64) {
    use rand::Rng;

    let mut r = rand::thread_rng();

    let n1 = r.gen::<u64>();
    let n2 = r.gen::<u64>();
    let n3 = r.gen::<u64>();

    // Replace 1s with noise.
    let left = msg | n1;
    // Replace 0s with noise.
    let right = msg & n2;

    // When 10, the original value is unknown,
    // but since 01 is not possible, one can learn
    // the message from the right part.
    //
    // To prevent this, get cases of 10 and swap
    // both left and right randomly.
    let m = (left ^ right) & n3;
    let left = left ^ m;
    let right = right ^ m;

    (left, right)
}

/// Generates a random message.
pub fn random_msg() -> u64 {
    use rand::Rng;

    let mut r = rand::thread_rng();
    r.gen::<u64>()
}

// 00 0     if the first is 0, the bit is 0
// 01 ?     unknown
// 10 ?     unknown
// 11 1     if the second bit is 1, the bit is 1
//
// Returns the values and a mask telling certain/uncertain.
pub fn join(a: u64, b: u64) -> (u64, u64) {
    let mask = !(a ^ b);
    let r = a & b;
    (r, mask)
}

/// Learns the message over time, using both left and right.
pub fn learn(a: u64, b: u64, answer: &mut u64) {
    *answer |= join(a, b).0
}

// 0 0 if 0, the bit is 0
// 1 ? unknown
//
// Initialize answer to 1s.
pub fn learn_left(a: u64, answer: &mut u64) {
    *answer &= a;
}

// 0 ? unknown
// 1 if 1, the bit is 1.
//
// Initialize answer to 0s.
pub fn learn_right(b: u64, answer: &mut u64) {
    *answer |= b;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut answer = 0;
        let mut left_answer = !0;
        let mut right_answer = 0;
        let msg = random_msg();
        for _ in 0..10 {
            let (a, b) = split(msg);
            learn(a, b, &mut answer);
            learn_left(a, &mut left_answer);
            learn_right(b, &mut right_answer);
        }

        // assert_eq!(answer, msg);
        // assert_eq!(left_answer, msg);
        // assert_eq!(right_answer, msg);
    }
}
