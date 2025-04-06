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
      earliestTime: null,
      latestTime: null,
      isLoading: false,
      currentRequest: null,
      expectedCandles: 1000,
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
          rightOffset: 10,
          fixLeftEdge: false,
          fixRightEdge: false,
        },
      });

      this.candlestickSeries = this.chart.addCandlestickSeries({
        upColor: '#26a69a',
        downColor: '#ef5350',
        borderVisible: false,
        wickUpColor: '#26a69a',
        wickDownColor: '#ef5350',
      });

      this.chart.timeScale().subscribeVisibleTimeRangeChange(() => {
        const range = this.chart.timeScale().getVisibleRange();
        console.log('Видимый диапазон:', range);
        if (range && this.earliestTime && !this.isLoading) {
          const earliestVisibleTime = range.from;
          console.log('Сравнение:', { earliestVisibleTime, earliestTime: this.earliestTime });
          if (earliestVisibleTime && earliestVisibleTime < this.earliestTime) {
            console.log('Прокрутка влево, подгружаем данные');
            this.loadMoreHistoricalData();
          }
        }
      });

      console.log('График инициализирован:', this.chart);
      console.log('Серия свечей инициализирована:', this.candlestickSeries);
      console.log('Размеры контейнера:', chartContainer.clientWidth, chartContainer.clientHeight);
    },
    setupWebSocket() {
      this.websocket = new WebSocket('ws://' + window.location.host + '/ws');
      this.websocket.onopen = () => {
        console.log('Подключено к локальному WebSocket');
        this.updateSubscription();
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
        const open = parseFloat(message.open);
        const high = parseFloat(message.high);
        const low = parseFloat(message.low);
        const close = parseFloat(message.close);
        const time = message.time;

        if (typeof time !== 'number' || isNaN(time)) {
          console.warn('Некорректное значение time:', time);
          return;
        }

        if (
          isNaN(open) || open <= 0 ||
          isNaN(high) || high <= 0 ||
          isNaN(low) || low <= 0 ||
          isNaN(close) || close <= 0
        ) {
          console.warn('Пропущена некорректная свеча:', { time, open, high, low, close });
          return;
        }

        const candlestickData = {
          time: time,
          open: open,
          high: high,
          low: low,
          close: close,
        };
        this.historicalData.push(candlestickData);
        console.log('Добавлены исторические данные:', candlestickData);
        console.log('Текущее количество свечей:', this.historicalData.length);

        if (this.currentRequest && (
          this.historicalData.length >= this.expectedCandles ||
          time >= (this.currentRequest.end_time / 1000) - 60
        )) {
          console.log('Все свечи получены, обновляем график');
          this.historicalData.sort((a, b) => a.time - b.time);
          if (this.candlestickSeries) {
            this.candlestickSeries.setData(this.historicalData);
            console.log('Обновлены исторические данные в график');
            this.earliestTime = this.historicalData[0].time;
            this.latestTime = this.historicalData[this.historicalData.length - 1].time;
            this.isLoading = false;

            const visibleRange = {
              from: this.historicalData[this.historicalData.length - 1].time - 60 * 60,
              to: this.historicalData[this.historicalData.length - 1].time + 60 * 60,
            };
            this.chart.timeScale().setVisibleRange(visibleRange);
          } else {
            console.error('Серия свечей не инициализирована для исторических данных');
          }
          this.currentRequest = null;
        }
      } else if (message.event_type === 'kline') {
        // Временно закомментировано для тестов
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
      const now = Math.floor(new Date().getTime() / 1000); // Текущая дата: 5 апреля 2025
      const intervalSeconds = this.selectedInterval === '1m' ? 60 : 0; // 60 секунд для 1m
      const periodsBack = this.expectedCandles * intervalSeconds; // 1000 свечей назад
      const startTime = now - periodsBack; // Время 1000 свечей назад
      console.log('Текущие значения времени:', { now, startTime });
      this.historicalData = [];
      this.requestHistoricalData(startTime * 1000, now * 1000); // Конвертируем в миллисекунды
    },
    loadMoreHistoricalData() {
      if (this.isLoading) {
        console.log('Подгрузка заблокирована, isLoading = true');
        return;
      }
      this.isLoading = true;
      console.log('Запрашиваем более ранние данные');

      const endTime = this.earliestTime * 1000;
      const startTime = endTime - 7 * 24 * 60 * 60 * 1000;
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
        this.currentRequest = request;
        this.websocket.send(JSON.stringify(request));
        console.log('Запрошены исторические данные:', request);
      } else {
        console.error('WebSocket не открыт');
        this.isLoading = false;
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