import { defineStore } from 'pinia';

export const useChartStore = defineStore('chart', {
  state: () => ({
    chart: null,
    websocket: null,
    chartObjects: [],
    drawnLines: [],
    drawingTool: null,
    symbol: 'BTCUSDT',
    interval: '1m',
    priceLine: null,
    previousPrice: null,
    lastCandle: null,
    earliestTime: null,
    isLoading: false,
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
    addDrawnLine(line) {
      this.drawnLines.push(line);
    },
    removeDrawnLine(id) {
      this.drawnLines = this.drawnLines.filter(line => line.id !== id);
    },
    updateDrawnLine(id, line) {
      const index = this.drawnLines.findIndex(l => l.id === id);
      if (index !== -1) {
        this.drawnLines[index] = line;
      }
    },
    clearDrawnLines() {
      this.drawnLines = [];
    },
    setDrawingTool(tool) {
      this.drawingTool = tool;
    },
    updatePriceLine(priceLine) {
      this.priceLine = priceLine;
    },
    setPreviousPrice(price) {
      this.previousPrice = price;
    },
    setLastCandle(candle) {
      this.lastCandle = candle;
    },
    setEarliestTime(time) {
      this.earliestTime = time;
    },
    setLoading(loading) {
      this.isLoading = loading;
    },
    toggleObjectVisibility(id) {
      const obj = this.chartObjects.find((o) => o.id === id);
      if (obj) {
        obj.visible = !obj.visible;
      }
    },
  },
});