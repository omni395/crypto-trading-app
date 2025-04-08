FROM node:20
WORKDIR /app
# Устанавливаем Yarn
RUN npm install -g yarn
# Копируем package.json и yarn.lock (если есть)
COPY package.json yarn.lock* ./
# Устанавливаем зависимости с помощью Yarn
RUN yarn install
COPY . .
EXPOSE 8080
# Запускаем приложение с помощью Yarn
CMD ["yarn", "dev", "--host"]