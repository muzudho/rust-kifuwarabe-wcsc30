use super::super::super::super::model::dto::application_part::ap_universe_dto::*;

// 対話モードのタイトル画面
pub fn hyoji_title() {
    // 横幅は 半角79文字使えるぜ☆（＾～＾）
    // 80文字目を使うと、次の行が改行で空行になってしまう☆（＾～＾）
    g_writeln(&format!(
        "\
+--------- --------- --------- --------- --------- --------- --------- -------+
| KifuWarabe Shogi 2018                                                       |
+---------+--------- --------- --------- --------- --------- --------- -------+
          | Created by Muzudho (Doujin Circle Grayscale)                      |
          +--------- --------- --------- --------- --------- --------- -------+
05
          [Enter]
07
08
09
10
11
12
13
14
15
16
17
18
19
20
21
22
23\
"
    ));
}
