# unofficial-api

2019年度のコンフェスの作品(のAPI部分)

## デプロイ先

* https://unofficial-api.herokuapp.com/

## Usage

1. make docker.serve
2. curl localhost:8000/api/classes/{canceled, moved, supplementary}

Podmanで実行したい場合は"docker"を"podman"に置き換えて実行すれば良い。(e.g make podman.serve)

~~コンフェス期間中は授業がないため2019年12月の情報をとってくるように設定しています。~~

自宅学習期間が延びちゃったからしばらくこのまま。

## Makefile

### make docker.pull

コンテナをpullする

### make docker.serve

サーバーを立ち上げる

### make docker.serve.release

ビルド時の最適化を有効にして、サーバーを立ち上げる

### make docker.run-bash

コンテナ内のBashにアクセスする

## LICENSE

GNU Affero General Public License, Version 3, ([LICENSE](./LICENSE))
