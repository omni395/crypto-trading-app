<template>
  <div class="container mx-auto p-4">
    <h1 class="text-2xl font-bold mb-4 mx-auto">Crypto Trading Apps - test</h1>
    <div ref="chartContainer" class="w-full h-[500px]"></div>
  </div>
</template>

<script>
import { createChart, CrosshairMode } from "lightweight-charts";

export default {
  name: "App",
  data() {
    return {
      chart: null,
      candlestickSeries: null,
      websocket: null,
      symbol: "btcusdt",
      interval: "1m",
    };
  },
  mounted() {
    this.initChart();
    this.setupWebSocket();
  },
  methods: {
    initChart() {
      const chartContainer = this.$refs.chartContainer;
      console.log("Размеры контейнера:", chartContainer.clientWidth, chartContainer.clientHeight);

      this.chart = createChart(chartContainer, {
        width: chartContainer.clientWidth,
        height: chartContainer.clientHeight,
        layout: {
          background: { color: "#222" },
          textColor: "#DDD",
        },
        grid: {
          vertLines: { color: "#444" },
          horzLines: { color: "#444" },
        },
        crosshair: {
          mode: CrosshairMode.Normal,
        },
        timeScale: {
          timeVisible: true,
          secondsVisible: false,
        },
      });

      this.candlestickSeries = this.chart.addCandlestickSeries({
        upColor: "#26a69a",
        downColor: "#ef5350",
        borderVisible: false,
        wickUpColor: "#26a69a",
        wickDownColor: "#ef5350",
      });

      console.log("График инициализирован:", this.chart);
      console.log("Серия свечей инициализирована:", this.candlestickSeries);
    },
    setupWebSocket() {
      this.websocket = new WebSocket("ws://127.0.0.1:3000/ws");
      this.websocket.onopen = () => {
        console.log("Подключено к локальному WebSocket");
        // Вызываем методы только после того, как WebSocket открыт
        this.subscribe();
        this.requestHistoricalData();
      };
      this.websocket.onmessage = (event) => {
        const message = JSON.parse(event.data);
        console.log("Получено сообщение:", message);
        this.handleWebSocketMessage(message);
      };
      this.websocket.onerror = (error) => {
        console.error("WebSocket ошибка:", error);
      };
      this.websocket.onclose = () => {
        console.log("WebSocket закрыт");
      };
    },
    subscribe() {
      const subscription = {
        symbol: this.symbol,
        interval: this.interval,
      };
      this.websocket.send(JSON.stringify(subscription));
      console.log("Отправлена подписка:", subscription);
    },
    requestHistoricalData() {
      const now = Math.floor(Date.now() / 1000);
      const startTime = now - 60 * 60 * 24; // 24 часа назад
      console.log("Текущие значения времени:", { now, startTime });

      const request = {
        type: "historical",
        symbol: this.symbol,
        interval: this.interval,
        startTime: startTime * 1000, // в миллисекундах
        endTime: now * 1000, // в миллисекундах
      };
      this.websocket.send(JSON.stringify(request));
      console.log("Запрошены исторические данные:", {
        symbol: this.symbol,
        interval: this.interval,
        start_time: request.startTime,
        end_time: request.endTime,
      });
    },
    handleWebSocketMessage(message) {
      console.log("Обработка сообщения:", message);
      if (message.event_type === "kline") {
        const kline = message.kline;
        if (kline.is_closed) {
          const candle = {
            time: kline.start_time / 1000,
            open: parseFloat(kline.open),
            high: parseFloat(kline.high),
            low: parseFloat(kline.low),
            close: parseFloat(kline.close),
          };
          console.log("Обновление свечи:", candle);
          this.candlestickSeries.update(candle);
        }
      } else if (message.type === "historical") {
        const historicalData = message.data.map((kline) => ({
          time: kline.time,
          open: parseFloat(kline.open),
          high: parseFloat(kline.high),
          low: parseFloat(kline.low),
          close: parseFloat(kline.close),
        }));
        console.log("Установка исторических данных:", historicalData);
        this.candlestickSeries.setData(historicalData);
      } else {
        console.log("Неизвестный тип сообщения:", message);
      }
    },
  },
  beforeUnmount() {
    if (this.websocket) {
      this.websocket.close();
    }
    if (this.chart) {
      this.chart.remove();
    }
  },
};
</script>

<style>
#chart-container {
  width: 100%;
  height: 500px;
}
</style>