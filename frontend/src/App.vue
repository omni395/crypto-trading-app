<!-- src/App.vue -->
<template>
  <div id="app" class="flex h-screen m-0 p-0">
    <div class="w-64 bg-gray-900 text-white p-5 flex flex-col gap-5">
      <div>
        <h1 class="text-2xl">Crypto Trading App</h1>
      </div>
      <div class="flex-1">
        <h3>Разные фильтры</h3>
      </div>
      <div class="flex-1">
        <h3>Список инструментов</h3>
      </div>
    </div>
    <div class="flex-1 flex flex-col h-full">
      <div class="flex-1 h-1/2">
        <h3>Основной график</h3>
        <Chart :data="[]"/>
      </div>
      <div class="flex-1 h-1/4 bg-gray-800 text-white flex items-center justify-center">Пока пусто</div>
      <div class="flex-1 h-1/4 bg-gray-800 text-white flex items-center justify-center">Пока пусто</div>
      <div class="flex-1 h-1/4 bg-gray-800 text-white flex items-center justify-center">Пока пусто</div>
    </div>
  </div>
</template>

<script>
import Chart from './components/Chart.vue'

export default {
  name: 'App',
  components: { Chart }
}
</script>