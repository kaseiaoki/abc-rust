// -*- coding:utf-8-unix -*-

use std::os::raw::c_longlong;

use proconio::{fastout, input};

// ABC typical90 a 001

// 左右の長さが L [cm] のようかんがあります。 N 個の切れ目が付けられており、左から i 番目の切れ目は左から A
// i
// ​
//   [cm] の位置にあります。

// あなたは N 個の切れ目のうち K 個を選び、ようかんを K+1 個のピースに分割したいです。そこで、以下の値を スコア とします。

// K+1 個のピースのうち、最も短いものの長さ（cm 単位）
// スコアが最大となるように分割する場合に得られるスコアを求めてください。

// 入力例 1

// 3 34
// 1
// 8 13 26

// 出力例 1
// 13
// 左から 2 番目の切れ目で分割すると、長さ 13 [cm] のピースと長さ 21 [cm] のピースに分かれ、スコア 13 を達成できます。

fn main() {
    input! {
        N: usize,
        L: isize,
        K: usize,
        A: [isize; N],
    }
    let mut left = -1;
    let mut right = L + 1;

    while right > left + 1 {
        let mid = left + (right - left) / 2;
        let mut pre = 0;
        let mut cnt = 0;
        for x in A.iter() {
            if *x >= mid + pre && L >= mid + x {
                pre = *x;
                cnt += 1;
            }
        }

        if cnt >= K {
            left = mid;
        } else {
            right = mid;
        }
        // print!("{} {}", right, left);
    }

    print!("{}", left);
}
