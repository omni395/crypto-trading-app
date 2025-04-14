<!-- src/components/Chart.vue -->
<template>
  <div class="bg-custom">
    <div class="flex">
      <!-- Панель инструментов для рисования (Drawings.vue) -->
      <Drawings
        v-if="chartStore.chart"
        :chart="chartStore.chart"
        @chart-click="handleChartClickInDrawings"
        @edit-line="openLineProperties"
      />

      <!-- Контейнер графика и дочерние компоненты -->
      <div class="relative flex-1">
        <div id="chart-container" ref="chartContainer" class="w-full h-[600px]"></div>

        <!-- Компонент для свечей и объёмов -->
        <CandlesAndVolume
          v-if="chartStore.chart"
          :chart="chartStore.chart"
        />

        <!-- Кнопка "Вернуться к текущей свече" -->
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

    <!-- Дерево объектов -->
    <ObjectTree
      :show-modal="showModal"
      :chart-objects="chartStore.chartObjects"
      :drawn-lines="chartStore.drawnLines"
      @update:show-modal="showModal = $event"
      @toggle-visibility="toggleVisibility"
      @remove-line="removeDrawnLine"
      @edit-line="openLineProperties"
    />
  </div>
</template>

<script>
import { createChart, CrosshairMode } from 'lightweight-charts';
import { useChartStore } from '@/stores/chart';
import CandlesAndVolume from './chart/CandlesAndVolume.vue';
import Drawings from './chart/Drawings.vue';
import ObjectTree from './chart/ObjectTree.vue';

