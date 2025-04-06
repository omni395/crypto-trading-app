FROM node:20

# Устанавливаем @vue/cli-service глобально
RUN npm install -g @vue/cli-service

WORKDIR /app
COPY package*.json ./
RUN npm install
COPY . .
EXPOSE 8080
CMD ["npm", "run", "serve"]