<!-- src/components/chart/CandlesAndVolume.vue -->
<template>
  <!-- UI не нужен, рендеринг происходит на канвасе через lightweight-charts -->
</template>

<script>
import { CandlestickSeries, HistogramSeries } from 'lightweight-charts';
import { useChartStore } from '@/stores/chart';
import { nextTick } from 'vue';

export default {
  name: 'CandlesAndVolume',
  props: {
    chart: {
      type: Object,
      required: true,
    },
  },
  setup() {
    const chartStore = useChartStore();
    return { chartStore };
  },
  watch: {
    chart: {
      immediate: true,
      handler(newChart) {
        if (newChart) {
          // Откладываем инициализацию, чтобы график успел полностью настроиться
          nextTick(() => {
            this.initSeries();
            this.loadHistoricalData();
          });
        }
      },
    },
  },
  mounted() {
    this.chartStore.$subscribe((mutation, state) => {
      if (mutation.type === 'setHistoricalData') {
        this.handleHistoricalData(state.historicalData);
      }
    });
  },
  methods: {
    initSeries() {
      if (!this.chart) {
        console.warn('График не инициализирован, пропуск инициализации серий');
        return;
      }

      // Добавляем серию свечей
      let candlestickSeries;
      try {
        candlestickSeries = this.chart.addCandlestickSeries({
          upColor: '#26a69a',
          downColor: '#ef5350',
          wickUpColor: '#26a69a',
          wickDownColor: '#ef5350',
        });

        // Проверяем, что серия создана корректно
        if (!candlestickSeries) {
          console.error('Не удалось создать серию свечей');
          return;
        }

        // Применяем настройки шкалы цен
        candlestickSeries.priceScale().applyOptions({
          scaleMargins: {
            top: 0.1,
            bottom: 0.4,
          },
        });

        this.chartStore.addChartObject({
          id: 'candlestick',
          type: 'candlestick',
          series: candlestickSeries,
          visible: true,
          settings: { upColor: '#26a69a', downColor: '#ef5350' },
        });
      } catch (error) {
        console.error('Ошибка при добавлении серии свечей:', error);
        return;
      }

      // Добавляем серию объёмов
      try {
        const volumeSeries = this.chart.addHistogramSeries({
          color: '#26a69a',
          priceFormat: { type: 'volume' },
          priceScaleId: '',
          lastValueVisible: true,
        });

        // Проверяем, что серия создана корректно
        if (!volumeSeries) {
          console.error('Не удалось создать серию объёмов');
          return;
        }

        // Применяем настройки шкалы цен
        volumeSeries.priceScale().applyOptions({
          scaleMargins: {
            top: 0.7,
            bottom: 0,
          },
        });

        this.chartStore.addChartObject({
          id: 'volume',
          type: 'volume',
          series: volumeSeries,
          visible: true,
          settings: {},
        });
      } catch (error) {
        console.error('Ошибка при добавлении серии объёмов:', error);
        return;
      }
    },
    loadHistoricalData() {
      const historicalData = this.chartStore.historicalData;
      if (historicalData) {
        this.handleHistoricalData(historicalData);
      }
    },
    handleHistoricalData({ candlestickData, volumeData }) {
      const candlestickObj = this.chartStore.chartObjects.find((obj) => obj.id === 'candlestick');
      const volumeObj = this.chartStore.chartObjects.find((obj) => obj.id === 'volume');

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
    },
    updateVolumeLabel() {
      const volumeObj = this.chartStore.chartObjects.find((obj) => obj.id === 'volume');
      if (volumeObj && volumeObj.visible) {
        const volumeData = volumeObj.series.data();
        if (volumeData.length > 0) {
          const lastVolume = volumeData[volumeData.length - 1];
          const color = lastVolume.color || (this.chartStore.lastCandle.close >= this.chartStore.lastCandle.open ? '#26a69a' : '#ef5350');
          volumeObj.series.applyOptions({
            lastValueVisible: true,
            priceLineVisible: false,
            lastPriceAnimation: 0,
            color: color,
          });
        }
      }
    },
  },
};
</script>