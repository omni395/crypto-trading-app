<template>
  <div>
    <button @click="enableDrawing('line')">Рисовать линию</button>
    <button @click="enableDrawing(null)">Отключить рисование</button>
    <div id="chart-container" ref="chartContainer" class="w-full h-full"></div>
  </div>
</template>

<script>
import { createChart, CrosshairMode, LineStyle } from "lightweight-charts";

export default {
  name: "Chart",
  data() {
    return {
      chart: null,
      candlestickSeries: null,
      volumeSeries: null,
      priceLine: null,
      websocket: null,
      symbol: "BTCUSDT",
      interval: "1m",
      earliestTime: null,
      isLoading: false,
      drawingTool: null,
      drawnLines: [],
    };
  },
  async mounted() {
    this.initChart();
    await this.setupWebSocket();
    this.requestHistoricalData();
    this.loadDrawingLines();
  },
  methods: {
    initChart() {
      const chartContainer = this.$refs.chartContainer;
      if (!chartContainer) {
        console.error("Chart container not found");
        return;
      }

      this.chart = createChart(chartContainer, {
        width: chartContainer.clientWidth,
        height: chartContainer.clientHeight || 600,
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
        rightPriceScale: {
          borderColor: "#444",
        },
      });

      this.candlestickSeries = this.chart.addCandlestickSeries({
        upColor: "#26a69a",
        downColor: "#ef5350",
        borderVisible: false,
        wickUpColor: "#26a69a",
        wickDownColor: "#ef5350",
      });

      this.volumeSeries = this.chart.addHistogramSeries({
        color: "#26a69a",
        priceFormat: {
          type: "volume",
        },
        priceScaleId: "",
        scaleMargins: {
          top: 1.9,
          bottom: 0,
        },
      });

      this.chart.timeScale().subscribeVisibleTimeRangeChange(() => {
        const timeRange = this.chart.timeScale().getVisibleLogicalRange();
        if (timeRange && timeRange.from < 0 && !this.isLoading) {
          this.loadMoreHistoricalData();
        }
      });

      this.chart.subscribeClick((param) => {
        if (this.drawingTool === "line" && param.point) {
          const price = this.candlestickSeries.coordinateToPrice(param.point.y);
          const time = this.chart.timeScale().coordinateToLogical(param.point.x);
          const line = this.candlestickSeries.createPriceLine({
            price: price,
            color: "#FFD700",
            lineWidth: 1,
            lineStyle: LineStyle.Dashed,
          });
          this.drawnLines.push({ price, time, line });
          this.saveDrawingLine(price, time);
        }
      });

      chartContainer.addEventListener("contextmenu", (e) => {
        e.preventDefault();
        this.enableDrawing(null);
      });
    },
    async setupWebSocket() {
      return new Promise((resolve) => {
        this.websocket = new WebSocket("ws://127.0.0.1:3000/ws");
        this.websocket.onopen = () => {
          console.log("Подключено к WebSocket");
          this.subscribe();
          resolve();
        };
        this.websocket.onmessage = (event) => {
          const message = JSON.parse(event.data);
          console.log("Получено сообщение от WebSocket:", message);
          if (message.event_type === "kline") {
            this.handleWebSocketMessage(message);
          } else if (message.event_type === "drawing_saved") {
            console.log("Линия сохранена:", message.status);
          } else if (message.event_type === "drawings_loaded") {
            message.data.forEach(({ price }) => {
              const line = this.candlestickSeries.createPriceLine({
                price,
                color: "#FFD700",
                lineWidth: 1,
                lineStyle: LineStyle.Dashed,
              });
              this.drawnLines.push({ price, line });
            });
            console.log("Линии загружены:", message.data);
          }
        };
        this.websocket.onerror = (error) => {
          console.error("WebSocket ошибка:", error);
        };
        this.websocket.onclose = () => {
          console.log("WebSocket закрыт");
        };
      });
    },
    subscribe() {
      const subscription = {
        symbol: this.symbol,
        interval: this.interval,
        streams: ["kline"],
      };
      this.websocket.send(JSON.stringify(subscription));
      console.log("Отправлена подписка:", subscription);
    },
    async requestHistoricalData() {
      const now = Math.floor(Date.now() / 1000);
      const startOfYesterday = Math.floor(new Date().setHours(0, 0, 0, 0) / 1000) - 24 * 60 * 60; // Начало вчерашнего дня (10.04 00:00)

      const url = `http://127.0.0.1:3000/historical?symbol=${this.symbol}&interval=${this.interval}&start_time=${startOfYesterday * 1000}&end_time=${now * 1000}`;
      console.log("Запрошены начальные исторические данные с начала вчера:", url);

      try {
        const response = await fetch(url);
        if (!response.ok) {
          throw new Error(`HTTP error! status: ${response.status}`);
        }
        const message = await response.json();
        console.log("Получены начальные исторические данные:", message.data.length, "свечей");
        this.handleHistoricalData(message);
        this.earliestTime = startOfYesterday;
      } catch (error) {
        console.error("Ошибка при запросе начальных исторических данных:", error);
      }
    },
    async loadMoreHistoricalData() {
      if (this.isLoading) return;
      this.isLoading = true;

      const existingData = this.candlestickSeries.data();
      if (existingData.length === 0) {
        this.isLoading = false;
        return;
      }

      const earliestCandleTime = Math.min(...existingData.map((c) => c.time));
      const newEndTime = earliestCandleTime * 1000;
      const newStartTime = newEndTime - 60 * 60 * 8 * 1000; // 8 часов назад

      if (newStartTime < 0) {
        console.warn("Достигнуто начало доступных данных");
        this.isLoading = false;
        return;
      }

      const url = `http://127.0.0.1:3000/historical?symbol=${this.symbol}&interval=${this.interval}&start_time=${newStartTime}&end_time=${newEndTime}`;
      console.log(
        "Запрошены дополнительные исторические данные:",
        url,
        `от ${new Date(newStartTime).toISOString()} до ${new Date(newEndTime).toISOString()}`
      );

      try {
        const response = await fetch(url);
        if (!response.ok) {
          throw new Error(`HTTP error! status: ${response.status}`);
        }
        const message = await response.json();
        console.log("Получены дополнительные исторические данные:", message.data.length, "свечей");
        this.handleHistoricalData(message);
        this.earliestTime = newStartTime / 1000;
      } catch (error) {
        console.error("Ошибка при загрузке данных:", error);
      }

      this.isLoading = false;
    },
    handleWebSocketMessage(message) {
      const kline = message.kline;
      const candle = {
        time: kline.start_time / 1000,
        open: parseFloat(kline.open),
        high: parseFloat(kline.high),
        low: parseFloat(kline.low),
        close: parseFloat(kline.close),
      };
      const volume = {
        time: kline.start_time / 1000,
        value: parseFloat(kline.volume || 0),
        color: parseFloat(kline.close) >= parseFloat(kline.open) ? "#26a69a" : "#ef5350",
      };

      this.candlestickSeries.update(candle);
      this.volumeSeries.update(volume);

      if (this.priceLine) {
        this.candlestickSeries.removePriceLine(this.priceLine);
        this.priceLine = null;
      }
    },
    handleHistoricalData(message) {
      if (message.type === "historical") {
        const candlestickData = message.data.map((kline) => ({
          time: kline.time,
          open: parseFloat(kline.open),
          high: parseFloat(kline.high),
          low: parseFloat(kline.low),
          close: parseFloat(kline.close),
        }));

        const volumeData = message.data.map((kline) => ({
          time: kline.time,
          value: parseFloat(kline.volume || 0),
          color: parseFloat(kline.close) >= parseFloat(kline.open) ? "#26a69a" : "#ef5350",
        }));

        const existingData = this.candlestickSeries.data();
        if (existingData.length > 0) {
          const newData = candlestickData.filter(
            (newCandle) => !existingData.some((existingCandle) => existingCandle.time === newCandle.time)
          );
          const combinedCandlestickData = [...existingData, ...newData].sort((a, b) => a.time - b.time);
          const combinedVolumeData = [...this.volumeSeries.data(), ...volumeData]
            .filter((v, i, self) => self.findIndex((t) => t.time === v.time) === i)
            .sort((a, b) => a.time - b.time);

          this.candlestickSeries.setData(combinedCandlestickData);
          this.volumeSeries.setData(combinedVolumeData);
        } else {
          this.candlestickSeries.setData(candlestickData);
          this.volumeSeries.setData(volumeData);
        }
      } else {
        console.error("Неверный формат исторических данных:", message);
      }
    },
    enableDrawing(tool) {
      this.drawingTool = tool;
      console.log("Инструмент рисования:", tool);
    },
    saveDrawingLine(price, time) {
      const message = {
        event_type: "save_drawing",
        data: { symbol: this.symbol, price, time },
      };
      if (this.websocket.readyState === WebSocket.OPEN) {
        this.websocket.send(JSON.stringify(message));
      } else {
        console.error("WebSocket не готов для отправки save_drawing");
      }
    },
    loadDrawingLines() {
      const message = {
        event_type: "load_drawings",
        data: { symbol: this.symbol },
      };
      if (this.websocket.readyState === WebSocket.OPEN) {
        this.websocket.send(JSON.stringify(message));
      } else {
        console.error("WebSocket не готов для отправки load_drawings");
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

<style scoped>
#chart-container {
  width: 100%;
  height: 600px;
}
</style>