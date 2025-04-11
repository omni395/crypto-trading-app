<template>
  <div>
    <div class="controls">
      <div v-for="obj in chartObjects" :key="obj.id" class="control-item">
        <label>
          <input type="checkbox" v-model="obj.visible" @change="toggleVisibility(obj)" />
          {{ obj.id }}
        </label>
        <button @click="removeObject(obj.id)">Удалить</button>
      </div>
    </div>
    <button @click="enableDrawing('line')">Рисовать линию</button>
    <button @click="enableDrawing(null)">Отключить рисование</button>
    <div id="chart-container" ref="chartContainer" class="chart-container"></div>
  </div>
</template>

<script>
import { createChart, CrosshairMode, LineStyle } from "lightweight-charts";

export default {
  name: "Chart",
  data() {
    return {
      chart: null,
      chartObjects: [], // Дерево объектов
      priceLine: null,
      websocket: null,
      symbol: "BTCUSDT",
      interval: "1m",
      earliestTime: null,
      isLoading: false,
      drawingTool: null,
      drawnLines: [],
      previousPrice: null,
      lastCandle: null,
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
          autoScale: true,
        },
      });

      // Определяем шкалы цен
      this.chart.applyOptions({
        priceScale: [
          {
            id: "right", // Шкала для свечей
            visible: true,
            autoScale: true,
          },
          {
            id: "volume", // Шкала для объёмов
            visible: true,
            scaleMargins: {
              top: 0.8,
              bottom: 0,
            },
            autoScale: true,
            priceFormatter: (value) => {
              if (value >= 1000000) return `${(value / 1000000).toFixed(1)}M`;
              if (value >= 1000) return `${(value / 1000).toFixed(1)}K`;
              return value.toFixed(2);
            },
          },
        ],
      });

      // Добавляем свечи
      const candlestickSeries = this.chart.addCandlestickSeries({
        upColor: "#26a69a",
        downColor: "#ef5350",
        borderVisible: false,
        wickUpColor: "#26a69a",
        wickDownColor: "#ef5350",
        priceScaleId: "right",
      });

      // Добавляем объёмы
      const volumeSeries = this.chart.addHistogramSeries({
        color: "#26a69a",
        priceFormat: {
          type: "volume",
          formatter: (value) => {
            if (value >= 1000000) return `${(value / 1000000).toFixed(1)}M`;
            if (value >= 1000) return `${(value / 1000).toFixed(1)}K`;
            return value.toFixed(2);
          },
        },
        priceScaleId: "volume",
        scaleMargins: {
          top: 0.8,
          bottom: 0,
        },
      });

      // Инициализируем дерево объектов
      this.chartObjects = [
        {
          id: "candlestick",
          type: "candlestick",
          series: candlestickSeries,
          visible: true,
          settings: { upColor: "#26a69a", downColor: "#ef5350" },
        },
        {
          id: "volume",
          type: "volume",
          series: volumeSeries,
          visible: true,
          settings: { scaleMargins: { top: 0.8, bottom: 0 } },
        },
      ];

      this.chart.timeScale().subscribeVisibleTimeRangeChange(() => {
        const timeRange = this.chart.timeScale().getVisibleLogicalRange();
        if (timeRange && timeRange.from < 0 && !this.isLoading) {
          this.loadMoreHistoricalData();
        }
      });

      chartContainer.addEventListener("contextmenu", (e) => {
        e.preventDefault();
        this.enableDrawing(null);
      });

      this.chart.subscribeClick((param) => {
        if (this.drawingTool === "line" && param.point) {
          const candlestickObj = this.chartObjects.find((obj) => obj.id === "candlestick");
          if (candlestickObj && candlestickObj.visible) {
            const price = candlestickObj.series.coordinateToPrice(param.point.y);
            const time = this.chart.timeScale().coordinateToLogical(param.point.x);
            const line = candlestickObj.series.createPriceLine({
              price: price,
              color: "#FFD700",
              lineWidth: 1,
              lineStyle: LineStyle.Dashed,
            });
            this.drawnLines.push({ price, time, line });
            this.saveDrawingLine(price, time);
          }
        }
      });
    },
    toggleVisibility(obj) {
      obj.series.applyOptions({ visible: obj.visible });
      if (obj.type === "volume") {
        this.chart.applyOptions({
          priceScale: [
            {
              id: "right",
              visible: true,
              autoScale: true,
            },
            {
              id: "volume",
              visible: obj.visible, // Управляем видимостью шкалы объёмов
              scaleMargins: {
                top: 0.8,
                bottom: 0,
              },
              autoScale: true,
              priceFormatter: (value) => {
                if (value >= 1000000) return `${(value / 1000000).toFixed(1)}M`;
                if (value >= 1000) return `${(value / 1000).toFixed(1)}K`;
                return value.toFixed(2);
              },
            },
          ],
        });
      }
    },
    removeObject(id) {
      const index = this.chartObjects.findIndex((obj) => obj.id === id);
      if (index !== -1) {
        const obj = this.chartObjects[index];
        this.chart.removeSeries(obj.series); // Удаляем серию из графика
        this.chartObjects.splice(index, 1); // Удаляем объект из дерева
        if (obj.type === "volume") {
          this.chart.applyOptions({
            priceScale: [
              {
                id: "right",
                visible: true,
                autoScale: true,
              },
              {
                id: "volume",
                visible: false, // Скрываем шкалу объёмов
              },
            ],
          });
        }
      }
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
              const candlestickObj = this.chartObjects.find((obj) => obj.id === "candlestick");
              if (candlestickObj && candlestickObj.visible) {
                const line = candlestickObj.series.createPriceLine({
                  price,
                  color: "#FFD700",
                  lineWidth: 1,
                  lineStyle: LineStyle.Dashed,
                });
                this.drawnLines.push({ price, line });
              }
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
      const startOfYesterday = Math.floor(new Date().setHours(0, 0, 0, 0) / 1000) - 24 * 60 * 60;

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

        if (message.data.length > 0) {
          const lastCandle = message.data[message.data.length - 1];
          this.lastCandle = {
            open: parseFloat(lastCandle.open),
            close: parseFloat(lastCandle.close),
          };
          this.updateCurrentPrice(parseFloat(lastCandle.close));
        }
      } catch (error) {
        console.error("Ошибка при запросе начальных исторических данных:", error);
      }
    },
    async loadMoreHistoricalData() {
      if (this.isLoading) return;
      this.isLoading = true;

      const candlestickObj = this.chartObjects.find((obj) => obj.id === "candlestick");
      if (!candlestickObj) return;

      const existingData = candlestickObj.series.data();
      if (existingData.length > 0) {
        const earliestCandleTime = Math.min(...existingData.map((c) => c.time));
        const newEndTime = earliestCandleTime * 1000;
        const newStartTime = newEndTime - 60 * 60 * 8 * 1000;

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

      this.lastCandle = {
        open: parseFloat(kline.open),
        close: parseFloat(kline.close),
      };

      const candlestickObj = this.chartObjects.find((obj) => obj.id === "candlestick");
      const volumeObj = this.chartObjects.find((obj) => obj.id === "volume");

      if (candlestickObj && candlestickObj.visible) {
        candlestickObj.series.update(candle);
      }
      if (volumeObj && volumeObj.visible) {
        volumeObj.series.update(volume);
      }

      this.updateCurrentPrice(candle.close);
    },
    updateCurrentPrice(price) {
      if (this.priceLine) {
        const candlestickObj = this.chartObjects.find((obj) => obj.id === "candlestick");
        if (candlestickObj) {
          candlestickObj.series.removePriceLine(this.priceLine);
        }
      }

      let color = "#00FF00";
      if (this.previousPrice !== null) {
        color = price >= this.previousPrice ? "#26a69a" : "#ef5350";
      } else if (this.lastCandle) {
        color = this.lastCandle.close >= this.lastCandle.open ? "#26a69a" : "#ef5350";
      }
      this.previousPrice = price;

      const formattedPrice = price.toFixed(2);

      const candlestickObj = this.chartObjects.find((obj) => obj.id === "candlestick");
      if (candlestickObj && candlestickObj.visible) {
        this.priceLine = candlestickObj.series.createPriceLine({
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

        const candlestickObj = this.chartObjects.find((obj) => obj.id === "candlestick");
        const volumeObj = this.chartObjects.find((obj) => obj.id === "volume");

        if (candlestickObj) {
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

        if (volumeObj) {
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
.chart-container {
  width: 100%;
  height: 600px;
}

.controls {
  margin-bottom: 10px;
}

.control-item {
  display: flex;
  align-items: center;
  gap: 10px;
  margin-bottom: 5px;
}
</style>