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
    drawingTool: null,
    lastCandle: null,
    earliestTime: null,
    isLoading: false,
    historicalData: null, // Для передачи исторических данных
    indicators: [], // Для будущих индикаторов
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
    toggleObjectVisibility(id) {
      const obj = this.chartObjects.find((o) => o.id === id);
      if (obj) {
        obj.visible = !obj.visible;
      }
    },
    addDrawnLine(line) {
      this.drawnLines.push(line);
    },
    removeDrawnLine(id) {
      this.drawnLines = this.drawnLines.filter((line) => line.id !== id);
    },
    updateDrawnLine(oldId, updatedLine) {
      const index = this.drawnLines.findIndex((line) => line.id === oldId);
      if (index !== -1) {
        this.drawnLines[index] = updatedLine;
      }
    },
    setDrawingTool(tool) {
      this.drawingTool = tool;
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
    setHistoricalData(data) {
      this.historicalData = data;
    },
  },
});