export default {
  name: 'Chart',
  components: {
    CandlesAndVolume,
    Drawings,
    ObjectTree,
  },
  setup() {
    const chartStore = useChartStore();
    return { chartStore };
  },
  data() {
    return {
      showModal: false,
      clickData: null, // Для хранения данных клика
    };
  },
  async mounted() {
    try {
      this.initChart();
      await this.setupWebSocket();
      this.requestHistoricalData();
      this.loadDrawingLines();
    } catch (error) {
      console.error('Ошибка в хуке mounted:', error);
    }
  },
  methods: {
    initChart() {
      const chartContainer = this.$refs.chartContainer;
      if (!chartContainer) {
        console.error('Контейнер графика не найден');
        return;
      }

      const width = chartContainer.clientWidth || window.innerWidth;
      const height = chartContainer.clientHeight || 600;
      if (width === 0 || height === 0) {
        console.error('Недопустимые размеры контейнера:', { width, height });
        return;
      }

      const chart = createChart(chartContainer, {
        width,
        height,
        layout: {
          background: { color: 'rgb(32, 41, 56)' },
          textColor: '#DDD',
        },
        grid: {
          vertLines: { color: '#444' },
          horzLines: { color: '#444' },
        },
        crosshair: {
          mode: CrosshairMode.Normal,
          vertLine: {
            labelVisible: false,
          },
          horzLine: {
            labelVisible: false,
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

      // Подписка на изменение видимого диапазона для подгрузки исторических данных
      chart.timeScale().subscribeVisibleTimeRangeChange(() => {
        const timeRange = chart.timeScale().getVisibleLogicalRange();
        if (timeRange && timeRange.from && timeRange.from < 0 && !this.chartStore.isLoading) {
          this.loadMoreHistoricalData();
        }
      });

      // Подписка на клики (централизованно)
      chart.subscribeClick((param) => {
        if (!param.point) return;

        const candlestickObj = this.chartStore.chartObjects.find((obj) => obj.id === 'candlestick');
        if (!candlestickObj || !candlestickObj.visible) return;

        const price = candlestickObj.series.coordinateToPrice(param.point.y);
        const time = chart.timeScale().coordinateToTime(param.point.x);

        // Сохраняем данные клика и передаём их в Drawings.vue
        this.clickData = { price, time, point: param.point };
        this.$refs.drawings.handleChartClick(this.clickData);
      });

      // Отмена рисования по правому клику
      chartContainer.addEventListener('contextmenu', (e) => {
        e.preventDefault();
        this.chartStore.setDrawingTool(null);
      });
    },
    async setupWebSocket() {
      return new Promise((resolve) => {
        const websocket = new WebSocket('ws://127.0.0.1:3000/ws');
        this.chartStore.setWebsocket(websocket);
        websocket.onopen = () => {
          this.subscribe();
          resolve();
        };
        websocket.onmessage = (event) => {
          const message = JSON.parse(event.data);
          switch (message.event_type) {
            case 'subscribed':
              console.log('Подписка подтверждена:', message);
              break;
            case 'kline':
              this.handleWebSocketMessage(message);
              break;
            case 'historical_data':
              this.handleHistoricalData(message, message.is_initial_load);
              break;
            case 'drawings_loaded':
            case 'drawing_added':
            case 'drawing_deleted':
            case 'drawing_saved':
              // Эти события обрабатываются в Drawings.vue через chartStore
              break;
          }
        };
        websocket.onerror = (error) => {
          console.error('Ошибка WebSocket:', error);
        };
        websocket.onclose = () => {
          console.log('WebSocket закрыт, переподключение...');
          setTimeout(() => this.setupWebSocket(), 5000);
        };
      });
    },
    subscribe() {
      const subscription = {
        event_type: 'subscribe',
        symbol: this.chartStore.symbol || 'BTCUSDT',
        interval: this.chartStore.interval || '1m',
        streams: ['kline'],
      };
      if (this.chartStore.websocket && this.chartStore.websocket.readyState === WebSocket.OPEN) {
        this.chartStore.websocket.send(JSON.stringify(subscription));
      }
    },
    requestHistoricalData() {
      const now = Math.floor(Date.now() / 1000);
      const startOfYesterday = Math.floor(new Date().setHours(0, 0, 0, 0) / 1000) - 24 * 60 * 60;

      const message = {
        event_type: 'load_historical_data',
        data: {
          symbol: this.chartStore.symbol || 'BTCUSDT',
          interval: this.chartStore.interval || '1m',
          start_time: startOfYesterday * 1000,
          end_time: now * 1000,
          is_initial_load: true,
        },
      };

      if (this.chartStore.websocket && this.chartStore.websocket.readyState === WebSocket.OPEN) {
        this.chartStore.websocket.send(JSON.stringify(message));
      }

      this.chartStore.setEarliestTime(startOfYesterday);
    },
    loadMoreHistoricalData() {
      if (this.chartStore.isLoading) return;
      this.chartStore.setLoading(true);

      const candlestickObj = this.chartStore.chartObjects.find((obj) => obj.id === 'candlestick');
      if (!candlestickObj) {
        this.chartStore.setLoading(false);
        return;
      }

      const currentRange = this.chartStore.chart.timeScale().getVisibleRange();
      const existingData = candlestickObj.series.data();
      if (existingData.length > 0) {
        const earliestCandleTime = Math.min(...existingData.map((c) => c.time));
        const newEndTime = earliestCandleTime * 1000;
        const newStartTime = newEndTime - 60 * 60 * 24 * 1000;

        if (newStartTime < 0) {
          this.chartStore.setLoading(false);
          return;
        }

        const message = {
          event_type: 'load_historical_data',
          data: {
            symbol: this.chartStore.symbol || 'BTCUSDT',
            interval: this.chartStore.interval || '1m',
            start_time: newStartTime,
            end_time: newEndTime,
            is_initial_load: false,
          },
        };

        if (this.chartStore.websocket && this.chartStore.websocket.readyState === WebSocket.OPEN) {
          this.chartStore.websocket.send(JSON.stringify(message));
        }

        if (currentRange) {
          this.chartStore.chart.timeScale().setVisibleRange(currentRange);
        }
        this.chartStore.setEarliestTime(newStartTime / 1000);
      }

      this.chartStore.setLoading(false);
    },
    handleWebSocketMessage(message) {
      const candle = {
        time: message.time,
        open: parseFloat(message.open),
        high: parseFloat(message.high),
        low: parseFloat(message.low),
        close: parseFloat(message.close),
      };
      const volume = {
        time: message.time,
        value: parseFloat(message.volume || 0),
        color: parseFloat(message.close) >= parseFloat(message.open) ? '#26a69a' : '#ef5350',
      };

      this.chartStore.setLastCandle({
        open: parseFloat(message.open),
        close: parseFloat(message.close),
      });

      // Эти данные будут обрабатываться в CandlesAndVolume.vue через chartStore
    },
    handleHistoricalData(message, isInitialLoad = false) {
      if (!this.chartStore.chart) return;

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
        color: parseFloat(kline.close) >= parseFloat(kline.open) ? '#26a69a' : '#ef5350',
      }));

      // Данные передаются в CandlesAndVolume.vue через chartStore
      this.chartStore.setHistoricalData({ candlestickData, volumeData });

      if (isInitialLoad) {
        this.chartStore.chart.timeScale().fitContent();
      }
    },
    scrollToLatestCandle() {
      if (this.chartStore.chart && this.chartStore.lastCandle) {
        this.chartStore.chart.timeScale().scrollToPosition(this.chartStore.lastCandle.time, false);
      }
    },
    loadDrawingLines() {
      const message = {
        event_type: 'load_drawings',
        data: {
          symbol: this.chartStore.symbol || 'BTCUSDT',
        },
      };
      if (this.chartStore.websocket && this.chartStore.websocket.readyState === WebSocket.OPEN) {
        this.chartStore.websocket.send(JSON.stringify(message));
      }
    },
    toggleVisibility(obj) {
      this.chartStore.toggleObjectVisibility(obj.id);
      const updatedObj = this.chartStore.chartObjects.find((o) => o.id === obj.id);
      updatedObj.series.applyOptions({ visible: updatedObj.visible });
      if (updatedObj.id === 'volume' && updatedObj.visible) {
        this.$refs.candlesAndVolume.updateVolumeLabel();
      }
    },
    removeDrawnLine(id) {
      const line = this.chartStore.drawnLines.find((l) => l.id === id);
      if (line && line.line) {
        try {
          if (line.drawing_type === 'drawing.line') {
            const candlestickObj = this.chartStore.chartObjects.find((obj) => obj.id === 'candlestick');
            if (candlestickObj) {
              candlestickObj.series.removePriceLine(line.line);
            }
          } else {
            this.chartStore.chart.removeSeries(line.line);
          }
          this.deleteDrawingLine(line.id, line.drawing_type, line.symbol || this.chartStore.symbol);
        } catch (error) {
          console.error('Ошибка при удалении линии:', error);
        }
      }
      this.chartStore.removeDrawnLine(id);
    },
    deleteDrawingLine(id, drawing_type, symbol) {
      const message = {
        event_type: 'delete_drawing',
        data: {
          id,
          drawing_type,
          symbol,
        },
      };
      if (this.chartStore.websocket && this.chartStore.websocket.readyState === WebSocket.OPEN) {
        this.chartStore.websocket.send(JSON.stringify(message));
      }
    },
    openLineProperties(line) {
      this.$refs.drawings.openLineProperties(line);
    },
    handleChartClickInDrawings(clickData) {
      this.$refs.drawings.handleChartClick(clickData);
    },
  },
};
</script>