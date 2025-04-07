<template>
  <div class="flex flex-row h-screen p-4 gap-4">
    <!-- Левая часть (5/6 ширины) -->
    <div class="flex flex-col w-5/6 h-full gap-4">
      <!-- График (60% высоты) -->
      <div class="h-[60%] bg-gray-800 rounded-lg">
        <Chart />
      </div>
      <!-- Три контейнера-заглушки (40% высоты, одинаковой ширины) -->
      <div class="flex flex-row h-[40%] gap-4">
        <div class="flex-1 bg-gray-800 rounded-lg">
          <EmptyBlock />
        </div>
        <div class="flex-1 bg-gray-800 rounded-lg">
          <EmptyBlock />
        </div>
        <div class="flex-1 bg-gray-800 rounded-lg">
          <EmptyBlock />
        </div>
      </div>
    </div>

    <!-- Правая часть (1/6 ширины) -->
    <div class="flex flex-col w-1/6 h-full gap-4">
      <!-- Фильтры (1/6 высоты) -->
      <div class="h-1/6 bg-gray-800 rounded-lg">
        <Filters />
      </div>
      <!-- Список инструментов (остальная высота) -->
      <div class="flex-1 bg-gray-800 rounded-lg">
        <InstrumentList />
      </div>
    </div>
  </div>
</template>

<script>
import Filters from './components/Filters.vue';
import InstrumentList from './components/InstrumentList.vue';
import Chart from './components/Chart.vue';
import EmptyBlock from './components/EmptyBlock.vue';

export default {
  name: 'App',
  components: {
    Filters,
    InstrumentList,
    Chart,
    EmptyBlock,
  },
};
</script>