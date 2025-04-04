<template>
  <div class="container mx-auto p-4">
    <h1 class="text-2xl font-bold mb-4">Binance WebSocket Data</h1>
    <pre id="data" class="bg-gray-100 p-4 rounded"></pre>
  </div>
</template>

<script>
export default {
  name: 'App',
  mounted() {
    const ws = new WebSocket("ws://127.0.0.1:3000/ws");
    const dataElement = document.getElementById("data");

    ws.onopen = () => {
      console.log("Подключено к локальному WebSocket");
    };

    ws.onmessage = (event) => {
      const data = JSON.parse(event.data);
      console.log("Получены данные:", data);

      if (data.event_type === "kline") {
        const kline = data.kline;
        dataElement.textContent += `Свеча: Open=${kline.open}, High=${kline.high}, Low=${kline.low}, Close=${kline.close}, Time=${kline.start_time}\n`;
      } else if (data.event_type === "depthUpdate") {
        dataElement.textContent += `Стакан: Bids=${JSON.stringify(data.bids.slice(0, 3))}, Asks=${JSON.stringify(data.asks.slice(0, 3))}\n`;
      }
    };

    ws.onerror = (error) => {
      console.log("Ошибка WebSocket:", error);
    };

    ws.onclose = () => {
      console.log("WebSocket закрыт");
    };
  },
};
</script>