<template>
  <div class="container mx-auto p-4">
    <h1 class="text-2xl font-bold mb-4">Binance WebSocket Data</h1>
    <div class="mb-4 flex space-x-4">
      <div>
        <label for="symbol" class="mr-2">Symbol:</label>
        <select id="symbol" v-model="selectedSymbol" @change="updateSubscription">
          <option value="BTCUSDT">BTCUSDT</option>
          <option value="ETHUSDT">ETHUSDT</option>
        </select>
      </div>
      <div>
        <label for="interval" class="mr-2">Interval:</label>
        <select id="interval" v-model="selectedInterval" @change="updateSubscription">
          <option value="1m">1m</option>
          <option value="5m">5m</option>
          <option value="15m">15m</option>
        </select>
      </div>
    </div>
    <div id="chart" class="w-full h-96"></div>
  </div>
</template>

<script>
import { createChart } from 'lightweight-charts';

export default {
  name: 'App',
  data() {
    return {
      chart: null,
      candlestickSeries: null,
      ws: null,
      candles: [],
      lastCandleTime: null,
      selectedSymbol: 'BTCUSDT',
      selectedInterval: '1m',
    };
  },
  mounted() {
    this.$nextTick(() => {
      this.initChart();
      this.connectWebSocket();
    });
  },
  beforeUnmount() {
    if (this.ws) {
      this.ws.close();
    }
    if (this.chart) {
      this.chart.remove();
    }
  },
  methods: {
    initChart() {
      const chartContainer = document.getElementById('chart');
      if (!chartContainer) {
        console.error('Контейнер для графика не найден!');
        return;
      }

      try {
        this.chart = createChart(chartContainer, {
          width: chartContainer.clientWidth,
          height: 384,
          layout: {
            backgroundColor: '#ffffff',
            textColor: '#333',
          },
          grid: {
            vertLines: {
              color: '#f0f0f0',
            },
            horzLines: {
              color: '#f0f0f0',
            },
          },
          timeScale: {
            timeVisible: true,
            secondsVisible: true,
          },
        });

        console.log('this.chart:', this.chart);

        this.candlestickSeries = this.chart.addCandlestickSeries({
          upColor: '#26a69a',
          downColor: '#ef5350',
          borderVisible: false,
          wickUpColor: '#26a69a',
          wickDownColor: '#ef5350',
        });

        console.log('this.candlestickSeries:', this.candlestickSeries);

        window.addEventListener('resize', () => {
          this.chart.resize(chartContainer.clientWidth, 384);
        });
      } catch (error) {
        console.error('Ошибка при инициализации графика:', error);
      }
    },
    connectWebSocket() {
      this.ws = new WebSocket('ws://127.0.0.1:3000/ws');

      this.ws.onopen = () => {
        console.log('Подключено к локальному WebSocket');
        // Отправляем начальную подписку
        this.updateSubscription();
      };

      this.ws.onmessage = (event) => {
        const message = JSON.parse(event.data);
        console.log('Получено сообщение:', message);
        if (message.e === 'kline') {
          const kline = message.k;
          console.log('kline:', kline);

          const candle = {
            time: kline.t / 1000,
            open: parseFloat(kline.o),
            high: parseFloat(kline.h),
            low: parseFloat(kline.l),
            close: parseFloat(kline.c),
          };

          console.log('Обновление свечи:', candle);

          if (
            typeof candle.time !== 'number' ||
            isNaN(candle.open) ||
            isNaN(candle.high) ||
            isNaN(candle.low) ||
            isNaN(candle.close)
          ) {
            console.error('Некорректные данные свечи:', candle);
            return;
          }

          if (this.candlestickSeries) {
            if (this.lastCandleTime === null || candle.time > this.lastCandleTime) {
              this.candles.push(candle);
              this.candlestickSeries.setData(this.candles);
              console.log('Новая свеча добавлена:', candle);
            } else {
              this.candles[this.candles.length - 1] = candle;
              this.candlestickSeries.setData(this.candles);
              console.log('Свеча обновлена:', candle);
            }

            this.lastCandleTime = candle.time;

            this.chart.timeScale().fitContent();
          } else {
            console.error('candlestickSeries не инициализирован!');
          }
        }
      };

      this.ws.onerror = (error) => {
        console.error('Ошибка WebSocket:', error);
      };

      this.ws.onclose = () => {
        console.log('WebSocket закрыт');
      };
    },
    updateSubscription() {
      if (this.ws && this.ws.readyState === WebSocket.OPEN) {
        // Очищаем текущие данные
        this.candles = [];
        this.lastCandleTime = null;
        this.candlestickSeries.setData(this.candles);

        // Отправляем новую подписку
        const subscription = {
          symbol: this.selectedSymbol.toLowerCase(),
          interval: this.selectedInterval,
        };
        this.ws.send(JSON.stringify(subscription));
        console.log('Отправлена подписка:', subscription);
      }
    },
  },
};
</script>