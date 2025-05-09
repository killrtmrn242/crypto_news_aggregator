# Crypto News Aggregator

Этот проект — агрегатор новостей о криптовалютах, который использует API CryptoCompare для получения актуальных новостей по запросу пользователя. Пользователи могут вводить символ криптовалюты (например, BTC) и получать список последних новостей, связанных с выбранной криптовалютой.

## Стек технологий

- **Rust** — основной язык программирования
- **Actix-web** — фреймворк для создания веб-серверов
- **Askama** — шаблонизатор для генерации HTML
- **Reqwest** — HTTP-клиент для работы с внешними API
- **Dotenv** — для работы с переменными окружения
- **Chrono** — библиотека для работы с датами и временем

## Требования

- **Rust** (версия 1.56 или выше)
- **Cargo** (должен быть установлен вместе с Rust)
- **Node.js** (для запуска статических файлов и CSS)
- **.env файл с API-ключом** для доступа к CryptoCompare API

## Установка

1. **Клонируйте репозиторий:**

    ```bash
    git clone https://github.com/yourusername/crypto-news-aggregator.git
    cd crypto-news-aggregator
    ```

2. **Установите зависимости:**

    Для установки всех зависимостей, используемых в проекте, выполните команду:

    ```bash
    cargo build
    ```

3. **Создайте файл `.env`** в корне проекта и добавьте ваш API-ключ от CryptoCompare:

    ```env
    CRYPTOCOMPARE_API_KEY=your_api_key
    ```

    **Получить API-ключ** можно, зарегистрировавшись на сайте CryptoCompare: https://min-api.cryptocompare.com

4. **Запуск проекта**:

    Для запуска проекта выполните команду:

    ```bash
    cargo run
    ```

    Сервер будет доступен по адресу: `http://127.0.0.1:8080`

5. **Статические файлы**:

    Проект также использует CSS для стилизации страницы. Статические файлы должны быть размещены в папке `/static`.

    CSS-файл доступен по пути `static/styles.css` и используется для стилизации страницы поиска и отображения новостей.

## Использование

1. Перейдите в браузер по адресу `http://127.0.0.1:8080`.
2. Введите символ криптовалюты (например, `BTC`) в поле поиска и нажмите "Search".
3. Страница отобразит список последних новостей, связанных с выбранной криптовалютой. Если новостей нет, отобразится сообщение "No articles found".

## Структура проекта

```
/crypto-news-aggregator
├── /static                  # Статические файлы (например, CSS)
│   └── styles.css           # Стили для сайта
├── src
│   ├── main.rs              # Основной файл с логикой сервера
├── .env                     # Конфигурация с API-ключом
├── Cargo.toml               # Зависимости проекта
└── README.md                # Документация
```

## Лицензия

Этот проект распространяется под лицензией MIT. Подробнее см. в файле `LICENSE`.
