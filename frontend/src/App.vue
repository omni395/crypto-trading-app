<template>
  <!-- Header Section Navbar -->
  <div class="">
    <Navigation />
  </div>
  <!-- Main section -->
  <section class="w-full h-screen p-5 flex flex-cols-2 gap-4">
    <!-- Left side with chart -->
    <div class="w-5/6 flex-grow">
      <div class="flex-row">
        <Chart />
      </div>
      <div class="flex-row">
        <div class="w-1/3">
          <EmptyBlock />
        </div>
        <div class="w-1/3">
          <EmptyBlock />
        </div>
        <div class="w-1/3">
          <EmptyBlock />
        </div>
      </div>
    </div>
    <!-- Right side with filters and instruments-->
    <div class="w-1/6">
      <div class="">
        <Filters />
        <instrumentList />
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
      EmptyBlock
    },
  };
</script>