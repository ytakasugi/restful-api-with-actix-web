```
■ユーザー追加
curl -X POST -H "Content-Type: application/json" -d '{"user_name": "admin", "e_mail": "admin@example.com"}' http://localhost:8080/todo/users

curl -X POST -H "Content-Type: application/json" -d '{"user_name": "test01", "e_mail": "test01@example.com"}' http://localhost:8080/todo/users
curl -X POST -H "Content-Type: application/json" -d '{"user_name": "test02", "e_mail": "test02@example.com"}' http://localhost:8080/todo/users
curl -X POST -H "Content-Type: application/json" -d '{"user_name": "test03", "e_mail": "test03@example.com"}' http://localhost:8080/todo/users
curl -X POST -H "Content-Type: application/json" -d '{"user_name": "test04", "e_mail": "test04@example.com"}' http://localhost:8080/todo/users
curl -X POST -H "Content-Type: application/json" -d '{"user_name": "test05", "e_mail": "test05@example.com"}' http://localhost:8080/todo/users
curl -X POST -H "Content-Type: application/json" -d '{"user_name": "test06", "e_mail": "test06@example.com"}' http://localhost:8080/todo/users

■タスク追加
curl -X POST -H "Content-Type: application/json" -d '{"user_id": 1, "content": "Rust", "dead_line": "2022-05-30T12:00:00"}' http://localhost:8080/todo/task
curl -X POST -H "Content-Type: application/json" -d '{"user_id": 1, "content": "Go", "dead_line": "2022-05-30T12:00:00"}' http://localhost:8080/todo/task
curl -X POST -H "Content-Type: application/json" -d '{"user_id": 1, "content": "Java", "dead_line": "2022-05-30T12:00:00"}' http://localhost:8080/todo/task

■参照
curl http://localhost:8080/todo/users/1/tasks/1
curl http://localhost:8080/todo/users
curl http://localhost:8080/todo/users/1/tasks
curl http://localhost:8080/todo/tasks

■更新
curl -X PUT -H "Content-Type: application/json" -d '{"dead_line": "2022-06-30T12:00:00"}' http://localhost:8080/todo/users/1/tasks/1
curl -X PUT -H "Content-Type: application/json" -d '{"e_mail": "test01test01@example.com"}' http://localhost:8080/todo/users/2

■削除
curl -X DELETE http://localhost:8080/todo/users/2

■一括削除
curl -X DELETE http://localhost:8080/todo/users

■参考
GETメソッドでJson形式でパラメータを渡す場合
curl -X GET -H "accept: application/json" -H "Content-Type: application/json" -d '{"user_id": 1, "user_name": "admin", "e_mail": null}' http://localhost:8080/todo/users
```