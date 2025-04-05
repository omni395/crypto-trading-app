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
      historicalData: [],
      earliestTime: null, // Самое раннее время на графике
      latestTime: null,  // Самое позднее время на графике
      isLoading: false,  // Флаг загрузки
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

      // Отслеживаем изменение видимого диапазона
      this.chart.timeScale().subscribeVisibleLogicalRangeChange(() => {
        const range = this.chart.timeScale().getVisibleLogicalRange();
        if (range && this.earliestTime && !this.isLoading) {
          const earliestVisibleTime = this.chart.timeScale().logicalToTime(range.from);
          if (earliestVisibleTime && earliestVisibleTime < this.earliestTime) {
            this.loadMoreHistoricalData();
          }
        }
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
        // Загружаем данные за последние 7 дней
        this.loadInitialHistoricalData();
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

        // Сортируем данные и обновляем график
        this.historicalData.sort((a, b) => a.time - b.time);
        if (this.candlestickSeries) {
          this.candlestickSeries.setData(this.historicalData);
          console.log('Обновлены исторические данные в график');
          // Обновляем earliestTime и latestTime
          this.earliestTime = this.historicalData[0].time;
          this.latestTime = this.historicalData[this.historicalData.length - 1].time;
          this.isLoading = false;
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
          // Обновляем latestTime
          this.latestTime = candlestickData.time;
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
    loadInitialHistoricalData() {
      const now = Math.floor(Date.now() / 1000); // Текущее время в секундах
      const sevenDaysAgo = now - 7 * 24 * 60 * 60; // 7 дней назад в секундах
      this.requestHistoricalData(sevenDaysAgo * 1000, now * 1000); // Конвертируем в миллисекунды
    },
    loadMoreHistoricalData() {
      if (this.isLoading) return;
      this.isLoading = true;

      const endTime = this.earliestTime * 1000; // В миллисекундах
      const startTime = endTime - 7 * 24 * 60 * 60 * 1000; // Загружаем ещё 7 дней назад
      this.requestHistoricalData(startTime, endTime);
    },
    requestHistoricalData(startTime, endTime) {
      if (this.websocket && this.websocket.readyState === WebSocket.OPEN) {
        const request = {
          symbol: this.selectedSymbol,
          interval: this.selectedInterval,
          start_time: startTime,
          end_time: endTime,
        };
        this.websocket.send(JSON.stringify(request));
        console.log('Запрошены исторические данные:', request);
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