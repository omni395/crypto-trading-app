<!-- src/components/chart/Drawings.vue -->
<template>
  <div class="w-10 bg-gray-800 h-[600px] flex flex-col items-center py-2">
    <!-- Кнопка для горизонтального уровня -->
    <button
      @click="toggleDrawing('line')"
      :class="[
        'p-2 text-gray-400 hover:text-gray-200',
        chartStore.drawingTool === 'line' ? 'text-green-400' : ''
      ]"
      title="Горизонтальный уровень"
    >
      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 28 28" width="28" height="28">
        <g fill="currentColor" fill-rule="nonzero">
          <path d="M4 15h8.5v-1h-8.5zM16.5 15h8.5v-1h-8.5z"></path>
          <path
            d="M14.5 16c.828 0 1.5-.672 1.5-1.5s-.672-1.5-1.5-1.5-1.5.672-1.5 1.5.672 1.5 1.5 1.5zm0 1c-1.381 0-2.5-1.119-2.5-2.5s1.119-2.5 2.5-2.5 2.5 1.119 2.5 2.5-1.119 2.5-2.5 2.5z"
          />
        </g>
      </svg>
    </button>

    <!-- Кнопка для горизонтального луча -->
    <button
      @click="toggleDrawing('ray')"
      :class="[
        'p-2 text-gray-400 hover:text-gray-200',
        chartStore.drawingTool === 'ray' ? 'text-green-400' : ''
      ]"
      title="Горизонтальный луч"
    >
      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 28 28" width="28" height="28">
        <g fill="currentColor" fill-rule="nonzero">
          <path d="M8.5 15h16.5v-1h-16.5z"></path>
          <path
            d="M6.5 16c.828 0 1.5-.672 1.5-1.5s-.672-1.5-1.5-1.5-1.5.672-1.5 1.5.672 1.5 1.5 1.5zm0 1c-1.381 0-2.5-1.119-2.5-2.5s1.119-2.5 2.5-2.5 2.5 1.119 2.5 2.5-1.119 2.5-2.5 2.5z"
          />
        </g>
      </svg>
    </button>

    <!-- Кнопка для линии тренда -->
    <button
      @click="toggleDrawing('trendline')"
      :class="[
        'p-2 text-gray-400 hover:text-gray-200',
        chartStore.drawingTool === 'trendline' ? 'text-green-400' : ''
      ]"
      title="Линия тренда"
    >
      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 28 28" width="28" height="28">
        <g fill="currentColor" fill-rule="nonzero">
          <path d="M7.354 21.354l14-14-.707-.707-14 14z"></path>
          <path
            d="M22.5 7c.828 0 1.5-.672 1.5-1.5s-.672-1.5-1.5-1.5-1.5.672-1.5 1.5.672 1.5 1.5 1.5zm0 1c-1.381 0-2.5-1.119-2.5-2.5s1.119-2.5 2.5-2.5 2.5 1.119 2.5 2.5-1.119 2.5-2.5 2.5zM5.5 24c.828 0 1.5-.672 1.5-1.5s-.672-1.5-1.5-1.5-1.5.672-1.5 1.5.672 1.5 1.5 1.5zm0 1c-1.381 0-2.5-1.119-2.5-2.5s1.119-2.5 2.5-2.5 2.5 1.119 2.5 2.5-1.119 2.5-2.5 2.5z"
          />
        </g>
      </svg>
    </button>

    <div class="flex-1"></div>

    <!-- Кнопка для магнита -->
    <button
      @click="toggleMagnet"
      :class="[
        'p-2 text-gray-400 hover:text-gray-200',
        magnetEnabled ? 'text-green-400' : ''
      ]"
      title="Переключить магнит"
    >
      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 28 28" width="28" height="28">
        <g fill="currentColor" fill-rule="evenodd">
          <path
            fill-rule="nonzero"
            d="M14 10a2 2 0 0 0-2 2v11H6V12c0-4.416 3.584-8 8-8s8 3.584 8 8v11h-6V12a2 2 0 0 0-2-2zm-3 2a3 3 0 0 1 6 0v10h4V12c0-3.864-3.136-7-7-7s-7 3.136-7 7v10h4V12z"
          />
          <path d="M6.5 18h5v1h-5zm10 0h5v1h-5z" />
        </g>
      </svg>
    </button>

    <!-- Кнопка для открытия дерева объектов -->
    <button
      @click="$emit('update:show-modal', true)"
      class="p-2 text-gray-400 hover:text-gray-200"
      title="Показать настройки графика"
    >
      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 28 28" width="28" height="28" fill="none">
        <path
          fill="currentColor"
          fill-rule="evenodd"
          clip-rule="evenodd"
          d="M13 5.5c0-.28.22-.5.5-.5h1c.28 0 .5.22.5.5V7.05l.4.09c.9.18 1.73.53 2.46 1.02l.34.23.29-.3.81-.8c.2-.2.52-.2.71 0l.7.7.36-.35-.35.35c.2.2.2.51 0 .7l-.82.82-.29.29.23.34c.49.73.84 1.57 1.02 2.46l.08.4H22.5c.28 0 .5.22.5.5v1a.5.5 0 0 1-.5.5H20.95l-.09.4c-.18.9-.53 1.73-1.02 2.46l-.23.34.3.29.8.81c.2.2.2.52 0 .71l-.7.7a.5.5 0 0 1-.7 0l-.82-.8-.29-.3-.34.23c-.73.49-1.57.84-2.46 1.02l-.4.08V22.5a.5.5 0 0 1-.5.5h-1a.5.5 0 0 1-.5-.5V20.95l-.4-.09a6.96 6.96 0 0 1-2.46-1.02l-.34-.23-.29.3-.81.8.35.36-.35-.35a.5.5 0 0 1-.71 0l-.7-.71a.5.5 0 0 1 0-.7l-.36-.36.35.35.82-.81.29-.29-.23-.34a6.96 6.96 0 0 1-1.02-2.46l-.08-.4H5.5a.5.5 0 0 1-.5-.5v-1c0-.28.22-.5.5-.5H7.05l.09-.4c.18-.9.53-1.73 1.02-2.46l.23-.34-.3-.29-.8-.81a.5.5 0 0 1 0-.71l.7-.7c.2-.2.51-.2.7 0l.82.8.29.3.34-.23a6.96 6.96 0 0 1 2.46-1.02l.4-.08V5.5zm.5-1.5c-.83 0-1.5.67-1.5 1.5v.75c-.73.2-1.43.48-2.06.86l-.54-.53a1.5 1.5 0 0 0-2.12 0l-.7.7a1.5 1.5 0 0 0 0 2.12l.53.54A7.95 7.95 0 0 0 6.25 12H5.5c-.83 0-1.5.67-1.5 1.5v1c0 .83.67 1.5 1.5 1.5h.75c.2.73.48 1.43.86 2.06l-.53.54a1.5 1.5 0 0 0 0 2.12l.7.7a1.5 1.5 0 0 0 2.12 0l.54-.53c.63.38 1.33.67 2.06.86v.75c0 .83.67 1.5 1.5 1.5h1c.83 0 1.5-.67 1.5-1.5v-.75a7.95 7.95 0 0 0 2.06-.86l.54.53a1.5 1.5 0 0 0 2.12 0l.7-.7a1.5 1.5 0 0 0 0-2.12l-.53-.54c.38-.63.67-1.33.86-2.06h.75c.83 0 1.5-.67 1.5-1.5v-1c0-.83-.67-1.5-1.5-1.5h-.75a7.95 7.95 0 0 0-.86-2.06l.53-.54a1.5 1.5 0 0 0 0-2.12l-.7-.7a1.5 1.5 0 0 0-2.12 0l-.54.53A7.95 7.95 0 0 0 16 6.25V5.5c0-.83-.67-1.5-1.5-1.5h-1zM12 14a2 2 0 1 1 4 0 2 2 0 0 1-4 0zm2-3a3 3 0 1 0 0 6 3 3 0 0 0 0-6z"
        />
      </svg>
    </button>
  </div>

  <!-- Модалка свойств линии -->
  <div
    v-if="showPropertiesModal"
    @click.self="showPropertiesModal = false"
    class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50"
  >
    <div class="bg-gray-800 p-0 rounded-lg w-96">
      <div class="flex justify-between items-center p-2 border-b border-gray-600">
        <h2 class="text-white text-lg">Свойства линии</h2>
        <button @click="showPropertiesModal = false" class="text-gray-400 hover:text-gray-200">
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
      </div>
      <div class="p-2">
        <div class="flex items-center gap-2 mb-2">
          <label class="text-white w-24">Цвет:</label>
          <input
            type="color"
            v-model="selectedLine.data.color"
            @change="updateLineProperties"
            class="w-10 h-6 rounded"
          />
        </div>
        <div class="flex items-center gap-2 mb-2">
          <label class="text-white w-24">Толщина:</label>
          <select
            v-model.number="selectedLine.data.line_width"
            @change="updateLineProperties"
            class="p-1 bg-gray-700 text-white rounded w-16"
          >
            <option v-for="size in [1, 2, 3, 4, 5]" :key="size" :value="size">{{ size }}px</option>
          </select>
        </div>
        <div v-if="selectedLine.drawing_type !== 'drawing.trendline'" class="flex items-center gap-2 mb-2">
          <label class="text-white w-24">Цена:</label>
          <input
            type="number"
            v-model.number="selectedLine.data.price"
            @change="updateLineProperties"
            step="0.01"
            class="p-1 bg-gray-700 text-white rounded w-32"
          />
        </div>
        <div v-if="selectedLine.drawing_type === 'drawing.trendline'" class="flex items-center gap-2 mb-2">
          <label class="text-white w-24">Начальная цена:</label>
          <input
            type="number"
            v-model.number="selectedLine.data.start_price"
            @change="updateLineProperties"
            step="0.01"
            class="p-1 bg-gray-700 text-white rounded w-32"
          />
        </div>
        <div v-if="selectedLine.drawing_type === 'drawing.trendline'" class="flex items-center gap-2 mb-2">
          <label class="text-white w-24">Конечная цена:</label>
          <input
            type="number"
            v-model.number="selectedLine.data.end_price"
            @change="updateLineProperties"
            step="0.01"
            class="p-1 bg-gray-700 text-white rounded w-32"
          />
        </div>
        <button
          @click="$emit('remove-line', selectedLine.id); showPropertiesModal = false"
          class="p-2 bg-red-600 text-white rounded hover:bg-red-700"
          title="Удалить линию"
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
  </div>
