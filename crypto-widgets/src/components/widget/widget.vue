<script setup>
import { ref, onMounted, watchEffect, onBeforeUnmount } from 'vue';
import Coin from '../Coin.vue';
const props = defineProps(['Open', 'Selected', 'CoinsData']);

const coins = ref(props.CoinsData);

watchEffect(() => {
  coins.value = props.CoinsData;
});

const coinsList = ref([]);
const evtSource = ref(null);

console.log(coins);
const setupEventSource = () => {
  const baseURL = "http://localhost:8000";
  evtSource.value = new EventSource(`${baseURL}/sse/?symbols=${coins.value}`);

  evtSource.value.onmessage = (event) => {
    const data = JSON.parse(event.data);

    for (let key in data) {
      if (data.hasOwnProperty(key)) {
        const existingCoinIndex = coinsList.value.findIndex((coin) => coin.name === key);

        data[key].name = key;
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
    <div class="coin-container flex gap-2">
      <Coin v-for="coin in coinsList" :key="coin.id" :data="coin" />
    </div>
  </div>
</template>
