<script setup>
import { ref, onMounted, onBeforeUnmount } from 'vue';
import {  watch, nextTick } from 'vue';

import Dropdown from './DropDown.vue';
import Coin from './Coin.vue';
import Slidebar from './Slidebar.vue'
import draggable from "vuedraggable";
import Navbar from './Navbar.vue'
import { computed } from 'vue';

const showResults = ref(false);

const defaultBoxTheme = 'm-4 mt-0 ml-0 bg-gray-100 rounded p-2 col-span-3';
const defaultHeaderTheme = 'text-gray-900 text-l font-semibold subpixel-antialiased font-mono';
const selectedCoinsNames = computed(() => selectedCoins.value.map(x => x.name).join(','));


const data = ref({});
const visibleCoins = ref([]);
const selectedCoins = ref([]);
const batchSize = 50;

async function fetchData() {
  try {
    const response = await fetch('http://127.0.0.1:8000/');

    if (!response.ok) {
      throw new Error(`HTTP error! Status: ${response.status}`);
    }


let r = await response.json();

const coinArray = Object.entries(r);

coinArray.sort(([, valueA], [, valueB]) => {
    const cmcA = Number(valueA.cmc) || 0; // Convert to number or default to 0
    const cmcB = Number(valueB.cmc) || 0; // Convert to number or default to 0
    return cmcA - cmcB;
});

// Convert back to object
r = Object.fromEntries(coinArray);

data.value = r;
  
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

const resultsPadding = ref(0);


onMounted(() => {
  window.addEventListener('scroll', handleScroll);
  fetchData();
  
  //Very ugly fixed couldn't get watch working god knows why hopefully no one sees this
  //https://i.imgur.com/foClXMw.jpeg  
let selected = document.getElementById('selected');
resultsPadding.value = selected.offsetHeight;
let observer = new MutationObserver((mutationsList) => {
  for (let mutation of mutationsList) {
    if (mutation.type === 'childList' || mutation.type === 'attributes') {
      resultsPadding.value = selected.offsetHeight;
    }
  }
});
observer.observe(selected, { childList: true, attributes: true, subtree: true });     

});

onBeforeUnmount(() => {
  window.removeEventListener('scroll', handleScroll);
});
</script>

<template>
<div class="flex h-full">
  <div class="w-full">
    <div id="selected" class="fixed top-0 left-0 right-0 bg-gray-100 border border-gray-300 rounded-lg">
        <Navbar />
      <h1 :class="`${defaultHeaderTheme}`">Selected Coins</h1>
      <div v-if="selectedCoins.length === 0">
      <h1>Drag a coin here</h1>
    </div>
    
       <draggable
        class="flex flex-wrap gap-2 justify-center items-center"
        :list="selectedCoins"
        group="people"
        itemKey="id" 
      >
        <template #item="{ element, index }">
          <div class="list-group-item">
              <Coin :data="element" />
              </div>
        </template>
      </draggable>

      <button v-if="selectedCoins.length > 0" :class="`${defaultHeaderTheme} bg-gray-300 hover:bg-gray-400 rounded-xl p-2 m-2`" @click="showResults = !showResults">Create Widget</button>
    </div>
    <div id="results" class="col-span-12 rounded-lg border border-gray-500 bg-gray-200 overflow-y-auto" :style="{ paddingTop: resultsPadding + 'px' }">
      <h1 :class="`${defaultHeaderTheme}`">Results</h1>
        <draggable
  class="flex flex-wrap gap-2 justify-center items-center"
  :list="visibleCoins"
  group="people"
  itemKey="id" 
>
  <template #item="{ element, index }">
    <Coin :data="element" />
  </template>
</draggable>

    </div>
  </div>
    <Slidebar :Open="showResults" :CoinsData="selectedCoinsNames"/>
</div>
</template>

<style scoped>
.list-group {
  
}
</style>
