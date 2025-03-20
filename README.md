# TaskTracker

Простая утилита для управления задачами через командную строку, написанная на Rust. TaskTracker позволяет добавлять, просматривать, обновлять и удалять задачи, сохраняя их в файле `tasks.json`. Каждая задача имеет ID, описание, статус (Выполнена, В процессе, Отменена) и приоритет (Низкий, Средний, Высокий).

## Возможности
- Добавление задач с описанием и приоритетом (1-3).
- Просмотр списка задач с фильтрацией по статусу или приоритету.
- Обновление статуса или описания задачи по ID.
- Удаление задач по ID.
- Постоянное хранение в `tasks.json`.

## Требования
- [Rust](https://www.rust-lang.org/tools/install) (рекомендуется последняя стабильная версия).
- [Git](https://git-scm.com/downloads) для клонирования репозитория.
- Настроенный SSH-ключ для работы с GitHub (см. [руководство GitHub по SSH](https://docs.github.com/en/authentication/connecting-to-github-with-ssh)).

## Установка

### Клонирование репозитория
1. Склонируйте репозиторий через SSH:
   ```
   git clone git@github.com:sblro4eeek/TaskTracker.git
   cd TaskTracker
   ```
   Убедитесь, что SSH-ключ настроен (см. [Настройка SSH](#настройка-ssh)).

### Сборка проекта
1. Соберите релизную версию:
   ```
   cargo build --release
   ```
2. Исполняемый файл будет находиться в `target/release/TaskTracker`.

## Использование

Запускайте приложение с помощью следующих команд:

### Добавление задачи
```
./target/release/TaskTracker add "Описание задачи" --priority <1-3>
```
- Приоритет по умолчанию: `2` (Средний).
- Пример:
  ```
  ./target/release/TaskTracker add "Написать код" --priority 1
  ```

### Просмотр задач
```
./target/release/TaskTracker list --filter <фильтр>
```
- Фильтры: `all` (все), `done` (выполненные), `in-progress` (в процессе), `cancelled` (отмененные), `low` (низкий), `medium` (средний), `high` (высокий).
- По умолчанию: `all`.
- Пример:
  ```
  ./target/release/TaskTracker list --filter in-progress
  ```

### Обновление задачи
```
./target/release/TaskTracker update <id> --status <статус> --description "Новое описание"
```
- Статусы: `done` (выполнена), `in-progress` (в процессе), `cancelled` (отменена).
- Параметры `--status` и `--description` необязательны.
- Пример:
  ```
  ./target/release/TaskTracker update 1 --status done
  ```

### Удаление задачи
```
./target/release/TaskTracker remove <id>
```
- Пример:
  ```
  ./target/release/TaskTracker remove 1
  ```

## Пример работы
```
./target/release/TaskTracker add "Изучить Rust"
./target/release/TaskTracker add "Создать приложение" --priority 3
./target/release/TaskTracker list
./target/release/TaskTracker update 1 --status done
./target/release/TaskTracker remove 2
```

## Структура проекта
- `src/main.rs`: Точка входа и обработка команд.
- `src/task.rs`: Определение структуры задачи и перечислений (`Task`, `TaskStatus`, `Priority`, `TaskFilter`).
- `src/storage.rs`: Работа с файлом `tasks.json`.
- `src/cli.rs`: Определение интерфейса командной строки через `clap`.

## Зависимости
- `clap`: Обработка аргументов командной строки.
- `prettytable-rs`: Форматирование списка задач в виде таблицы.
- `serde` и `serde_json`: Сериализация/десериализация JSON.
- `anyhow`: Упрощенная обработка ошибок.

Указаны в `Cargo.toml`:
```toml
[dependencies]
clap = { version = "4.5", features = ["derive"] }
prettytable-rs = "0.10"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
anyhow = "1.0"
```

## Хранение данных
Задачи сохраняются в файле `tasks.json` в текущей директории. Пример:
```json
[
  {
    "id": 1,
    "description": "Изучить Rust",
    "status": "Done",
    "priority": "Medium"
  },
  {
    "id": 2,
    "description": "Создать приложение",
    "status": "InProgress",
    "priority": "High"
  }
]
```

## Настройка SSH
Для отправки изменений на GitHub:
1. Сгенерируйте SSH-ключ:
   ```
   ssh-keygen -t ed25519 -C "ваш@email.com"
   ```
2. Добавьте ключ в SSH-агент:
   ```
   eval "$(ssh-agent -s)"
   ssh-add ~/.ssh/id_ed25519
   ```
3. Скопируйте публичный ключ и добавьте его на GitHub:
   ```
   cat ~/.ssh/id_ed25519.pub
   ```
    - Перейдите в `Settings > SSH and GPG keys > New SSH key` на GitHub и вставьте ключ.
4. Проверьте соединение:
   ```
   ssh -T git@github.com
   ```

## Как внести вклад
Форкните репозиторий, внесите изменения и отправьте pull request. Приветствуются любые предложения и сообщения об ошибках!

## Лицензия
Проект распространяется под лицензией MIT.
