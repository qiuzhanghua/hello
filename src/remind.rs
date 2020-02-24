///! 按位获取十进制数的奇偶性
pub fn reminder(x: i32) -> i32 {
    let mut x = x;
    let mut ans = 0;
    let mut count = 0;
    loop {
        ans += (x % 2) << count;
        x /= 10;
        count += 1;
        if x == 0 {
            break;
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reminder() {
        assert_eq!(
            format!("{:>0width$b}", reminder(23), width = 2),
            String::from("01")
        )
    }

    #[test]
    fn test_reminder_2() {
        let number = 23;

        let mut count = 0;
        for i in 10..100 {
            if reminder(i + number) == reminder(i) + reminder(number) {
                count += 1;
                println!(
                    "{} ({:>0width$b}) = {}({:>0width$b}) + {}({:>0width$b})",
                    i + number,
                    reminder(i + number),
                    i,
                    reminder(i),
                    number,
                    reminder(number),
                    width = 2
                )
            }
        }
        println!("{:?}", count);
    }
}
