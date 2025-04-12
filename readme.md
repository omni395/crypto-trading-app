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




<button aria-label="Показать дерево объектов" data-name="showObjectsTree" id="drawing-toolbar-object-tree" type="button" class="button-KTgbfaP5 apply-common-tooltip common-tooltip-vertical accessible-KTgbfaP5" tabindex="-1" data-tooltip="Показать дерево объектов"><div class="bg-KTgbfaP5"><span class="icon-KTgbfaP5"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 28 28" width="28" height="28"><g fill="currentColor"><path fill-rule="nonzero" d="M14 18.634l-.307-.239-7.37-5.73-2.137-1.665 9.814-7.633 9.816 7.634-.509.394-1.639 1.269-7.667 5.969zm7.054-6.759l1.131-.876-8.184-6.366-8.186 6.367 1.123.875 7.063 5.491 7.054-5.492z"></path><path d="M7 14.5l-1 .57 8 6.43 8-6.5-1-.5-7 5.5z"></path><path d="M7 17.5l-1 .57 8 6.43 8-6.5-1-.5-7 5.5z"></path></g></svg></span></div></button>