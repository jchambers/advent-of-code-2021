fn main() {
}

fn checksum(digits: &[i64]) -> i64 {
    const A: [i64; 14] = [11, 13, 15, -8, 13, 15, -11, -4, -15, 14, 14, -1, -8, -14];
    const B: [i64; 14] = [6, 14, 14, 10, 9, 12, 8, 13, 12, 6, 9, 15, 4, 10];

    let mut checksum = 0;

    for i in 0..digits.len() {
        let shift_left = (checksum % 26) + A[i] != digits[i];
        let shift_right = A[i] < 0;

        if shift_right {
            checksum /= 26;
        }

        if shift_left {
            checksum *= 26;
            checksum += digits[i] + B[i]
        }
    }

    checksum
}

