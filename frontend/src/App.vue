<template>
  <div class="grid grid-cols-12 grid-rows-3 gap-1">
      <div class="col-span-12 col-start-1">
        <Navigation />
      </div>
      <div class="col-span-9 row-span-2 col-start-1 row-start-2">
        <Chart />
      </div>
      <div class="col-span-3 col-start-1 row-start-3">
        <Filters />
        <instrumentList />
      </div>
      <div class="col-span-3 col-start-4 row-start-3">
        <EmptyBlock />
      </div>
      <div class="col-span-3 col-start-7 row-start-3">
        <EmptyBlock />
      </div>
      <div class="col-span-3 row-span-2 col-start-10 row-start-2">
        <EmptyBlock />
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
    Navigation,
    EmptyBlock
  },
};
</script>