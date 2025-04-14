// src/stores/chart.js
import { defineStore } from 'pinia';

export const useChartStore = defineStore('chart', {
  state: () => ({
    chart: null,
    websocket: null,
    symbol: 'BTCUSDT',
    interval: '1m',
    chartObjects: [],
    drawnLines: [],
    historicalData: null,
    crosshairData: null,
  }),

  actions: {
    setChart(chart) {
      this.chart = chart;
    },

    setWebsocket(websocket) {
      this.websocket = websocket;
    },

    setSymbol(symbol) {
      this.symbol = symbol;
    },

    setInterval(interval) {
      this.interval = interval;
    },

    addChartObject(obj) {
      const existingIndex = this.chartObjects.findIndex((item) => item.id === obj.id);
      if (existingIndex !== -1) {
        this.chartObjects[existingIndex] = { ...obj };
      } else {
        this.chartObjects.push({ ...obj });
      }
    },

    removeChartObject(id) {
      const index = this.chartObjects.findIndex((obj) => obj.id === id);
      if (index !== -1) {
        this.chartObjects.splice(index, 1);
      }
    },

    toggleObjectVisibility(id) {
      const obj = this.chartObjects.find((o) => o.id === id);
      if (obj) {
        obj.visible = !obj.visible;
        obj.series.applyOptions({ visible: obj.visible });
      }
    },

    addDrawnLine(line) {
      this.drawnLines.push(line);
    },

    removeDrawnLine(id) {
      const index = this.drawnLines.findIndex((line) => line.id === id);
      if (index !== -1) {
        const line = this.drawnLines[index];
        if (line.line) {
          if (line.drawing_type === 'drawing.line') {
            const candlestickObj = this.chartObjects.find((obj) => obj.id === 'candlestick');
            if (candlestickObj) {
              candlestickObj.series.removePriceLine(line.line);
            }
          } else {
            this.chart.removeSeries(line.line);
          }
        }
        this.drawnLines.splice(index, 1);
      }
    },

    setHistoricalData(data) {
      this.historicalData = data;
    },

    setCrosshairData(data) {
      this.crosshairData = data;
    },

    clearStore() {
      this.chart = null;
      this.websocket = null;
      this.chartObjects = [];
      this.drawnLines = [];
      this.historicalData = null;
      this.crosshairData = null;
    },
  },
});