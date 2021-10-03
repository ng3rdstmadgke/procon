#!/bin/bash

function usage {
cat >&2 <<EOS
######################################################################################################
# [usage]
#  $0 <BIN_NAME>
#
# [args]
#  <BIN_NAME>:
#    コマンド名。例) 001-slide-bowling
#
# [options]
#  -h | --help:
#    ヘルプを表示
#  -f | --force:
#    既に存在していても強制的に作成する
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

FORCE=
args=() # 引数(ARG1, ARG2)を格納する配列
while [ "$#" != 0 ]; do
  case $1 in
    -h | --help      ) usage;;
    -f | --force     ) FORCE="1";;
    -* | --*         ) error "$1 : 不正なオプションです" ;;
    *                ) args+=("$1");; # 引数を配列に追加します
  esac
  shift
done

[ "${#args[@]}" != 1 ] && usage

BIN_NAME="${args[0]}"
RS_FILE="./src/bin/${BIN_NAME}.rs"
TEST_DIR="./tests/${BIN_NAME}/1"

[ -e "$RS_FILE" -a -z "${FORCE}" ] && error "${BIN_NAME} は既に存在します"

cp -v "./src/bin/000-sample.rs" "./src/bin/${BIN_NAME}.rs"
mkdir -vp "${TEST_DIR}"
touch "${TEST_DIR}/input.txt"
touch "${TEST_DIR}/output.txt"
