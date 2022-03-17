use proconio::input;

// https://atcoder.jp/contests/typical90/tasks/typical90_d

fn main() {
    input! {
        h: usize,
        w: usize,
        x: [[u32;h];w]
    }

    let mut tate: Vec<u32> = vec![0; h];
    let mut yoko: Vec<u32> = vec![0; w];

    for i in 0..h {
        for j in 0..w {
            tate[i] += x[i][j];
            yoko[j] += x[i][j];
        }
    }

    for i in 0..h {
        for j in 0..w {
            print!("{}", tate[i] + yoko[j] - x[i][j]);
            if j != w - 1 {
                print!(" ");
            }
        }
        print!("\n")
    }
}
