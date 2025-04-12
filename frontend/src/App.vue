<template>
  <!-- Header Section Navbar -->
  <div>
    <Navigation />
  </div>
  <!-- Main section -->
  <section class="flex w-full h-screen p-5">
    <!-- Left side with chart -->
    <div class="w-5/6 p-4 bg-gray-100 rounded-lg shadow-md flex flex-col">
      <Chart />
      <div class="flex flex-row">
        <div class="w-1/3">
          <EmptyBlock />
          Пусто
        </div>
        <div class="w-1/3">
          <EmptyBlock />
          Пусто
        </div>
        <div class="w-1/3">
          <EmptyBlock />
          Пусто
        </div>
      </div>
    </div>
    <!-- Right side with filters and instruments -->
    <div class="w-1/6 p-4 bg-gray-100 rounded-lg shadow-md flex flex-col">
      <div>
        <Filters />
        <InstrumentList />
      </div>
    </div>
  </section>
  <EmptyBlock />
  <EmptyBlock />
</template>

<script>
import Navigation from './components/Navigation.vue';
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
    EmptyBlock,
  },
};
</script>