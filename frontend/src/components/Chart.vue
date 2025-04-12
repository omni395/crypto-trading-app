<template>
  <div class="bg-custom">
    <div class="flex">
      <div class="w-10 bg-gray-800 h-[600px] flex flex-col items-center py-2">
        <button
          @click="toggleDrawing"
          :class="[
            'p-2 text-gray-400 hover:text-gray-200',
            chartStore.drawingTool === 'line' ? 'bg-gray-600' : ''
          ]"
          title="Рисовать линию"
        >
          <svg
            width="20"
            height="20"
            viewBox="0 0 15 15"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
            class="fill-current"
          >
            <path
              d="M12.2071 2.79289C12.5976 3.18342 12.5976 3.81658 12.2071 4.20711L5.70711 10.7071C5.51957 10.8946 5.26522 11 5 11H3C2.44772 11 2 10.5523 2 10V8C2 7.73478 2.10536 7.48043 2.29289 7.29289L8.79289 0.792893C9.18342 0.402369 9.81658 0.402369 10.2071 0.792893L12.2071 2.79289ZM9.5 2.5L3.5 8.5V9.5H4.5L10.5 3.5L9.5 2.5Z"
              fill="currentColor"
            />
          </svg>
        </button>
        <div class="flex-1"></div>
        <button
          @click="showModal = true"
          aria-label="Показать дерево объектов"
          data-name="showObjectsTree"
          id="drawing-toolbar-object-tree"
          type="button"
          class="button-KTgbfaP5 apply-common-tooltip common-tooltip-vertical accessible-KTgbfaP5 text-gray-400 hover:text-gray-200"
          tabindex="-1"
          data-tooltip="Показать дерево объектов"
        >
          <div class="bg-KTgbfaP5">
            <span class="icon-KTgbfaP5">
              <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 28 28" width="28" height="28">
                <g fill="currentColor">
                  <path
                    fill-rule="nonzero"
                    d="M14 18.634l-.307-.239-7.37-5.73-2.137-1.665 9.814-7.633 9.816 7.634-.509.394-1.639 1.269-7.667 5.969zm7.054-6.759l1.131-.876-8.184-6.366-8.186 6.367 1.123.875 7.063 5.491 7.054-5.492z"
                  ></path>
                  <path d="M7 14.5l-1 .57 8 6.43 8-6.5-1-.5-7 5.5z"></path>
                  <path d="M7 17.5l-1 .57 8 6.43 8-6.5-1-.5-7 5.5z"></path>
                </g>
              </svg>
            </span>
          </div>
        </button>
      </div>
      <div class="relative flex-1">
        <div id="chart-container" ref="chartContainer" class="w-full h-[600px]"></div>
        <button
          @click="scrollToLatestCandle"
          class="absolute top-2 right-2 p-2 bg-gray-700 text-white rounded hover:bg-gray-600 z-10"
          title="Вернуться к текущей свече"
        >
          <svg
            width="20"
            height="20"
            viewBox="0 0 15 15"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
            class="fill-current"
          >
            <path
              d="M6.70711 2.29289C6.31658 1.90237 5.68342 1.90237 5.29289 2.29289L0.792893 6.79289C0.402369 7.18342 0.402369 7.81658 0.792893 8.20711L5.29289 12.7071C5.68342 13.0976 6.31658 13.0976 6.70711 12.7071C7.09763 12.3166 7.09763 11.6834 6.70711 11.2929L4 8.5H13.5C14.0523 8.5 14.5 8.05228 14.5 7.5C14.5 6.94772 14.0523 6.5 13.5 6.5H4L6.70711 3.70711C7.09763 3.31658 7.09763 2.68342 6.70711 2.29289Z"
              fill="currentColor"
            />
          </svg>
        </button>
      </div>
    </div>

    <div
      v-if="showModal"
      @click.self="showModal = false"
      class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50"
    >
      <div class="bg-gray-800 p-4 rounded-lg w-96">
        <h2 class="text-white text-lg mb-4">Дерево объектов</h2>
        <div class="mb-4">
          <div v-for="obj in chartStore.chartObjects" :key="obj.id" class="flex items-center gap-2 mb-2">
            <label class="flex items-center gap-2 text-white">
              <input type="checkbox" v-model="obj.visible" @change="toggleVisibility(obj)" />
              {{ obj.id }}
            </label>
            <button
              @click="removeObject(obj.id)"
              class="p-2 bg-gray-700 text-white rounded hover:bg-gray-600"
              title="Удалить объект"
            >
              <svg
                width="16"
                height="16"
                viewBox="0 0 15 15"
                fill="none"
                xmlns="http://www.w3.org/2000/svg"
                class="fill-current"
              >
                <path
                  d="M5 3V2C5 1.44772 5.44772 1 6 1H9C9.55228 1 10 1.44772 10 2V3H12.5C12.7761 3 13 3.22386 13 3.5C13 3.77614 12.7761 4 12.5 4H12V12C12 12.5523 11.5523 13 11 13H4C3.44772 13 3 12.5523 3 12V4H2.5C2.22386 4 2 3.77614 2 3.5C2 3.22386 2.22386 3 2.5 3H5ZM6 2V3H9V2H6ZM4 4V12H11V4H4ZM6 6C6 5.44772 6.44772 5 7 5C7.55228 5 8 5.44772 8 6V10C8 10.5523 7.55228 11 7 11C6.44772 11 6 10.5523 6 10V6Z"
                  fill="currentColor"
                />
              </svg>
            </button>
          </div>
        </div>
        <div v-for="(line, index) in chartStore.drawnLines" :key="index" class="flex items-center gap-2 mb-2">
          <span class="text-white">Линия {{ index + 1 }}: {{ line.price.toFixed(2) }}</span>
          <button @click="removeDrawnLine(index)" class="p-1 text-white hover:text-gray-300">
            <svg
              width="16"
              height="16"
              viewBox="0 0 15 15"
              fill="none"
              xmlns="http://www.w3.org/2000/svg"
              class="fill-current"
            >
              <path
                d="M12.8536 2.85355C13.0488 2.65829 13.0488 2.34171 12.8536 2.14645C12.6583 1.95118 12.3417 1.95118 12.1464 2.14645L7.5 6.79289L2.85355 2.14645C2.65829 1.95118 2.34171 1.95118 2.14645 2.14645C1.95118 2.34171 1.95118 2.65829 2.14645 2.85355L6.79289 7.5L2.14645 12.1536C1.95118 12.3488 1.95118 12.6654 2.14645 12.8606C2.34171 13.0558 2.65829 13.0558 2.85355 12.8606L7.5 8.20711L12.1464 12.8606C12.3417 13.0558 12.6583 13.0558 12.8536 12.8606C13.0488 12.6654 13.0488 12.3488 12.8536 12.1536L8.20711 7.5L12.8536 2.85355Z"
                fill="currentColor"
              />
            </svg>
          </button>
          <button @click="openLineProperties(index)" class="p-1 text-white hover:text-gray-300">
            <svg
              width="16"
              height="16"
              viewBox="0 0 15 15"
              fill="none"
              xmlns="http://www.w3.org/2000/svg"
              class="fill-current"
            >
              <path
                d="M7.5 1C6.39543 1 5.5 1.89543 5.5 3C5.5 3.74028 5.93591 4.37338 6.56503 4.68282C5.60462 5.02251 4.89269 5.72727 4.572 6.65842C3.98187 6.31563 3.25 5.69382 3.25 5C3.25 3.89543 4.14543 3 5.25 3C5.59565 3 5.91766 3.10518 6.18616 3.289C6.07548 3.19115 5.94124 3.10991 5.79289 3.05051C5.32652 2.85885 5 2.46659 5 2C5 1.44772 5.44772 1 6 1H7.5ZM7.5 1C8.60457 1 9.5 1.89543 9.5 3C9.5 3.74028 9.06409 4.37338 8.43497 4.68282C9.39538 5.02251 10.1073 5.72727 10.428 6.65842C11.0181 6.31563 11.75 5.69382 11.75 5C11.75 3.89543 10.8546 3 9.75 3C9.40435 3 9.08234 3.10518 8.81384 3.289C8.92452 3.19115 9.05876 3.10991 9.20711 3.05051C9.67348 2.85885 10 2.46659 10 2C10 1.44772 9.55228 1 9 1H7.5ZM7.5 5C5.01472 5 3 7.01472 3 9.5C3 11.9853 5.01472 14 7.5 14C9.98528 14 12 11.9853 12 9.5C12 7.01472 9.98528 5 7.5 5ZM7.5 6C9.433 6 11 7.567 11 9.5C11 11.433 9.433 13 7.5 13C5.567 13 4 11.433 4 9.5C4 7.567 5.567 6 7.5 6Z"
                fill="currentColor"
              />
            </svg>
          </button>
        </div>
        <button @click="showModal = false" class="mt-4 p-2 bg-gray-700 text-white rounded hover:bg-gray-600">
          Закрыть
        </button>
      </div>
    </div>

    <div
      v-if="showPropertiesModal"
      @click.self="showPropertiesModal = false"
      class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50"
    >
      <div class="bg-gray-800 p-4 rounded-lg w-96">
        <h2 class="text-white text-lg mb-4">Свойства линии</h2>
        <div class="mb-4">
          <label class="block text-white mb-2">Цвет линии:</label>
          <input
            type="color"
            v-model="selectedLine.color"
            @change="updateLineProperties"
            class="w-full h-10 rounded"
          />
        </div>
        <div class="mb-4">
          <label class="block text-white mb-2">Толщина линии:</label>
          <input
            type="number"
            v-model.number="selectedLine.lineWidth"
            @change="updateLineProperties"
            min="1"
            max="5"
            class="w-full p-2 bg-gray-700 text-white rounded"
          />
        </div>
        <div class="mb-4">
          <label class="block text-white mb-2">Уровень цены:</label>
          <input
            type="number"
            v-model.number="selectedLine.price"
            @change="updateLineProperties"
            step="0.01"
            class="w-full p-2 bg-gray-700 text-white rounded"
          />
        </div>
        <button
          @click="showPropertiesModal = false"
          class="mt-4 p-2 bg-gray-700 text-white rounded hover:bg-gray-600"
        >
          Закрыть
        </button>
      </div>
    </div>
  </div>
