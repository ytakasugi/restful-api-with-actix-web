#!/bin/sh

IP_ADDR=127.0.0.1
PORT=8080
CONTENT=$1
TIMESTAMP=`date "+%Y-%m-%dT%H:%M:%S"`
#TIMESTAMP=`date "+%Y-%m-%dT%H:%M:%S.%H%M%S"`

# タスクを登録
curl -X POST -H \"Content-Type: application/json\" -d '{\"content\": \"${CONTENT}\", \"dead_line\": \"${TIMESTAMP}\"}' http://${IP_ADDR}:${PORT}/task
# 登録したタスクを参照
curl http://localhost:8080/task/1

echo "curl -X POST -H \"Content-Type: application/json\" -d '{\"content\": \"${CONTENT}\", \"dead_line\": \"${TIMESTAMP}\"}' http://${IP_ADDR}:${PORT}/task"