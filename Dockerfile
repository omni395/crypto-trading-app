# Базовый образ
FROM ubuntu:20.04

# Настраиваем часовой пояс автоматически
ENV DEBIAN_FRONTEND=noninteractive
ENV TZ=Europe/Kiev
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone

# Устанавливаем зависимости
RUN apt-get update && apt-get install -y \
    build-essential \
    curl \
    openssl \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Устанавливаем Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Устанавливаем рабочую директорию
WORKDIR /app

# Копируем исходный код
COPY . .

# Собираем проект
RUN cargo build --release

# Команда для запуска сервера
CMD ["cargo", "run", "--release"]