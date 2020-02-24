/// 递归方式
pub fn fib1(n: i32) -> i32 {
    match n {
        1 | 2 => 1, // 1..=2
        x if x <= 0 => 0,
        _ => fib1(n - 1) + fib1(n - 2),
    }
}

/// 动态规划
pub fn fib2(n: i32) -> i32 {
    if n <= 0 {
        return 0;
    }
    if n <= 2 {
        return 1;
    }
    let mut current = 0;
    let mut prev1 = 1;
    let mut prev2 = 1;
    for _i in 3..=n {
        current = prev1 + prev2;
        prev2 = prev1;
        prev1 = current;
    }
    current
}

/// 流式处理
/// ```rust
///         use hello::fib::fib3;
///         assert_eq!(fib3(-1), 0);
///         assert_eq!(fib3(0), 0);
///         assert_eq!(fib3(1), 1);
///         assert_eq!(fib3(2), 1);
///         assert_eq!(fib3(3), 2);
///         assert_eq!(fib3(4), 3);
///         assert_eq!(fib3(5), 5);
///         assert_eq!(fib3(6), 8);
/// ```
pub fn fib3(n: i32) -> i32 {
    if n <= 0 {
        return 0;
    }
    (0..n).fold((0, 1), |(a, b), _| (b, a + b)).0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_fib1() {
        assert_eq!(fib1(-1), 0);
        assert_eq!(fib1(0), 0);
        assert_eq!(fib1(1), 1);
        assert_eq!(fib1(2), 1);
        assert_eq!(fib1(3), 2);
        assert_eq!(fib1(4), 3);
        assert_eq!(fib1(5), 5);
        assert_eq!(fib1(6), 8);
    }

    #[test]
    pub fn test_fib2() {
        assert_eq!(fib2(-1), 0);
        assert_eq!(fib2(0), 0);
        assert_eq!(fib2(1), 1);
        assert_eq!(fib2(2), 1);
        assert_eq!(fib2(3), 2);
        assert_eq!(fib2(4), 3);
        assert_eq!(fib2(5), 5);
        assert_eq!(fib2(6), 8);
    }

    #[test]
    pub fn test_fib3() {
        assert_eq!(fib3(-1), 0);
        assert_eq!(fib3(0), 0);
        assert_eq!(fib3(1), 1);
        assert_eq!(fib3(2), 1);
        assert_eq!(fib3(3), 2);
        assert_eq!(fib3(4), 3);
        assert_eq!(fib3(5), 5);
        assert_eq!(fib3(6), 8);
    }
}
