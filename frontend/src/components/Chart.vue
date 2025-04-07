<template>
    <div id="chart-container" ref="chartContainer" class="w-full h-full"></div>
  </template>
  
  <script>
  import { createChart, CrosshairMode } from "lightweight-charts";
  
  export default {
    name: "Chart",
    data() {
      return {
        chart: null,
        candlestickSeries: null,
        websocket: null,
        symbol: "btcusdt",
        interval: "1m",
        earliestTime: null,
        isLoading: false,
      };
    },
    mounted() {
      this.initChart();
      this.setupWebSocket();
      this.requestHistoricalData();
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
  
        this.chart.timeScale().subscribeVisibleTimeRangeChange(() => {
          const timeRange = this.chart.timeScale().getVisibleLogicalRange();
          if (timeRange && timeRange.from < 0 && !this.isLoading) {
            this.loadMoreHistoricalData();
          }
        });
  
        console.log("График инициализирован:", this.chart);
        console.log("Серия свечей инициализирована:", this.candlestickSeries);
      },
      setupWebSocket() {
        this.websocket = new WebSocket("ws://127.0.0.1:3000/ws");
        this.websocket.onopen = () => {
          console.log("Подключено к локальному WebSocket");
          this.subscribe();
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
      async requestHistoricalData() {
        const now = Math.floor(Date.now() / 1000);
        const startTime = now - 60 * 60 * 24; // 24 часа назад
        console.log("Текущие значения времени:", { now, startTime });
  
        const url = `http://127.0.0.1:3000/historical?symbol=${this.symbol}&interval=${this.interval}&start_time=${startTime * 1000}&end_time=${now * 1000}`;
        console.log("Запрошены исторические данные через HTTP:", url);
  
        try {
          const response = await fetch(url);
          const message = await response.json();
          this.handleWebSocketMessage(message);
        } catch (error) {
          console.error("Ошибка при запросе исторических данных:", error);
        }
  
        this.earliestTime = startTime;
      },
      async loadMoreHistoricalData() {
        if (this.isLoading) return;
        this.isLoading = true;
  
        const newEndTime = this.earliestTime * 1000;
        const newStartTime = (this.earliestTime - 60 * 1000) * 1000;
  
        const url = `http://127.0.0.1:3000/historical?symbol=${this.symbol}&interval=${this.interval}&start_time=${newStartTime}&end_time=${newEndTime}`;
        console.log("Запрошены дополнительные исторические данные через HTTP:", url);
  
        try {
          const response = await fetch(url);
          const message = await response.json();
          this.handleWebSocketMessage(message);
        } catch (error) {
          console.error("Ошибка при запросе дополнительных исторических данных:", error);
        }
  
        this.earliestTime = newStartTime / 1000;
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
  
          if (this.candlestickSeries.data().length > 0) {
            const existingData = this.candlestickSeries.data();
            const newData = historicalData.filter(
              (newCandle) =>
                !existingData.some(
                  (existingCandle) => existingCandle.time === newCandle.time
                )
            );
            this.candlestickSeries.setData([...newData, ...existingData]);
          } else {
            this.candlestickSeries.setData(historicalData);
          }
  
          this.isLoading = false;
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