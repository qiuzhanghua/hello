use std::collections::BTreeSet;

/// n个相同的球放进m个相同的盒子中，总共有几种放的方法。
pub fn ball_in_box(ball_number: usize, box_number: usize) -> Vec<Vec<usize>> {
    let mut dp = vec![vec![vec![vec![]; 0]; box_number]; ball_number + 1];
    let mut v = vec![0];

    for i in 0..box_number {
        let mut row = Vec::<Vec<usize>>::new();
        row.push(v.clone());
        dp[0][i] = row;
        v.push(0);
    }
    //    print_dp(&dp);
    // dp[k][1]=1,dp[1][k]=1,dp[0][k]=1
    for i in 1..=ball_number {
        for j in 0..box_number {
            if j == 0 {
                let v = vec![i];
                dp[i][j].push(v);
            } else if i > j {
                // dp[n][m]=dp[n][m-1]+dp[n-m][m], n>=m
                let mut x = dp[i][j - 1].clone();
                for y in &mut x {
                    y.push(0)
                }
                dp[i][j] = x;
                let mut z = dp[i - j - 1][j].clone();
                for y in &mut z {
                    for item in y.iter_mut() {
                        *item += 1;
                    }
                    // for k in 0..y.len() {
                    //     y[k] += 1;
                    // }
                    dp[i][j].push(y.to_owned());
                }
            } else {
                // dp[n][m]=dp[n][m-1], n<m
                let mut x = dp[i][j - 1].clone();
                for y in &mut x {
                    y.push(0)
                }
                dp[i][j] = x;
            }
        }
    }
    dp[ball_number][box_number - 1].clone()
}

/// 平面上的n条直线，没有3条或者以上的直线交于一点，可能的交点数。
/// n: 线的数量, cell: 平行线集合的列表
///
pub fn count_cross(n: usize, cell: &[Vec<usize>]) -> BTreeSet<usize> {
    let mut ans = BTreeSet::<usize>::new();
    for v in cell.iter() {
        //       let v = &cell[i];
        let mut sum = 0;
        let mut p = 0;
        for c in v.iter() {
            p += *c;
            sum += c * (n - p);
        }
        ans.insert(sum);
        for c in v.iter() {
            print!("{}, ", c);
        }
        println!(" = {}", sum);
    }
    ans
}

// fn print_dp(dp: &[Vec<Vec<Vec<usize>>>]) {
//     for i in 0..dp.len() {
//         for j in 0..dp[0].len() {
//             print!("{:?} ", dp[i][j]);
//         }
//         println!()
//     }
// }
//
// fn print_cell(cell: &[Vec<usize>]) {
//     for i in 0..cell.len() {
//         for j in 0..cell[0].len() {
//             print!("{}, ", cell[i][j]);
//         }
//         println!()
//     }
// }
