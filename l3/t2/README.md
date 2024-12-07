Я использовал notify вместо inotify. Синтаксис примерно тот же, зато работает и на windows в том числе =). Если комипилировать под линукс - под капотом inotify.  Если надо будет переделаю =)

### Структура

```task_creator``` - сервер axum принимающий запросы на POST localhost/tasks 
и записывающий задачи как отдельные файлы в data/tasks

```task_processor``` - процессор задач читающий из data/tasks, обрабатавыющий каждую задачу в отдельном треде и записывающий в data/completed как отдельный файл

```task_logger``` - логгер результатов читающий из data/completed и записывающий в файл data/logs.jsonl

Каждый сервис удаляет файл после прочтения, при успешном выполнении остаётся только строчка лога в data/logs.jsonl

### Пример запроса:

``` 
POST /tasks 
c Body
{
    "task_type" : "sum_of_square_roots",
    "n" : 2000000000
}
```

На выходе в data/logs.jsonl появится строчка: 

``` json
{"log_line":"Задача с uuid: 0e785f74-a2bb-42d8-949c-0e6c3bf766a2 завершена","task":{"task_uuid":"0e785f74-a2bb-42d8-949c-0e6c3bf766a2","task_type":"sum_of_square_roots","n":2000000000,"elapsed":23,"result":59628479422355.336}}
```

### Возможные task_type:
(со всем кроме sleep и sum_of_square_roots при больших числах происходит integer overflow =)
```
"sleep" - просто спит n секунд и возвращает 0 как результат
"fibonnaci" - n-нный номер фибоначчи
"tribonnaci" - n-нный номер трибоначчи (по аналогии с фибоначчи - сумма из 3-ёх)
"sum_of_square_roots" - сумма квадратных корней чисел от 1 до n
"sum_of_squares"  - сумма квадратов чисел от 1 до n
```