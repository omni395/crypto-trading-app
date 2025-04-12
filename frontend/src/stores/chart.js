// stores/chart.js
import { defineStore } from 'pinia';

export const useChartStore = defineStore('chart', {
  state: () => ({
    chart: null,
    chartObjects: [],
    priceLine: null,
    websocket: null,
    symbol: "BTCUSDT",
    interval: "1m",
    earliestTime: null,
    isLoading: false,
    drawingTool: null,
    drawnLines: [],
    previousPrice: null,
    lastCandle: null,
  }),
  actions: {
    setChart(chart) {
      this.chart = chart;
    },
    setWebsocket(websocket) {
      this.websocket = websocket;
    },
    addChartObject(obj) {
      this.chartObjects.push(obj);
    },
    updatePriceLine(priceLine) {
      this.priceLine = priceLine;
    },
    setEarliestTime(time) {
      this.earliestTime = time;
    },
    setLoading(loading) {
      this.isLoading = loading;
    },
    setDrawingTool(tool) {
      this.drawingTool = tool;
    },
    addDrawnLine(line) {
      this.drawnLines.push(line);
    },
    setPreviousPrice(price) {
      this.previousPrice = price;
    },
    setLastCandle(candle) {
      this.lastCandle = candle;
    },
  },
});