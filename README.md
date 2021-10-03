# proconリポジトリ
プロコン用の練習リポジトリ


```bash
# プロジェクトの作成
./bin/create.sh 001-problem-title

# 実装
vim ./src/bin/001-problem-title.rs

# テストケース作成
vim ./tests/001-problem-title/1/input.txt
vim ./tests/001-problem-title/1/output.txt

# テストケース追加
mkdir -p ./tests/001-problem-title/2
touch ./tests/001-problem-title/2/input.txt
touch ./tests/001-problem-title/2/output.txt

# テスト実行
./bin/run.sh 001-problem-title 1
./bin/run.sh 001-problem-title 2
```
