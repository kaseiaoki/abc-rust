// -*- coding:utf-8-unix -*-

use proconio::input;

// 002 - Encyclopedia of Parentheses（★3）
// 実行時間制限: 2 sec / メモリ制限: 1024 MB

// 配点: 3 点

// 問題文
// 長さ N の正しいカッコ列をすべて、辞書順に出力してください。

// ただし、正しいカッコ列は次のように定義されています :

// () は正しいカッコ列である
// S が正しいカッコ列であるとき、文字列 ( +S+ ) は正しいカッコ列である
// S,T が正しいカッコ列であるとき、文字列 S+T は正しいカッコ列である
// それ以外の文字列はすべて、正しいカッコ列でない
// 例えば、

// ()()
// (()())(())
// ()()()()()()()()
// は正しいカッコ列ですが、

// )(
// )))()(((
// ((((a))))
// は正しいカッコ列ではありません。

// また、 ( の方が ) よりも辞書順で早いものとします。

// 制約
// 1≤N≤20
// N は整数
// 入力
// 入力は以下の形式で標準入力から与えられます。

// N
// 出力
// 長さ N の正しいカッコ列をすべて、辞書順に、改行区切りで出力してください。

fn main() {
    input! {
        n: usize,
    }
    for i in 0..(1 << n) {
        let mut s: String = "".to_string();
        for j in (0..n).rev() {
            if !(i >> j) & 1 == 1 {
                s += "("
            } else {
                s += ")";
            }
        }
        let _y = &s;

        if is_print(_y.to_string()) {
            print!("{}\n", _y.to_string())
        }
    }
}

fn is_print(s: String) -> bool {
    let mut score = 0;
    for c in s.chars() {
        if c == '(' {
            score += 1;
        } else {
            score -= 1;
        }
        if score < 0 {
            return false;
        };
    }
    return score == 0;
}
