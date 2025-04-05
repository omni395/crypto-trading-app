<template>
  <div class="app">
    <h1>Crypto Trading App</h1>
    <div id="chart" ref="chartContainer"></div>
  </div>
</template>

<script>
import { createChart, ColorType } from 'lightweight-charts';

export default {
  name: 'App',
  data() {
    return {
      chart: null,
      candlestickSeries: null,
      websocket: null,
      selectedSymbol: 'btcusdt',
      selectedInterval: '1m',
      historicalData: [], // Для хранения исторических данных
    };
  },
  mounted() {
    this.initChart();
    this.setupWebSocket();
  },
  beforeDestroy() {
    if (this.websocket) {
      this.websocket.close();
    }
    if (this.chart) {
      this.chart.remove();
    }
  },
  methods: {
    initChart() {
      const chartContainer = this.$refs.chartContainer;
      this.chart = createChart(chartContainer, {
        width: chartContainer.clientWidth,
        height: 500,
        layout: {
          background: { type: ColorType.Solid, color: '#000000' },
          textColor: '#d1d4dc',
        },
        grid: {
          vertLines: { color: 'rgba(197, 203, 206, 0.1)' },
          horzLines: { color: 'rgba(197, 203, 206, 0.1)' },
        },
        timeScale: {
          timeVisible: true,
          secondsVisible: false,
        },
      });

      this.candlestickSeries = this.chart.addCandlestickSeries({
        upColor: '#26a69a',
        downColor: '#ef5350',
        borderVisible: false,
        wickUpColor: '#26a69a',
        wickDownColor: '#ef5350',
      });

      console.log('График инициализирован:', this.chart);
      console.log('Серия свечей инициализирована:', this.candlestickSeries);
      console.log('Размеры контейнера:', chartContainer.clientWidth, chartContainer.clientHeight);
    },
    setupWebSocket() {
      this.websocket = new WebSocket('ws://localhost:3000/ws');
      this.websocket.onopen = () => {
        console.log('Подключено к локальному WebSocket');
        this.updateSubscription();
      };

      this.websocket.onmessage = (event) => {
        const message = JSON.parse(event.data);
        console.log('Получено сообщение:', message);
        this.handleWebSocketMessage(message);
      };

      this.websocket.onclose = () => {
        console.log('WebSocket закрыт');
      };

      this.websocket.onerror = (error) => {
        console.error('WebSocket ошибка:', error);
      };
    },
    handleWebSocketMessage(message) {
      if (message.event_type === 'historical_kline') {
        const candlestickData = {
          time: message.time,
          open: message.open,
          high: message.high,
          low: message.low,
          close: message.close,
        };
        this.historicalData.push(candlestickData);
        console.log('Добавлены исторические данные:', candlestickData);

        // Сортируем данные по времени и устанавливаем их в график
        this.historicalData.sort((a, b) => a.time - b.time);
        if (this.candlestickSeries) {
          this.candlestickSeries.setData(this.historicalData);
          console.log('Установлены исторические данные в график');
        } else {
          console.error('Серия свечей не инициализирована для исторических данных');
        }
      } else if (message.event_type === 'kline') {
        const kline = message.kline;
        const candlestickData = {
          time: kline.start_time / 1000,
          open: parseFloat(kline.open),
          high: parseFloat(kline.high),
          low: parseFloat(kline.low),
          close: parseFloat(kline.close),
        };
        console.log('Подготовлены данные для графика:', candlestickData);
        if (this.candlestickSeries) {
          this.candlestickSeries.update(candlestickData);
          console.log('Обновлена серия свечей');
        } else {
          console.error('Серия свечей не инициализирована');
        }
      }
    },
    updateSubscription() {
      if (this.websocket && this.websocket.readyState === WebSocket.OPEN) {
        const subscription = {
          symbol: this.selectedSymbol,
          interval: this.selectedInterval,
        };
        this.websocket.send(JSON.stringify(subscription));
        console.log('Отправлена подписка:', subscription);
      }
    },
  },
};
</script>

<style scoped>
.app {
  text-align: center;
  padding: 20px;
}

#chart {
  margin: 0 auto;
}
</style>