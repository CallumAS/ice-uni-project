<script setup>
import { ref, onMounted, watchEffect, onBeforeUnmount } from 'vue';
import Coin from '../Coin.vue';
const coins = ref("Bitcoin,Kaspa");
const coinsList = ref([]);
const evtSource = ref(null);

const setupEventSource = () => {
  const baseURL = "http://localhost:8000";
  evtSource.value = new EventSource(`${baseURL}/sse/?symbols=${coins.value}`);

  evtSource.value.onmessage = (event) => {
    const data = JSON.parse(event.data);

    for (let key in data) {
      if (data.hasOwnProperty(key)) {
        const existingCoinIndex = coinsList.value.findIndex((coin) => coin.id === key);

        if (existingCoinIndex !== -1) {
          coinsList.value[existingCoinIndex] = data[key];
        } else {
          coinsList.value.push(data[key]);
        }
      }
    }

    localStorage.setItem("coins", coins.value);
  };
};

onMounted(() => {
  setupEventSource();
});

watchEffect(() => {
  setupEventSource();
});

onBeforeUnmount(() => {
  if (evtSource.value) {
    evtSource.value.close();
  }
});
</script>

<style scoped>
/* Your component-specific styles go here */
</style>

<template>
  <div>
    <h1 class="coin-widget-title">Coin Ticker</h1>
    <div class="coin-container flex gap-2">
      <Coin v-for="coin in coinsList" :key="coin.id" :data="coin" />
    </div>
  </div>
</template>
