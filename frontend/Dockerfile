# Базовый образ
FROM ubuntu:20.04

# Устанавливаем Node.js и npm
RUN apt-get update && apt-get install -y \
    curl \
    && curl -fsSL https://deb.nodesource.com/setup_16.x | bash - \
    && apt-get install -y nodejs \
    && rm -rf /var/lib/apt/lists/*

# Устанавливаем рабочую директорию
WORKDIR /app

# Копируем зависимости фронтенда
COPY package*.json ./
RUN npm install

# Копируем остальной код фронтенда
COPY . .

# Указываем порт
EXPOSE 8080

# Команда для запуска фронтенда
CMD ["npm", "run", "serve"]