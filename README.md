# slack-unfurl-kibela

Kibela のリンクを展開する Slack アプリです

[![Deploy](https://www.herokucdn.com/deploy/button.svg)](https://heroku.com/deploy)

できること
---
- 記事の展開
- 記事に対するコメントの展開

使いかた
---
環境変数を設定して起動すると動きます。必要な環境変数は [.envrc.example](./.envrc.example) を参考にしてください

### AWS Lambda で動かす

`$ make pack-lambda` (docker が必要) と実行すると lambda.zip ができます
これを

- ランタイム: ruby-2.5
- ハンドラー: `lambda_function.lambda_handler`

の設定でアップロードして環境変数を指定すると動作します

LICENSE
---
特に明記されていなければ [LICENSE](./LICENSE) 通り MPL-2.0 です
