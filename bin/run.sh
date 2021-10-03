#!/bin/bash

function usage {
cat >&2 <<EOS
######################################################################################################
# [usage]
#  $0 <BIN_NAME> <TEST_CASE> [options]
#
# [args]
#  <BIN_NAME>:
#    コマンド名。例) 001-slide-bowling
#  <TEST_CASE>:
#    テストケース番号。例) 1
#
# [options]
#  -h | --help:
#    ヘルプを表示
######################################################################################################
EOS
exit 1
}

function error {
  echo "[error] $1" >&2
  exit 1
}

function info {
  echo "[info] $1"
}

PROJECT_DIR=$(cd $(dirname $0)/..; pwd)
cd "${PROJECT_DIR}"

args=() # 引数(ARG1, ARG2)を格納する配列
while [ "$#" != 0 ]; do
  case $1 in
    -h | --help      ) usage;;
    -* | --*         ) error "$1 : 不正なオプションです" ;;
    *                ) args+=("$1");; # 引数を配列に追加します
  esac
  shift
done

[ "${#args[@]}" != 2 ] && usage

BIN_NAME="${args[0]}"
TEST_CASE="${args[1]}"

INPUT="tests/${BIN_NAME}/${TEST_CASE}/input.txt"
OUTPUT="tests/${BIN_NAME}/${TEST_CASE}/output.txt"

RESULT="$(cat "${INPUT}" | RUST_BACKTRACE=1 cargo run --bin "${BIN_NAME}")"
echo "=== === === === Input === === === ==="
cat "${INPUT}"
echo ""
echo "=== === === === Result === === === ==="
echo "${RESULT}"
echo ""
echo "=== === === === Expected === === === ==="
cat "${OUTPUT}"
