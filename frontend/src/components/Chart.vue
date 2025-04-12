<template>
  <!-- Корневой контейнер -->
  <div>
    <!-- Чекбоксы и кнопки удаления -->
    <div class="mb-4">
      <div v-for="obj in chartStore.chartObjects" :key="obj.id" class="flex items-center gap-2 mb-2">
        <label class="flex items-center gap-2">
          <input type="checkbox" v-model="obj.visible" @change="toggleVisibility(obj)" />
          {{ obj.id }}
        </label>
        <button @click="removeObject(obj.id)" class="p-2 bg-gray-700 text-white rounded hover:bg-gray-600" title="Удалить объект">
          <svg width="20" height="20" viewBox="0 0 15 15" fill="none" xmlns="http://www.w3.org/2000/svg" class="fill-current">
            <path d="M5 3V2C5 1.44772 5.44772 1 6 1H9C9.55228 1 10 1.44772 10 2V3H12.5C12.7761 3 13 3.22386 13 3.5C13 3.77614 12.7761 4 12.5 4H12V12C12 12.5523 11.5523 13 11 13H4C3.44772 13 3 12.5523 3 12V4H2.5C2.22386 4 2 3.77614 2 3.5C2 3.22386 2.22386 3 2.5 3H5ZM6 2V3H9V2H6ZM4 4V12H11V4H4ZM6 6C6 5.44772 6.44772 5 7 5C7.55228 5 8 5.44772 8 6V10C8 10.5523 7.55228 11 7 11C6.44772 11 6 10.5523 6 10V6Z" fill="currentColor" />
          </svg>
        </button>
      </div>
    </div>
    <!-- Кнопки рисования -->
    <div class="flex gap-2 mb-4">
      <button @click="enableDrawing('line')" class="p-2 bg-gray-700 text-white rounded hover:bg-gray-600" title="Рисовать линию">
        <svg width="20" height="20" viewBox="0 0 15 15" fill="none" xmlns="http://www.w3.org/2000/svg" class="fill-current">
          <path d="M12.2071 2.79289C12.5976 3.18342 12.5976 3.81658 12.2071 4.20711L5.70711 10.7071C5.51957 10.8946 5.26522 11 5 11H3C2.44772 11 2 10.5523 2 10V8C2 7.73478 2.10536 7.48043 2.29289 7.29289L8.79289 0.792893C9.18342 0.402369 9.81658 0.402369 10.2071 0.792893L12.2071 2.79289ZM9.5 2.5L3.5 8.5V9.5H4.5L10.5 3.5L9.5 2.5Z" fill="currentColor" />
        </svg>
      </button>
      <button @click="enableDrawing(null)" class="p-2 bg-gray-700 text-white rounded hover:bg-gray-600" title="Отключить рисование">
        <svg width="20" height="20" viewBox="0 0 15 15" fill="none" xmlns="http://www.w3.org/2000/svg" class="fill-current">
          <path d="M12.8536 2.85355C13.0488 2.65829 13.0488 2.34171 12.8536 2.14645C12.6583 1.95118 12.3417 1.95118 12.1464 2.14645L7.5 6.79289L2.85355 2.14645C2.65829 1.95118 2.34171 1.95118 2.14645 2.14645C1.95118 2.34171 1.95118 2.65829 2.14645 2.85355L6.79289 7.5L2.14645 12.1536C1.95118 12.3488 1.95118 12.6654 2.14645 12.8606C2.34171 13.0558 2.65829 13.0558 2.85355 12.8606L7.5 8.20711L12.1464 12.8606C12.3417 13.0558 12.6583 13.0558 12.8536 12.8606C13.0488 12.6654 13.0488 12.3488 12.8536 12.1536L8.20711 7.5L12.8536 2.85355Z" fill="currentColor" />
        </svg>
      </button>
    </div>
    <!-- Flex-контейнер для боковой панели и графика -->
    <div class="flex">
      <!-- Боковая панель слева -->
      <div class="w-10 bg-gray-800 h-[600px] flex flex-col items-center py-2">
        <!-- Пока пусто, для будущих элементов управления -->
      </div>
      <!-- Контейнер графика с кнопкой возврата -->
      <div class="relative flex-1">
        <div id="chart-container" ref="chartContainer" class="w-full h-[600px]"></div>
        <button @click="scrollToLatestCandle" class="absolute top-2 right-20 p-2 bg-gray-700 text-white rounded hover:bg-gray-600 z-10" title="Вернуться к текущей свече">
          <svg width="20" height="20" viewBox="0 0 15 15" fill="none" xmlns="http://www.w3.org/2000/svg" class="fill-current">
            <path
              d="M8.29289 2.29289C8.68342 1.90237 9.31658 1.90237 9.70711 2.29289L14.2071 6.79289C14.5976 7.18342 14.5976 7.81658 14.2071 8.20711L9.70711 12.7071C9.31658 13.0976 8.68342 13.0976 8.29289 12.7071C7.90237 12.3166 7.90237 11.6834 8.29289 11.2929L11 8.5H1.5C0.947715 8.5 0.5 8.05228 0.5 7.5C0.5 6.94772 0.947715 6.5 1.5 6.5H11L8.29289 3.70711C7.90237 3.31658 7.90237 2.68342 8.29289 2.29289Z" fill="currentColor" />
          </svg>
        </button>
      </div>
    </div>
  </div>
