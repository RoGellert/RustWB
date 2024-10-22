### Запуск

```
docker compose up --build
```

### Файл инициализации базы данных

```SQL
-- данные пользователя
CREATE TABLE users (
    user_id INT PRIMARY KEY,
    name VARCHAR,
    email VARCHAR
);


CREATE OR REPLACE FUNCTION notify_user_change() RETURNS trigger AS $$
BEGIN
    IF (TG_OP = 'INSERT') THEN
        PERFORM pg_notify('user_changes', json_build_object('type', 'insert', 'user_id', NEW.user_id, 'name', NEW.name, 'email', NEW.email)::text);
    ELSIF (TG_OP = 'UPDATE') THEN
        PERFORM pg_notify('user_changes', json_build_object('type', 'update', 'user_id', NEW.user_id, 'new_name', NEW.name, 'old_name', OLD.name, 'new_email', NEW.email, 'old_email', OLD.email)::text);
    ELSIF (TG_OP = 'DELETE') THEN
        PERFORM pg_notify('user_changes', json_build_object('type', 'delete', 'user_id', NEW.user_id, 'old_name', OLD.name, 'old_email', OLD.email)::text);
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER user_change_trigger
AFTER INSERT OR UPDATE OR DELETE ON users
FOR EACH ROW EXECUTE FUNCTION notify_user_change();


--------------------------------------------------------------


-- данные продукта
CREATE TABLE products (
    product_id INT PRIMARY KEY,
    name VARCHAR,
    price FLOAT
);


CREATE OR REPLACE FUNCTION notify_product_change() RETURNS trigger AS $$
BEGIN
    IF (TG_OP = 'INSERT') THEN
        PERFORM pg_notify('product_changes', json_build_object('type', 'insert', 'product_id', NEW.product_id, 'name', NEW.name, 'price', NEW.price)::text);
    ELSIF (TG_OP = 'UPDATE') THEN
        PERFORM pg_notify('product_changes', json_build_object('type', 'update', 'product_id', NEW.product_id, 'new_name', NEW.name, 'old_name', OLD.name, 'new_price', NEW.price, 'old_price', OLD.price)::text);
    ELSIF (TG_OP = 'DELETE') THEN
        PERFORM pg_notify('product_changes', json_build_object('type', 'delete', 'product_id', NEW.product_id, 'old_name', OLD.name, 'old_price', OLD.price)::text);
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER product_change_trigger
AFTER INSERT OR UPDATE OR DELETE ON products
FOR EACH ROW EXECUTE FUNCTION notify_product_change();
```

### Структура проекта

#### server


| файл                   | назначение                                                  |
|------------------------|-------------------------------------------------------------|
| main                   | инициализация модулей и запуск сервера                      |
| pg_db                  | модуль работы с базой данных                                |
| modules/product_module | модуль работы с продуктами                                  |
| modules/user_module    | модуль работы с пользователями                              |
| config                 | модуль работы с конфигурацией и чтения аргументов окружения |
| controller             | функции хэндлеров                                           |
| errors                 | обработка ошибок                                            |

#### listener

| файл         | назначение                                                                                                |
|--------------|-----------------------------------------------------------------------------------------------------------|
| main         | инициализация модуля работы с kafka, подключение к базе данных, старт прослушивания и обработки изменений |
| config       | модуль работы с конфигурацией и чтения аргументов окружения                                               |
| kafka_module | модуль работы с сервисои kafka                                                                            |  
