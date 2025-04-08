<!-- src/components/Chart.vue -->
<template>
  <div ref="chartContainer" class="w-full h-full"></div>
</template>

<script>
import { createChart } from 'lightweight-charts'

export default {
  name: 'Chart',
  props: {
    data: {
      type: Array,
      required: true
    }
  },
  data() {
    return {
      chart: null,
      areaSeries: null
    }
  },
  mounted() {
    this.$nextTick(() => {
      this.initChart()
      const resizeObserver = new ResizeObserver(() => {
        if (this.chart) {
          this.chart.resize(
            this.$refs.chartContainer.clientWidth,
            this.$refs.chartContainer.clientHeight
          )
        }
      })
      resizeObserver.observe(this.$refs.chartContainer)
    })
  },
  beforeUnmount() {
    if (this.chart) {
      this.chart.remove()
    }
  },
  methods: {
    initChart() {
      if (!this.$refs.chartContainer) return

      this.chart = createChart(this.$refs.chartContainer, {
        autoSize: true,
        layout: {
          background: { color: '#1e1e1e' },
          textColor: '#d1d4dc'
        },
        grid: {
          vertLines: { color: '#2a2e39' },
          horzLines: { color: '#2a2e39' }
        }
      })

      this.areaSeries = this.chart.addAreaSeries({
        topColor: 'rgba(38, 198, 218, 0.56)',
        bottomColor: 'rgba(38, 198, 218, 0.04)',
        lineColor: 'rgba(38, 198, 218, 1)',
        lineWidth: 2
      })

      this.areaSeries.setData(this.data)
      this.chart.timeScale().fitContent()
    }
  }
}
</script>