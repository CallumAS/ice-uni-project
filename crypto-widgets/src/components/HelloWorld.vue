<script setup>
import { ref, onMounted, onBeforeUnmount } from 'vue';
import Dropdown from './DropDown.vue';
import Coin from './Coin.vue';

const defaultBoxTheme = 'm-4 mt-0 ml-0 bg-gray-100 rounded p-2 col-span-3';
const defaultHeaderTheme = 'text-gray-900 text-l font-semibold subpixel-antialiased font-mono';

const data = ref({});
const visibleCoins = ref([]);
const batchSize = 50;

async function fetchData() {
  try {
    const response = await fetch('http://127.0.0.1:8000/');

    if (!response.ok) {
      throw new Error(`HTTP error! Status: ${response.status}`);
    }

    data.value = await response.json();
    updateVisibleCoins();

  } catch (error) {
    console.error('Error fetching data:', error);
  }
}

function updateVisibleCoins() {
  const startIndex = visibleCoins.value.length;
  const endIndex = startIndex + batchSize;
  visibleCoins.value = [...visibleCoins.value, ...Object.values(data.value).slice(startIndex, endIndex)];
}

function handleScroll() {
  const { scrollTop, scrollHeight, clientHeight } = document.documentElement;

  if (scrollTop + clientHeight >= scrollHeight - 10) {
    updateVisibleCoins();
  }
}

onMounted(() => {
  window.addEventListener('scroll', handleScroll);
  fetchData();
});

onBeforeUnmount(() => {
  window.removeEventListener('scroll', handleScroll);
});
</script>

<template>
  <div class="mx-auto grid max-w-4xl grid-cols-12 gap-4 bg-zinc-50 p-1">
    <div id="selected" class="header col-span-12 rounded-lg border border-gray-300 bg-gray-600 py-8">
      <h1 :class="`${defaultHeaderTheme}`">Selected Coins</h1>
      <button :class="`${defaultHeaderTheme} bg-gray-300 hover:bg-gray-400 rounded-xl p-2`">Create Widget</button>
    </div>
    <div id="results" class="col-span-12 rounded-lg border border-gray-500 bg-gray-200 overflow-y-auto max-h-[400px]">
      <div id="filter">
        <h1 :class="`${defaultHeaderTheme}`">Search Coins</h1>
        <div class="flex">
          <Dropdown :Selected="selected" :Items="[234, 266, 273]" />
          <input type="text" placeholder="coinname" />
        </div>
      </div>
      <h1 :class="`${defaultHeaderTheme}`">Results</h1>
      <div class="flex flex-wrap">
        <Coin v-for="(coinData, key) in visibleCoins" :key="key" :data="coinData" />
      </div>
    </div>
  </div>
</template>

<style scoped>
.read-the-docs {
  color: #888;
}
</style>
