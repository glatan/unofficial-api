# unofficial-api

**スクレイピング部の挙動が不安定でレスポンスがなかったり情報が欠落したりなどが発生する**

## Usage

1. make docker.serve
2. curl localhost:8000/{yyyymm}

Podmanで実行したい場合は"docker"を"podman"に置き換えて実行すれば良い。(e.g make podman.serve)

## Command

### make docker.pull

コンテナをpullする

### make docker.serve

サーバーを立ち上げる

### make docker.serve.release

ビルド時の最適化を有効にして、サーバーを立ち上げる

### make docker.run-bash

コンテナ内のBashにアクセスする

## API Documentation

* GET /yyyymm

```json
    // ju: 授業変更
    "ju": [
        // 繰り返し
        [
            // 変更前
            {
                "date": "12月2日", // 日付
                "classNumber": "4-S", // クラス
                "program": "", // なんとかかんとかプログラム(e.g 数学・物理科学プログラム)
                "class": [ // 授業情報
                    {
                        "count": "1・2", // コマ数
                        "name": "電磁気学", // 授業名
                        "teacher": "香取" // 教員の名前
                    }
                ],
                "option": "" // その他(juの変更前のみ確定で空)
            },
            // 変更後
            // 変更がない要素は変更前のものと同じになる
            {
                "date": "12月2日",
                "classNumber": "4-S",
                "program": "",
                "class": [],
                "option": ""
            }
        ]
    ],
    // 補講
    "ho": [
        // 繰り返し
        {
            "date": "12月20日",
            "classNumber": "1-3",
            "program": "",
            "class": [],
            "option": ""
        }
    ],
    // 休講
    "kyu": [
        // 繰り返し
        {
            "date": "12月5日",
            "classNumber": "1-1",
            "program": "",
            "class": [],
            "option": ""
        }
    ]
```
