// stores/chart.js
import { defineStore } from 'pinia';

export const useChartStore = defineStore('chart', {
  state: () => ({
    chart: null,
    websocket: null,
    symbol: 'BTCUSDT',
    interval: '1m',
    chartObjects: [],
    drawnLines: [],
    drawingTool: null,
    isLoading: false,
    earliestTime: null,
    lastCandle: null,
    previousPrice: null,
    priceLine: null,
  }),
  actions: {
    setChart(chart) {
      this.chart = chart;
    },
    setWebsocket(websocket) {
      this.websocket = websocket;
    },
    addChartObject(object) {
      this.chartObjects.push(object);
    },
    setDrawingTool(tool) {
      this.drawingTool = tool;
    },
    addDrawnLine(line) {
      this.drawnLines.push(line);
    },
    removeDrawnLine(index) {
      this.drawnLines.splice(index, 1);
    },
    // Добавляем новый метод для обновления линии
    updateDrawnLine(index, updatedLine) {
      this.drawnLines[index] = updatedLine;
    },
    setLoading(loading) {
      this.isLoading = loading;
    },
    setEarliestTime(time) {
      this.earliestTime = time;
    },
    setLastCandle(candle) {
      this.lastCandle = candle;
    },
    setPreviousPrice(price) {
      this.previousPrice = price;
    },
    updatePriceLine(priceLine) {
      this.priceLine = priceLine;
    },
  },
});