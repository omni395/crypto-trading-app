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
        <!-- Компонент для свечей и объёмов -->
        <CandlesAndVolume />

        <!-- Кнопка "Вернуться к текущей свече" -->
        <button
          v-if="chartStore.chart"
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
            />
          </svg>
        </button>
      </div>
    </div>

    <!-- Дерево объектов -->
    <ObjectTree
      v-if="showModal"
      :show-modal="showModal"
      :chart-objects="chartStore.chartObjects"
      :drawn-lines="chartStore.drawnLines"
      @update:show-modal="showModal = $event"
      @toggle-visibility="toggleVisibility"
      @edit-line="openLineProperties"
      @remove-line="removeDrawnLine"
    />
  </div>
</template>

<script>
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
    };
  },
  async mounted() {
    try {
      await this.setupWebSocket();
      this.requestHistoricalData();
    } catch (error) {
      console.error('Ошибка в хуке mounted:', error);
    }
  },
  methods: {
    async setupWebSocket() {
      return new Promise((resolve, reject) => {
        try {
          const websocket = new WebSocket('ws://127.0.0.1:3000/ws');
          this.chartStore.setWebsocket(websocket);
          
          websocket.onopen = () => {
            this.subscribe();
            resolve();
          };

          websocket.onerror = (error) => {
            console.error('WebSocket error:', error);
            reject(error);
          };

          websocket.onmessage = (event) => {
            try {
              const message = JSON.parse(event.data);
              this.handleWebSocketMessage(message);
            } catch (error) {
              console.error('Ошибка при обработке сообщения WebSocket:', error);
            }
          };

          websocket.onclose = () => {
            console.log('WebSocket соединение закрыто');
          };
        } catch (error) {
          console.error('Ошибка при установке WebSocket:', error);
          reject(error);
        }
      });
    },
    subscribe() {
      const websocket = this.chartStore.websocket;
      if (websocket && websocket.readyState === WebSocket.OPEN) {
        websocket.send(JSON.stringify({
          event_type: 'subscribe',
          symbol: 'BTCUSDT',
          interval: '1m'
        }));
      }
    },
    requestHistoricalData() {
      const websocket = this.chartStore.websocket;
      if (websocket && websocket.readyState === WebSocket.OPEN) {
        websocket.send(JSON.stringify({
          event_type: 'get_historical_data',
          symbol: 'BTCUSDT',
          interval: '1m'
        }));
      }
    },
    handleWebSocketMessage(message) {
      switch (message.event_type) {
        case 'subscribed':
          console.log('Подписка подтверждена:', message);
          break;
        case 'kline':
          this.handleHistoricalData(message);
          break;
        case 'historical_data':
          this.handleHistoricalData(message, true);
          break;
        default:
          console.log('Неизвестный тип события:', message.event_type);
      }
    },
    handleHistoricalData(message, isInitialLoad = false) {
      // Проверяем тип сообщения
      if (message.event_type === 'kline') {
        // Обрабатываем одиночную свечу
        const candle = {
          time: message.start_time / 1000,
          open: parseFloat(message.open),
          high: parseFloat(message.high),
          low: parseFloat(message.low),
          close: parseFloat(message.close)
        };

        const volume = {
          time: message.start_time / 1000,
          value: parseFloat(message.volume || 0),
          color: parseFloat(message.close) >= parseFloat(message.open) ? '#26a69a' : '#ef5350'
        };

        this.chartStore.setHistoricalData({
          candlestickData: [candle],
          volumeData: [volume],
          isInitialLoad: false
        });
        return;
      }

      // Обработка исторических данных
      if (!message.candlestick_data && !message.volume_data) {
        console.warn('No valid data in message:', message);
        return;
      }

      const candlestickData = message.candlestick_data?.map(candle => ({
        time: candle.time,
        open: parseFloat(candle.open),
        high: parseFloat(candle.high),
        low: parseFloat(candle.low),
        close: parseFloat(candle.close)
      }));

      const volumeData = message.volume_data?.map(volume => ({
        time: volume.time,
        value: parseFloat(volume.value || 0),
        color: parseFloat(volume.close) >= parseFloat(volume.open) ? '#26a69a' : '#ef5350'
      }));

      this.chartStore.setHistoricalData({
        candlestickData,
        volumeData,
        isInitialLoad
      });
    },
    scrollToLatestCandle() {
      if (this.chartStore.chart) {
        this.chartStore.chart.timeScale().scrollToRealTime();
      }
    },
    toggleVisibility(obj) {
      obj.series.applyOptions({ visible: !obj.visible });
      obj.visible = !obj.visible;
    },
    removeDrawnLine(id) {
      this.chartStore.removeDrawnLine(id);
    },
    openLineProperties(line) {
      // Implement line properties dialog
    },
    handleChartClickInDrawings(clickData) {
      // Handle chart click in drawings
    },
  },
};
</script>

<style>
.bg-custom {
  background-color: rgb(19, 23, 34);
  color: white;
}
</style>