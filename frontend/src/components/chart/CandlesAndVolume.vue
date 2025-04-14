<!-- src/components/chart/CandlesAndVolume.vue -->
<template>
  <div ref="chartContainer" class="w-full h-[600px]">
  </div>
</template>

<script>
import { useChartStore } from '@/stores/chart';

export default {
  name: 'CandlesAndVolume',
  setup() {
    const chartStore = useChartStore();
    return { chartStore };
  },
  data() {
    return {
      chart: null,
      candlestickSeries: null,
      volumeSeries: null,
    };
  },
  async mounted() {
    try {
      console.log('Mounting CandlesAndVolume component');
      await this.initializeChart();
      this.setupChartObjects();
      this.updateDataFromStore();
    } catch (error) {
      console.error('Ошибка при инициализации графика:', error);
    }
  },
  beforeUnmount() {
    if (this.chart) {
      this.chart.remove();
    }
    window.removeEventListener('resize', this.handleResize);
  },
  methods: {
    async initializeChart() {
      const container = this.$refs.chartContainer;
      console.log('Initializing chart with container:', container);
      const chartOptions = {
        layout: {
          background: { color: '#1E222D' },
          textColor: '#DDD',
        },
        grid: {
          vertLines: { color: '#2B2B43' },
          horzLines: { color: '#2B2B43' },
        },
        crosshair: {
          mode: 0,
        },
        rightPriceScale: {
          borderColor: '#2B2B43',
        },
        timeScale: {
          borderColor: '#2B2B43',
          timeVisible: true,
          secondsVisible: false,
        },
        width: container.clientWidth || 800,
        height: container.clientHeight || 600,
      };

      this.chart = window.LightweightCharts.createChart(container, chartOptions);
      console.log('Chart created:', this.chart);
      this.chartStore.setChart(this.chart);

      this.chart.subscribeCrosshairMove(this.handleCrosshairMove);
      window.addEventListener('resize', this.handleResize);
    },
    setupChartObjects() {
      console.log('Setting up chart objects');
      
      this.candlestickSeries = this.chart.addSeries(window.LightweightCharts.CandlestickSeries, {
        upColor: '#26a69a',
        downColor: '#ef5350',
        borderVisible: false,
        wickUpColor: '#26a69a',
        wickDownColor: '#ef5350',
      });
      console.log('Candlestick series created:', this.candlestickSeries);

      this.volumeSeries = this.chart.addSeries(window.LightweightCharts.HistogramSeries, {
        color: '#26a69a',
        priceFormat: {
          type: 'volume',
        },
        priceScaleId: '',
        scaleMargins: {
          top: 0.8,
          bottom: 0,
        },
      });
      console.log('Volume series created:', this.volumeSeries);

      this.chartStore.addChartObject({
        id: 'candlestick',
        name: 'Свечной график',
        series: this.candlestickSeries,
        visible: true,
      });

      this.chartStore.addChartObject({
        id: 'volume',
        name: 'Объём',
        series: this.volumeSeries,
        visible: true,
      });
    },
    updateDataFromStore() {
      console.log('Updating data from store');
      const historicalData = this.chartStore.historicalData;
      console.log('Historical data:', historicalData);
      
      if (historicalData) {
        if (historicalData.candlestickData && this.candlestickSeries) {
          console.log('Setting candlestick data:', historicalData.candlestickData);
          this.candlestickSeries.setData(historicalData.candlestickData);
        }
        if (historicalData.volumeData && this.volumeSeries) {
          console.log('Setting volume data:', historicalData.volumeData);
          this.volumeSeries.setData(historicalData.volumeData);
        }
      }
    },
    handleCrosshairMove(param) {
      if (!param.time || param.point === undefined) {
        return;
      }

      const candlestickObj = this.chartStore.chartObjects.find((obj) => obj.id === 'candlestick');
      const volumeObj = this.chartStore.chartObjects.find((obj) => obj.id === 'volume');

      const price = param.seriesPrices.get(candlestickObj?.series);
      const volume = param.seriesPrices.get(volumeObj?.series);

      if (price) {
        this.chartStore.setCrosshairData({
          time: param.time,
          price: price,
          volume: volume || 0,
        });
      }
    },
    handleResize() {
      if (this.chart && this.$refs.chartContainer) {
        const { width, height } = this.$refs.chartContainer.getBoundingClientRect();
        this.chart.resize(width, height);
      }
    },
  },
  watch: {
    'chartStore.historicalData': {
      handler(newData) {
        if (newData) {
          this.updateDataFromStore();
        }
      },
      deep: true,
    },
  },
};
</script>

<style scoped>
div {
  width: 100%;
  height: 600px;
}
</style>