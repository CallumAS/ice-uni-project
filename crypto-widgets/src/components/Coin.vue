<!-- Coin.vue -->
<script>
import { ref, onMounted } from 'vue';

export default {
  props: {
    data: Object,
  },
  setup(props) {
    const coinData = ref(props.data);

    onMounted(() => {
      // You can add any additional setup logic here
    });

    return {
      coinData,
    };
  },
};
</script>

<template>
  <div class="coin-card p-4 bg-white rounded-lg shadow-md">
    <img
      :src="'http://localhost:8000/public/' + coinData.id + '.png'"
      :alt="coinData.symbol"
      class="rounded-t w-full h-36 object-cover"
    />
    <div class="flex flex-col justify-between p-4">
      <h4 class="text-xl font-semibold text-blue-gray-900 overflow-hidden overflow-ellipsis">{{ coinData.name }}</h4>
      <div class="flex flex-col mt-2">
        <p class="text-gray-700 overflow-hidden overflow-ellipsis">Price: ${{ coinData.price.toFixed(2) }}</p>
        <p class="text-gray-700 overflow-hidden overflow-ellipsis">MCap: ${{ coinData.marketcap.toFixed(2) }}B</p>
      </div>
    </div>
  </div>
</template>

<style scoped>
.coin-card {
  width: 200px;
  height: 300px;
  transition: transform 0.3s ease-in-out;
}


/* Add ellipsis for text overflow */
.overflow-ellipsis {
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
</style>