</template>

<script>
import * as LightweightCharts from 'lightweight-charts';
const { createChart, CrosshairMode, version, CandlestickSeries, HistogramSeries, LineStyle } = LightweightCharts;

import { useChartStore } from '@/stores/chart';

console.log("lightweight-charts version:", version());
console.log("Available exports:", Object.keys(LightweightCharts));
console.log("CandlestickSeries available:", !!CandlestickSeries);
console.log("HistogramSeries available:", !!HistogramSeries);
console.log("LineStyle available:", !!LineStyle);

export default {
  name: "Chart",
  setup() {
    const chartStore = useChartStore();
    return { chartStore };
  },
  async mounted() {
    try {
      this.initChart();
      await this.setupWebSocket();
      this.requestHistoricalData();
      this.loadDrawingLines();
    } catch (error) {
      console.error("Error in mounted hook:", error);
    }
  },
  methods: {
    initChart() {
      const chartContainer = this.$refs.chartContainer;
      if (!chartContainer) {
        console.error("Chart container not found");
        return;
      }

      const width = chartContainer.clientWidth || window.innerWidth;
      const height = chartContainer.clientHeight || 600;
      if (width === 0 || height === 0) {
        console.error("Chart container has invalid dimensions:", { width, height });
        return;
      }

      const chart = createChart(chartContainer, {
        width: width,
        height: height,
        layout: {
          background: { color: "rgb(32, 41, 56)" },
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
          borderVisible: false,
        },
      });

      console.log("Chart methods:", Object.getOwnPropertyNames(Object.getPrototypeOf(chart)));

      this.chartStore.setChart(chart);

      // Добавляем серию свечей
      try {
        console.log("Attempting to add Candlestick series");
        const candlestickSeries = chart.addSeries(CandlestickSeries, {
          upColor: "#26a69a",
          downColor: "#ef5350",
          wickUpColor: "#26a69a",
          wickDownColor: "#ef5350",
        });

        candlestickSeries.priceScale().applyOptions({
          scaleMargins: {
            top: 0.1,
            bottom: 0.4,
          },
        });

        console.log("Candlestick series added successfully");

        this.chartStore.addChartObject({
          id: "candlestick",
          type: "candlestick",
          series: candlestickSeries,
          visible: true,
          settings: { upColor: "#26a69a", downColor: "#ef5350" },
        });
      } catch (error) {
        console.error("Failed to add Candlestick series:", error);
        return;
      }

      // Добавляем серию объёмов
      try {
        console.log("Attempting to add Histogram series");
        const volumeSeries = chart.addSeries(HistogramSeries, {
          color: "#26a69a",
          priceFormat: {
            type: "volume",
          },
          priceScaleId: "",
        });

        volumeSeries.priceScale().applyOptions({
          scaleMargins: {
            top: 0.7,
            bottom: 0,
          },
        });

        console.log("Histogram series added successfully");

        this.chartStore.addChartObject({
          id: "volume",
          type: "volume",
          series: volumeSeries,
          visible: true,
          settings: {},
        });
      } catch (error) {
        console.error("Failed to add Histogram series:", error);
        return;
      }

      chart.timeScale().subscribeVisibleTimeRangeChange(() => {
        const timeRange = chart.timeScale().getVisibleLogicalRange();
        if (timeRange && timeRange.from < 0 && !this.chartStore.isLoading) {
          this.loadMoreHistoricalData();
        }
      });

      chartContainer.addEventListener("contextmenu", (e) => {
        e.preventDefault();
        this.enableDrawing(null);
      });

      chart.subscribeClick((param) => {
        if (this.chartStore.drawingTool === "line" && param.point) {
          const candlestickObj = this.chartStore.chartObjects.find((obj) => obj.id === "candlestick");
          if (candlestickObj && candlestickObj.visible) {
            const price = candlestickObj.series.coordinateToPrice(param.point.y);
            const time = chart.timeScale().coordinateToTime(param.point.x);
            const line = candlestickObj.series.createPriceLine({
              price: price,
              color: "#FFD700",
              lineWidth: 1,
              lineStyle: LineStyle.Dashed,
            });
            this.chartStore.addDrawnLine({ price, time, line });
            this.saveDrawingLine(price, time);
          }
        }
      });

      this.chartStore.chartObjects.forEach((obj) => this.toggleVisibility(obj));
    },
    toggleVisibility(obj) {
      obj.series.applyOptions({ visible: obj.visible });
    },
    removeObject(id) {
      const index = this.chartStore.chartObjects.findIndex((obj) => obj.id === id);
      if (index !== -1) {
        const obj = this.chartStore.chartObjects[index];
        this.chartStore.chart.removeSeries(obj.series);
        this.chartStore.chartObjects.splice(index, 1);
      }
    },
    scrollToLatestCandle() {
      if (this.chartStore.chart) {
        try {
          this.chartStore.chart.timeScale().fitContent();
          console.log("График возвращён к текущей свече");
        } catch (error) {
          console.error("Ошибка при возврате к текущей свече:", error);
        }
      }
    },
    async setupWebSocket() {
      return new Promise((resolve) => {
        const websocket = new WebSocket("ws://127.0.0.1:3000/ws");
        this.chartStore.setWebsocket(websocket);
        websocket.onopen = () => {
          console.log("Подключено к WebSocket");
          this.subscribe();
          resolve();
        };
        websocket.onmessage = (event) => {
          const message = JSON.parse(event.data);
          console.log("Получено сообщение от WebSocket:", message);
          if (message.event_type === "kline") {
            this.handleWebSocketMessage(message);
          } else if (message.event_type === "drawing_saved") {
            console.log("Линия сохранена:", message.status);
          } else if (message.event_type === "drawings_loaded") {
            message.data.forEach(({ price }) => {
              const candlestickObj = this.chartStore.chartObjects.find((obj) => obj.id === "candlestick");
              if (candlestickObj && candlestickObj.visible) {
                const line = candlestickObj.series.createPriceLine({
                  price,
                  color: "#FFD700",
                  lineWidth: 1,
                  lineStyle: LineStyle.Dashed,
                });
                this.chartStore.addDrawnLine({ price, line });
              }
            });
            console.log("Линии загружены:", message.data);
          }
        };
        websocket.onerror = (error) => {
          console.error("WebSocket ошибка:", error);
        };
        websocket.onclose = () => {
          console.log("WebSocket закрыт");
        };
      });
    },
    subscribe() {
      const subscription = {
        symbol: this.chartStore.symbol,
        interval: this.chartStore.interval,
        streams: ["kline"],
      };
      if (this.chartStore.websocket && this.chartStore.websocket.readyState === WebSocket.OPEN) {
        this.chartStore.websocket.send(JSON.stringify(subscription));
        console.log("Отправлена подписка:", subscription);
      } else {
        console.error("WebSocket не готов для отправки подписки");
      }
    },
    async requestHistoricalData() {
      const now = Math.floor(Date.now() / 1000);
      const startOfYesterday = Math.floor(new Date().setHours(0, 0, 0, 0) / 1000) - 24 * 60 * 60;

      const url = `http://127.0.0.1:3000/historical?symbol=${this.chartStore.symbol}&interval=${this.chartStore.interval}&start_time=${startOfYesterday * 1000}&end_time=${now * 1000}`;
      console.log("Запрошены начальные исторические данные с начала вчера:", url);

      try {
        const response = await fetch(url);
        if (!response.ok) {
          throw new Error(`HTTP error! status: ${response.status}`);
        }
        const message = await response.json();
        console.log("Получены начальные исторические данные:", message.data.length, "свечей");
        this.handleHistoricalData(message, true);
        this.chartStore.setEarliestTime(startOfYesterday);

        if (message.data.length > 0) {
          const lastCandle = message.data[message.data.length - 1];
          this.chartStore.setLastCandle({
            open: parseFloat(lastCandle.open),
            close: parseFloat(lastCandle.close),
          });
          this.updateCurrentPrice(parseFloat(lastCandle.close));
        }
      } catch (error) {
        console.error("Ошибка при запросе начальных исторических данных:", error);
      }
    },
    async loadMoreHistoricalData() {
      if (this.chartStore.isLoading) return;
      this.chartStore.setLoading(true);

      const candlestickObj = this.chartStore.chartObjects.find((obj) => obj.id === "candlestick");
      if (!candlestickObj) {
        this.chartStore.setLoading(false);
        return;
      }

      // Сохраняем текущий видимый диапазон времени
      const currentRange = this.chartStore.chart.timeScale().getVisibleRange();
      console.log("Сохранён текущий временной диапазон:", currentRange);

      const existingData = candlestickObj.series.data();
      if (existingData.length > 0) {
        const earliestCandleTime = Math.min(...existingData.map((c) => c.time));
        const newEndTime = earliestCandleTime * 1000;
        const newStartTime = newEndTime - 60 * 60 * 8 * 1000;

        if (newStartTime < 0) {
          console.warn("Достигнуто начало доступных данных");
          this.chartStore.setLoading(false);
          return;
        }

        const url = `http://127.0.0.1:3000/historical?symbol=${this.chartStore.symbol}&interval=${this.chartStore.interval}&start_time=${newStartTime}&end_time=${newEndTime}`;
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
          this.handleHistoricalData(message, false);

          // Восстанавливаем сохранённый диапазон
          if (currentRange) {
            try {
              this.chartStore.chart.timeScale().setVisibleRange(currentRange);
              console.log("Восстановлен временной диапазон:", currentRange);
            } catch (error) {
              console.warn("Не удалось восстановить временной диапазон:", error);
              // Не центрируем график, оставляем как есть
            }
          }
          this.chartStore.setEarliestTime(newStartTime / 1000);
        } catch (error) {
          console.error("Ошибка при загрузке данных:", error);
        }
      }

      this.chartStore.setLoading(false);
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

      this.chartStore.setLastCandle({
        open: parseFloat(kline.open),
        close: parseFloat(kline.close),
      });

      const candlestickObj = this.chartStore.chartObjects.find((obj) => obj.id === "candlestick");
      const volumeObj = this.chartStore.chartObjects.find((obj) => obj.id === "volume");

      if (candlestickObj && candlestickObj.visible) {
        candlestickObj.series.update(candle);
      }
      if (volumeObj && volumeObj.visible) {
        volumeObj.series.update(volume);
      }

      this.updateCurrentPrice(candle.close);
    },
    updateCurrentPrice(price) {
      if (this.chartStore.priceLine) {
        const candlestickObj = this.chartStore.chartObjects.find((obj) => obj.id === "candlestick");
        if (candlestickObj) {
          candlestickObj.series.removePriceLine(this.chartStore.priceLine);
        }
      }

      let color = "#00FF00";
      if (this.chartStore.previousPrice !== null) {
        color = price >= this.chartStore.previousPrice ? "#26a69a" : "#ef5350";
      } else if (this.chartStore.lastCandle && this.chartStore.lastCandle.close !== undefined) {
        color = this.chartStore.lastCandle.close >= this.chartStore.lastCandle.open ? "#26a69a" : "#ef5350";
      }
      this.chartStore.setPreviousPrice(price);

      const formattedPrice = price.toFixed(2);

      const candlestickObj = this.chartStore.chartObjects.find((obj) => obj.id === "candlestick");
      if (candlestickObj && candlestickObj.visible) {
        const priceLine = candlestickObj.series.createPriceLine({
          price: price,
          color: color,
          lineWidth: 1,
          lineStyle: LineStyle.Solid,
          axisLabelVisible: true,
          title: "",
          axisLabelColor: color,
          axisLabelTextColor: "#FFFFFF",
          priceFormatter: () => formattedPrice,
        });
        this.chartStore.updatePriceLine(priceLine);
      }
    },
    handleHistoricalData(message, isInitialLoad = false) {
      if (!this.chartStore.chart) {
        console.error("Chart is not initialized. Cannot process historical data:", message);
        return;
      }
      if (message.type !== "historical") {
        console.error("Invalid historical data format:", message);
        return;
      }

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

      const candlestickObj = this.chartStore.chartObjects.find((obj) => obj.id === "candlestick");
      const volumeObj = this.chartStore.chartObjects.find((obj) => obj.id === "volume");

      if (candlestickObj && candlestickObj.visible) {
        const existingData = candlestickObj.series.data();
        if (existingData.length > 0) {
          const newData = candlestickData.filter(
            (newCandle) => !existingData.some((existingCandle) => existingCandle.time === newCandle.time)
          );
          const combinedCandlestickData = [...existingData, ...newData].sort((a, b) => a.time - b.time);
          candlestickObj.series.setData(combinedCandlestickData);
        } else {
          candlestickObj.series.setData(candlestickData);
        }
      }

      if (volumeObj && volumeObj.visible) {
        const existingVolumeData = volumeObj.series.data();
        if (existingVolumeData.length > 0) {
          const combinedVolumeData = [...existingVolumeData, ...volumeData]
            .filter((v, i, self) => self.findIndex((t) => t.time === v.time) === i)
            .sort((a, b) => a.time - b.time);
          volumeObj.series.setData(combinedVolumeData);
        } else {
          volumeObj.series.setData(volumeData);
        }
      }

      // Центрируем только при начальной загрузке
      if (isInitialLoad) {
        this.chartStore.chart.timeScale().fitContent();
        console.log("График центрирован на последних свечах");
      }
    },
    enableDrawing(tool) {
      this.chartStore.setDrawingTool(tool);
      console.log("Инструмент рисования:", tool);
    },
    saveDrawingLine(price, time) {
      const message = {
        event_type: "save_drawing",
        data: { symbol: this.chartStore.symbol, price, time },
      };
      if (this.chartStore.websocket && this.chartStore.websocket.readyState === WebSocket.OPEN) {
        this.chartStore.websocket.send(JSON.stringify(message));
      } else {
        console.error("WebSocket не готов для отправки save_drawing");
      }
    },
    loadDrawingLines() {
      const message = {
        event_type: "load_drawings",
        data: { symbol: this.chartStore.symbol },
      };
      if (this.chartStore.websocket && this.chartStore.websocket.readyState === WebSocket.OPEN) {
        this.chartStore.websocket.send(JSON.stringify(message));
      } else {
        console.error("WebSocket не готов для отправки load_drawings");
      }
    },
  },
  beforeUnmount() {
    if (this.chartStore.websocket) {
      this.chartStore.websocket.close();
    }
    if (this.chartStore.chart) {
      this.chartStore.chart.remove();
    }
  },
};
</script>

<style scoped>
.chart-container {
  @apply w-full h-[600px];
}

.controls {
  @apply mb-4;
}

.control-item {
  @apply flex items-center gap-2 mb-2;
}
</style>