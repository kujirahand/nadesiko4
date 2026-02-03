# 日本語プログラミング言語「なでしこ」 第四版

Rustによる新しい実装である「なでしこ第四版」の公式リポジトリです。

**まだ何もできません。**

- [なでしこ公式サイト](https://nadesi.com/)

## ビルド方法

```sh
cargo build --release
```

## コマンドライン版の使い方

```sh
# hello.nako4 を実行する場合
target/release/nadesiko4 <hello.nako4>
# ソースを実行する場合
target/release/nadesiko4 -e '「こんにちは」と表示。'
```

## ライセンス

このリポジトリのコードはすべてMITライセンスの下で提供されています。
詳細は `LICENSE` ファイルを参照してください。