</template>

<script>
import { LineSeries, LineStyle } from 'lightweight-charts';
import { useChartStore } from '@/stores/chart';

export default {
  name: 'Drawings',
  props: {
    chart: {
      type: Object,
      required: true,
    },
  },
  emits: ['chart-click', 'edit-line'],
  setup() {
    const chartStore = useChartStore();
    return { chartStore };
  },
  data() {
    return {
      magnetEnabled: false,
      trendlineStart: null,
      showPropertiesModal: false,
      selectedLine: null,
      selectedLineIndex: null,
    };
  },
  watch: {
    chart: {
      immediate: true,
      handler(newChart) {
        if (newChart) {
          this.loadInitialDrawings();
        }
      },
    },
  },
  mounted() {
    this.chartStore.$subscribe((mutation, state) => {
      if (mutation.type === 'setDrawingTool' && state.drawingTool !== 'trendline') {
        this.trendlineStart = null;
      }
      if (mutation.type === 'removeDrawnLine' && this.selectedLineIndex === mutation.payload) {
        this.selectedLineIndex = null;
        this.showPropertiesModal = false;
      }
      if (['drawings_loaded', 'drawing_added'].includes(mutation.type)) {
        this.loadDrawings(mutation.payload || state.drawnLines);
      }
      if (mutation.type === 'drawing_deleted') {
        this.handleDrawingDeleted(mutation.payload);
      }
      if (mutation.type === 'drawing_saved') {
        this.handleDrawingSaved(mutation.payload);
      }
    });
  },
  methods: {
    toggleDrawing(tool) {
      const newTool = this.chartStore.drawingTool === tool ? null : tool;
      this.chartStore.setDrawingTool(newTool);
      if (newTool !== 'trendline') {
        this.trendlineStart = null;
      }
    },
    toggleMagnet() {
      this.magnetEnabled = !this.magnetEnabled;
    },
    handleChartClick({ price, time, point }) {
      if (!this.chart) {
        console.error('График не инициализирован');
        return;
      }

      const candlestickObj = this.chartStore.chartObjects.find((obj) => obj.id === 'candlestick');
      if (!candlestickObj || !candlestickObj.visible) return;

      let adjustedPrice = price;
      if (this.magnetEnabled) {
        adjustedPrice = this.getMagnetPrice(candlestickObj.series, price, time);
      }

      if (this.chartStore.drawingTool === 'line') {
        const line = candlestickObj.series.createPriceLine({
          price: adjustedPrice,
          color: '#FFD700',
          lineWidth: 1,
          lineStyle: LineStyle.Dashed,
        });
        this.chartStore.addDrawnLine({
          id: null,
          drawing_type: 'drawing.line',
          symbol: this.chartStore.symbol,
          data: { price: adjustedPrice, color: '#FFD700', line_width: 1 },
          line,
        });
        this.saveDrawingLine('drawing.line', { price: adjustedPrice, color: '#FFD700', line_width: 1 });
        this.chartStore.setDrawingTool(null);
      } else if (this.chartStore.drawingTool === 'ray') {
        let raySeries;
        try {
          raySeries = this.chart.addLineSeries({
            color: '#FFD700',
            lineWidth: 1,
            lineStyle: LineStyle.Dashed,
            priceLineVisible: false,
            lastValueVisible: false,
          });
        } catch (error) {
          console.error('Ошибка при добавлении линии для луча:', error);
          return;
        }

        const futureTime = Math.floor(Date.now() / 1000) + 30 * 24 * 60 * 60;
        raySeries.setData([
          { time, value: adjustedPrice },
          { time: futureTime, value: adjustedPrice },
        ]);

        this.chartStore.addDrawnLine({
          id: null,
          drawing_type: 'drawing.ray',
          symbol: this.chartStore.symbol,
          data: { price: adjustedPrice, start_time: time, color: '#FFD700', line_width: 1 },
          line: raySeries,
        });
        this.saveDrawingLine('drawing.ray', { price: adjustedPrice, start_time: time, color: '#FFD700', line_width: 1 });
        this.chartStore.setDrawingTool(null);
      } else if (this.chartStore.drawingTool === 'trendline') {
        if (!this.trendlineStart) {
          this.trendlineStart = { price: adjustedPrice, time };
        } else {
          let startPrice = this.trendlineStart.price;
          let endPrice = adjustedPrice;
          if (this.magnetEnabled) {
            startPrice = this.getMagnetPrice(candlestickObj.series, startPrice, this.trendlineStart.time);
            endPrice = this.getMagnetPrice(candlestickObj.series, endPrice, time);
          }

          let trendlineSeries;
          try {
            trendlineSeries = this.chart.addLineSeries({
              color: '#FFD700',
              lineWidth: 1,
              lineStyle: LineStyle.Solid,
              priceLineVisible: false,
              lastValueVisible: false,
            });
          } catch (error) {
            console.error('Ошибка при добавлении линии для тренда:', error);
            return;
          }

          trendlineSeries.setData([
            { time: this.trendlineStart.time, value: startPrice },
            { time, value: endPrice },
          ]);

          const trendlineData = {
            start_price: startPrice,
            end_price: endPrice,
            start_time: this.trendlineStart.time,
            end_time: time,
            color: '#FFD700',
            line_width: 1,
          };

          this.chartStore.addDrawnLine({
            id: null,
            drawing_type: 'drawing.trendline',
            symbol: this.chartStore.symbol,
            data: trendlineData,
            line: trendlineSeries,
          });

          this.saveDrawingLine('drawing.trendline', trendlineData);
          this.trendlineStart = null;
          this.chartStore.setDrawingTool(null);
        }
      } else {
        const line = this.chartStore.drawnLines.find((line) =>
          this.isPointNearLine(line, price, time)
        );
        if (line) {
          this.selectedLineIndex = line.id;
          this.openLineProperties(line);
        } else {
          this.selectedLineIndex = null;
        }
      }
    },
    getMagnetPrice(series, price, clickTime) {
      const data = series.data();
      if (data.length === 0) return price;

      const nearestCandle = data.reduce((closest, candle) => {
        const timeDiff = Math.abs(candle.time - clickTime);
        const closestTimeDiff = Math.abs(closest.time - clickTime);
        return timeDiff < closestTimeDiff ? candle : closest;
      }, data[0]);

      const prices = [
        nearestCandle.open,
        nearestCandle.high,
        nearestCandle.low,
        nearestCandle.close,
      ];
      return prices.reduce((closestPrice, p) => {
        return Math.abs(p - price) < Math.abs(closestPrice - price) ? p : closestPrice;
      }, prices[0]);
    },
    isPointNearLine(line, price, time) {
      const data = line.data;
      if (line.drawing_type === 'drawing.line') {
        return Math.abs(data.price - price) < 1.5;
      } else if (line.drawing_type === 'drawing.ray') {
        if (time < data.start_time) return false;
        return Math.abs(data.price - price) < 1.5;
      } else if (line.drawing_type === 'drawing.trendline') {
        const t = (time - data.start_time) / (data.end_time - data.start_time);
        if (t < 0 || t > 1) return false;
        const interpolatedPrice = data.start_price + t * (data.end_price - data.start_price);
        return Math.abs(price - interpolatedPrice) < 1.5;
      }
      return false;
    },
    saveDrawingLine(drawing_type, data) {
      const message = {
        event_type: 'save_drawing',
        data: {
          drawing_type,
          symbol: this.chartStore.symbol || 'BTCUSDT',
          data: JSON.stringify(data),
        },
      };
      if (this.chartStore.websocket && this.chartStore.websocket.readyState === WebSocket.OPEN) {
        this.chartStore.websocket.send(JSON.stringify(message));
      }
    },
    loadInitialDrawings() {
      if (!this.chart) return;
      const drawings = this.chartStore.drawnLines;
      if (drawings.length > 0) {
        this.loadDrawings(drawings);
      }
    },
    loadDrawings(drawings) {
      if (!this.chart) {
        console.warn('График не инициализирован, пропуск загрузки линий');
        return;
      }

      const candlestickObj = this.chartStore.chartObjects.find((obj) => obj.id === 'candlestick');
      if (!candlestickObj || !candlestickObj.visible) return;

      drawings.forEach(({ id, drawing_type, symbol, data }) => {
        let parsedData;
        try {
          parsedData = typeof data === 'string' ? JSON.parse(data) : data;
        } catch (error) {
          console.error('Ошибка парсинга данных drawing:', error);
          return;
        }

        try {
          if (drawing_type === 'drawing.line') {
            const line = candlestickObj.series.createPriceLine({
              price: parsedData.price,
              color: parsedData.color,
              lineWidth: parsedData.line_width,
              lineStyle: LineStyle.Dashed,
            });
            this.chartStore.addDrawnLine({
              id,
              drawing_type,
              symbol,
              data: parsedData,
              line,
            });
          } else if (drawing_type === 'drawing.ray') {
            let raySeries = this.chart.addLineSeries({
              color: parsedData.color,
              lineWidth: parsedData.line_width,
              lineStyle: LineStyle.Dashed,
              priceLineVisible: false,
              lastValueVisible: false,
            });
            const futureTime = Math.floor(Date.now() / 1000) + 30 * 24 * 60 * 60;
            raySeries.setData([
              { time: parsedData.start_time, value: parsedData.price },
              { time: futureTime, value: parsedData.price },
            ]);
            this.chartStore.addDrawnLine({
              id,
              drawing_type,
              symbol,
              data: parsedData,
              line: raySeries,
            });
          } else if (drawing_type === 'drawing.trendline') {
            let trendlineSeries = this.chart.addLineSeries({
              color: parsedData.color,
              lineWidth: parsedData.line_width,
              lineStyle: LineStyle.Solid,
              priceLineVisible: false,
              lastValueVisible: false,
            });
            trendlineSeries.setData([
              { time: parsedData.start_time, value: parsedData.start_price },
              { time: parsedData.end_time, value: parsedData.end_price },
            ]);
            this.chartStore.addDrawnLine({
              id,
              drawing_type,
              symbol,
              data: parsedData,
              line: trendlineSeries,
            });
          }
        } catch (error) {
          console.error(`Ошибка при загрузке ${drawing_type}:`, error);
        }
      });
    },
    handleDrawingDeleted({ id, drawing_type, symbol }) {
      if (!this.chart) return;

      const line = this.chartStore.drawnLines.find(
        (l) => l.id === id && l.drawing_type === drawing_type && l.symbol === symbol
      );
      if (line && line.line) {
        try {
          if (line.drawing_type === 'drawing.line') {
            const candlestickObj = this.chartStore.chartObjects.find((obj) => obj.id === 'candlestick');
            if (candlestickObj) {
              candlestickObj.series.removePriceLine(line.line);
            }
          } else {
            this.chart.removeSeries(line.line);
          }
        } catch (error) {
          console.error('Ошибка при удалении линии:', error);
        }
      }
      this.chartStore.removeDrawnLine(id);
    },
    handleDrawingSaved({ status, id, data }) {
      if (status === 'success') {
        const line = this.chartStore.drawnLines.find((l) => l.id === null && l.drawing_type === data?.drawing_type);
        if (line) {
          this.chartStore.updateDrawnLine(null, { ...line, id });
        }
      } else {
        console.error('Ошибка сохранения drawing:', data?.message);
      }
    },
    openLineProperties(line) {
      this.selectedLineIndex = line.id;
      this.selectedLine = { ...line, data: { ...line.data } };
      this.showPropertiesModal = true;
    },
    updateLineProperties() {
      if (!this.chart) {
        console.error('График не инициализирован');
        return;
      }

      const line = this.chartStore.drawnLines.find((l) => l.id === this.selectedLineIndex);
      if (line && line.line) {
        try {
          if (line.drawing_type === 'drawing.line') {
            const candlestickObj = this.chartStore.chartObjects.find((obj) => obj.id === 'candlestick');
            if (candlestickObj) {
              candlestickObj.series.removePriceLine(line.line);
            }
          } else {
            this.chart.removeSeries(line.line);
          }
        } catch (error) {
          console.error('Ошибка при удалении старой линии:', error);
        }
      }

      const candlestickObj = this.chartStore.chartObjects.find((obj) => obj.id === 'candlestick');
      let updatedLine;
      if (line.drawing_type === 'drawing.line') {
        try {
          updatedLine = candlestickObj.series.createPriceLine({
            price: this.selectedLine.data.price,
            color: this.selectedLine.data.color,
            lineWidth: this.selectedLine.data.line_width,
            lineStyle: LineStyle.Dashed,
          });
        } catch (error) {
          console.error('Ошибка при создании линии уровня:', error);
          return;
        }
      } else if (line.drawing_type === 'drawing.ray') {
        try {
          updatedLine = this.chart.addLineSeries({
            color: this.selectedLine.data.color,
            lineWidth: this.selectedLine.data.line_width,
            lineStyle: LineStyle.Dashed,
            priceLineVisible: false,
            lastValueVisible: false,
          });
          const futureTime = Math.floor(Date.now() / 1000) + 30 * 24 * 60 * 60;
          updatedLine.setData([
            { time: line.data.start_time, value: this.selectedLine.data.price },
            { time: futureTime, value: this.selectedLine.data.price },
          ]);
        } catch (error) {
          console.error('Ошибка при создании линии луча:', error);
          return;
        }
      } else if (line.drawing_type === 'drawing.trendline') {
        try {
          updatedLine = this.chart.addLineSeries({
            color: this.selectedLine.data.color,
            lineWidth: this.selectedLine.data.line_width,
            lineStyle: LineStyle.Solid,
            priceLineVisible: false,
            lastValueVisible: false,
          });
          updatedLine.setData([
            { time: line.data.start_time, value: this.selectedLine.data.start_price },
            { time: line.data.end_time, value: this.selectedLine.data.end_price },
          ]);
        } catch (error) {
          console.error('Ошибка при создании линии тренда:', error);
          return;
        }
      }

      this.chartStore.updateDrawnLine(this.selectedLineIndex, {
        ...line,
        data: { ...this.selectedLine.data },
        line: updatedLine,
      });

      this.saveDrawingLine(line.drawing_type, this.selectedLine.data);
      this.showPropertiesModal = false;
    },
  },
};
</script>