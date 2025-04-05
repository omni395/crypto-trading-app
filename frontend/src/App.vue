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
      currentRequest: null, // Храним текущий запрос
      expectedCandles: 1000, // Ожидаемое количество свечей (по лимиту Binance API)
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

      // Отслеживаем изменение видимого временного диапазона
      this.chart.timeScale().subscribeVisibleTimeRangeChange(() => {
        const range = this.chart.timeScale().getVisibleRange();
        if (range && this.earliestTime && !this.isLoading) {
          const earliestVisibleTime = range.from;
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
        // Проверяем, что все значения корректны
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

        // Проверяем, получили ли мы ожидаемое количество свечей
        if (this.currentRequest && this.historicalData.length >= this.expectedCandles) {
          console.log('Все свечи получены, обновляем график');
          // Сортируем данные по времени (по возрастанию) и обновляем график
          this.historicalData.sort((a, b) => a.time - b.time);
          if (this.candlestickSeries) {
            this.candlestickSeries.setData(this.historicalData);
            console.log('Обновлены исторические данные в график');
            // Обновляем earliestTime и latestTime
            this.earliestTime = this.historicalData[0].time; // Самая ранняя свеча
            this.latestTime = this.historicalData[this.historicalData.length - 1].time; // Самая поздняя свеча
            this.isLoading = false;

            // Фокусируемся на последней свече
            const visibleRange = {
              from: this.historicalData[this.historicalData.length - 1].time - 60 * 60, // На час раньше последней свечи
              to: this.historicalData[this.historicalData.length - 1].time + 60 * 60, // На час позже последней свечи
            };
            this.chart.timeScale().setVisibleRange(visibleRange);
          } else {
            console.error('Серия свечей не инициализирована для исторических данных');
          }
          this.currentRequest = null; // Сбрасываем текущий запрос
        }
      } else if (message.event_type === 'kline') {
        // Временно закомментировано для тестов
        // const kline = message.kline;
        // const candlestickData = {
        //   time: kline.start_time / 1000,
        //   open: parseFloat(kline.open),
        //   high: parseFloat(kline.high),
        //   low: parseFloat(kline.low),
        //   close: parseFloat(kline.close),
        // };
        // console.log('Подготовлены данные для графика:', candlestickData);
        // if (this.candlestickSeries) {
        //   this.candlestickSeries.update(candlestickData);
        //   console.log('Обновлена серия свечей');
        //   this.latestTime = candlestickData.time;
        // } else {
        //   console.error('Серия свечей не инициализирована');
        // }
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
      const now = Math.floor(new Date('2024-02-07').getTime() / 1000); // 7 февраля 2024
      const sevenDaysAgo = now - 7 * 24 * 60 * 60; // 31 января 2024
      console.log('Текущие значения времени:', { now, sevenDaysAgo });
      this.historicalData = []; // Очищаем перед новым запросом
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
        this.currentRequest = request; // Сохраняем текущий запрос
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