</template>

<script>
import * as LightweightCharts from 'lightweight-charts';
const { createChart, CrosshairMode, CandlestickSeries, HistogramSeries, LineStyle } = LightweightCharts;

import { useChartStore } from '@/stores/chart';

export default {
  name: "Chart",
  setup() {
    const chartStore = useChartStore();
    return { chartStore };
  },
  data() {
    return {
      showModal: false,
      showPropertiesModal: false,
      selectedLine: null,
      selectedLineIndex: null,
    };
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
          vertLine: {
            labelVisible: false, // Отключаем отображение цены на вертикальной линии
          },
          horzLine: {
            labelVisible: false, // Отключаем отображение цены на горизонтальной линии
          },
        },
        timeScale: {
          timeVisible: true,
          secondsVisible: false,
        },
        rightPriceScale: {
          borderVisible: false,
        },
      });

      this.chartStore.setChart(chart);

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

      this.chartStore.addChartObject({
        id: "candlestick",
        type: "candlestick",
        series: candlestickSeries,
        visible: true,
        settings: { upColor: "#26a69a", downColor: "#ef5350" },
      });

      const volumeSeries = chart.addSeries(HistogramSeries, {
        color: "#26a69a",
        priceFormat: {
          type: "volume",
        },
        priceScaleId: "",
        lastValueVisible: true,
      });

      volumeSeries.priceScale().applyOptions({
        scaleMargins: {
          top: 0.7,
          bottom: 0,
        },
      });

      this.chartStore.addChartObject({
        id: "volume",
        type: "volume",
        series: volumeSeries,
        visible: true,
        settings: {},
      });

      chart.timeScale().subscribeVisibleTimeRangeChange(() => {
        const timeRange = chart.timeScale().getVisibleLogicalRange();
        if (timeRange && timeRange.from < 0 && !this.chartStore.isLoading) {
          this.loadMoreHistoricalData();
        }
      });

      chartContainer.addEventListener("contextmenu", (e) => {
        e.preventDefault();
        if (this.chartStore.drawingTool === "line") {
          this.chartStore.setDrawingTool(null);
        }
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
            this.chartStore.addDrawnLine({ price, time, line, color: "#FFD700", lineWidth: 1 });
            this.saveDrawingLine(price, time);
          }
        } else if (param.point) {
          // Проверяем, попал ли клик на нарисованную линию
          const candlestickObj = this.chartStore.chartObjects.find((obj) => obj.id === "candlestick");
          if (candlestickObj && candlestickObj.visible) {
            const price = candlestickObj.series.coordinateToPrice(param.point.y);
            const lineIndex = this.chartStore.drawnLines.findIndex(
              (line) => Math.abs(line.price - price) < 0.5 // Порог для определения клика на линию
            );
            if (lineIndex !== -1) {
              this.openLineProperties(lineIndex);
            }
          }
        }
      });

      this.chartStore.chartObjects.forEach((obj) => this.toggleVisibility(obj));
    },
    toggleDrawing() {
      const newTool = this.chartStore.drawingTool === "line" ? null : "line";
      this.chartStore.setDrawingTool(newTool);
    },
    toggleVisibility(obj) {
      obj.series.applyOptions({ visible: obj.visible });
      if (obj.id === "volume" && obj.visible) {
        this.updateVolumeLabel();
      }
    },
    updateVolumeLabel() {
      const volumeObj = this.chartStore.chartObjects.find((obj) => obj.id === "volume");
      if (volumeObj && volumeObj.visible) {
        const volumeData = volumeObj.series.data();
        if (volumeData.length > 0) {
          const lastVolume = volumeData[volumeData.length - 1];
          const color = lastVolume.color || (this.chartStore.lastCandle.close >= this.chartStore.lastCandle.open ? "#26a69a" : "#ef5350");
          volumeObj.series.applyOptions({
            lastValueVisible: true,
            priceLineVisible: false,
            lastPriceAnimation: 0,
            color: color,
          });
        }
      }
    },
    removeObject(id) {
      const index = this.chartStore.chartObjects.findIndex((obj) => obj.id === id);
      if (index !== -1) {
        const obj = this.chartStore.chartObjects[index];
        this.chartStore.chart.removeSeries(obj.series);
        this.chartStore.chartObjects.splice(index, 1);
      }
    },
    removeDrawnLine(index) {
      const line = this.chartStore.drawnLines[index];
      const candlestickObj = this.chartStore.chartObjects.find((obj) => obj.id === "candlestick");
      if (candlestickObj && line.line) {
        candlestickObj.series.removePriceLine(line.line);
        this.deleteDrawingLine(line.price, line.time);
      }
      this.chartStore.removeDrawnLine(index);
    },
    openLineProperties(index) {
      this.selectedLineIndex = index;
      this.selectedLine = { ...this.chartStore.drawnLines[index] };
      this.showPropertiesModal = true;
    },
    updateLineProperties() {
      const line = this.chartStore.drawnLines[this.selectedLineIndex];
      const candlestickObj = this.chartStore.chartObjects.find((obj) => obj.id === "candlestick");
      if (candlestickObj && line.line) {
        candlestickObj.series.removePriceLine(line.line);
        const updatedLine = candlestickObj.series.createPriceLine({
          price: this.selectedLine.price,
          color: this.selectedLine.color,
          lineWidth: this.selectedLine.lineWidth,
          lineStyle: LineStyle.Dashed,
        });
        this.chartStore.updateDrawnLine(this.selectedLineIndex, {
          ...line,
          price: this.selectedLine.price,
          color: this.selectedLine.color,
          lineWidth: this.selectedLine.lineWidth,
          line: updatedLine,
        });
        this.saveDrawingLine(this.selectedLine.price, line.time);
      }
    },
    scrollToLatestCandle() {
      if (this.chartStore.chart) {
        this.chartStore.chart.timeScale().fitContent();
      }
    },
    async setupWebSocket() {
      return new Promise((resolve) => {
        const websocket = new WebSocket("ws://127.0.0.1:3000/ws");
        this.chartStore.setWebsocket(websocket);
        websocket.onopen = () => {
          this.subscribe();
          resolve();
        };
        websocket.onmessage = (event) => {
          const message = JSON.parse(event.data);
          if (message.event_type === "kline") {
            this.handleWebSocketMessage(message);
          } else if (message.event_type === "drawing_saved") {
            console.log("Линия сохранена:", message.status);
          } else if (message.event_type === "drawings_loaded") {
            message.data.forEach(({ price, time }) => {
              const candlestickObj = this.chartStore.chartObjects.find((obj) => obj.id === "candlestick");
              if (candlestickObj && candlestickObj.visible) {
                const line = candlestickObj.series.createPriceLine({
                  price,
                  color: "#FFD700",
                  lineWidth: 1,
                  lineStyle: LineStyle.Dashed,
                });
                this.chartStore.addDrawnLine({ price, time, line, color: "#FFD700", lineWidth: 1 });
              }
            });
          } else if (message.event_type === "drawing_deleted") {
            console.log("Линия удалена из базы данных:", message.status);
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
      }
    },
    async requestHistoricalData() {
      const now = Math.floor(Date.now() / 1000);
      const startOfYesterday = Math.floor(new Date().setHours(0, 0, 0, 0) / 1000) - 24 * 60 * 60;

      const url = `http://127.0.0.1:3000/historical?symbol=${this.chartStore.symbol}&interval=${this.chartStore.interval}&start_time=${startOfYesterday * 1000}&end_time=${now * 1000}`;

      try {
        const response = await fetch(url);
        if (!response.ok) {
          throw new Error(`HTTP error! status: ${response.status}`);
        }
        const message = await response.json();
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

      const currentRange = this.chartStore.chart.timeScale().getVisibleRange();

      const existingData = candlestickObj.series.data();
      if (existingData.length > 0) {
        const earliestCandleTime = Math.min(...existingData.map((c) => c.time));
        const newEndTime = earliestCandleTime * 1000;
        const newStartTime = newEndTime - 60 * 60 * 8 * 1000;

        if (newStartTime < 0) {
          this.chartStore.setLoading(false);
          return;
        }

        const url = `http://127.0.0.1:3000/historical?symbol=${this.chartStore.symbol}&interval=${this.chartStore.interval}&start_time=${newStartTime}&end_time=${newEndTime}`;

        try {
          const response = await fetch(url);
          if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
          }
          const message = await response.json();
          this.handleHistoricalData(message, false);

          if (currentRange) {
            this.chartStore.chart.timeScale().setVisibleRange(currentRange);
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
        this.updateVolumeLabel();
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
        return;
      }
      if (message.type !== "historical") {
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
        this.updateVolumeLabel();
      }

      if (isInitialLoad) {
        this.chartStore.chart.timeScale().fitContent();
      }
    },
    saveDrawingLine(price, time) {
      const message = {
        event_type: "save_drawing",
        data: { symbol: this.chartStore.symbol, price, time },
      };
      if (this.chartStore.websocket && this.chartStore.websocket.readyState === WebSocket.OPEN) {
        this.chartStore.websocket.send(JSON.stringify(message));
      }
    },
    deleteDrawingLine(price, time) {
      const message = {
        event_type: "delete_drawing",
        data: { symbol: this.chartStore.symbol, price, time },
      };
      if (this.chartStore.websocket && this.chartStore.websocket.readyState === WebSocket.OPEN) {
        this.chartStore.websocket.send(JSON.stringify(message));
      }
    },
    loadDrawingLines() {
      const message = {
        event_type: "load_drawings",
        data: { symbol: this.chartStore.symbol },
      };
      if (this.chartStore.websocket && this.chartStore.websocket.readyState === WebSocket.OPEN) {
        this.chartStore.websocket.send(JSON.stringify(message));
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
.bg-custom {
  background-color: rgb(32, 41, 56);
}

.chart-container {
  @apply w-full h-[600px];
}
</style>