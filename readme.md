wsl --shutdown
wsl.exe --list --verbose
diskpart
select vdisk file="C:\Users\home\AppData\Local\Docker\wsl\disk\docker_data.vhdx"
compact vdisk


<h1 align="center">Hi there, I'm <a href="https://daniilshat.ru/" target="_blank">Daniil</a> 
<img src="https://github.com/blackcater/blackcater/raw/main/images/Hi.gif" height="32"/></h1>
<h3 align="center">Computer science student, IT news writer from Russia 🇷🇺</h3>


Описание файлов серверной части:
app_state.rs
Что делает: Определяет состояние приложения (AppState), которое используется для передачи данных между компонентами сервера.
За что отвечает: Хранит клиент Redis (через Database) и список подключенных WebSocket-клиентов (clients) с использованием Arc<Mutex<Vec<WsSession>>> для безопасной работы в многопоточной среде. Обеспечивает доступ к этим данным через web::Data<AppState>.
binance.rs
Что делает: Подключается к WebSocket Binance для получения данных в реальном времени (Kline и Depth), запрашивает исторические данные через HTTP API Binance и обрабатывает их.
За что отвечает:
connect_to_binance: Запускает два WebSocket-потока (Kline и Depth).
start_binance_ws: Обрабатывает данные Kline (свечи) и передает их клиентам через WebSocket.
start_binance_depth_ws: Обрабатывает данные Depth (глубина рынка) и передает их клиентам.
get_historical_data и fetch_historical_data: Запрашивает и кэширует исторические данные свечей, включая volume.
database.rs (новый файл)
Что делает: Содержит всю логику взаимодействия с Redis.
За что отвечает:
Хранение данных Kline в упорядоченном множестве (zadd).
Получение исторических данных из Redis (zrangebyscore).
Кэширование исторических данных (set_ex).
Сохранение и загрузка линий рисования (lpush и lrange).
main.rs
Что делает: Является точкой входа приложения, запускает сервер Actix-web и инициализирует подключение к Binance.
За что отвечает:
Создание AppState с клиентом Redis.
Запуск WebSocket-подключений к Binance в отдельном потоке.
Настройка HTTP-сервера с маршрутами и CORS.
models.rs
Что делает: Определяет структуры данных для десериализации и сериализации JSON.
За что отвечает:
KlineEvent и Kline: Данные свечей от Binance (включая volume).
DepthEvent: Данные глубины рынка.
DrawingLine: Структура для линий рисования (добавлена для поддержки функции рисования).
routes.rs
Что делает: Определяет маршруты HTTP-сервера.
За что отвечает:
/ws: Маршрут для подключения WebSocket-клиентов.
/historical: Эндпоинт для получения исторических данных свечей.
/save-drawing (POST): Сохранение линий рисования в Redis.
/load-drawings (GET): Загрузка линий рисования из Redis.
configure_websocket: Регистрация всех маршрутов в приложении.
websocket.rs
Что делает: Реализует WebSocket-сервер для взаимодействия с клиентами.
За что отвечает:
ws_index: Инициализация WebSocket-сессии.
WsSession: Управление подключением клиента, отправка сообщений через send_message, добавление/удаление клиентов из списка в AppState.