<script>
import { defineProps, ref, onMounted } from 'vue';

export default {
  props: {
    data: Object,
  },
  setup(props) {
    const coinData = ref(props.data);

    onMounted(() => {
    });

    return {
      coinData,
    };
  },
};
</script>

<template>
  <div class="">
    <div class="relative flex max-w-[20rem] flex-col rounded-xl bg-white bg-clip-border text-gray-700 shadow-md">
        <img
        :src="'http://localhost:8000/public/' + coinData.id + '.png'"
        :alt="coinData.symbol"
        class="rounded-t"
        />
      <div class="p-6">
        <h4 class="block font-sans text-xl font-semibold leading-snug tracking-normal text-blue-gray-900 antialiased">
          <p>{{ coinData.symbol }}</p>
        </h4>
        <p class="mt-3 block font-sans font-normal leading-relaxed text-gray-700 antialiased">
          {{ coinData.price }}
        </p>
      </div>
      <div class="flex items-center justify-between p-6">
        <p class="block font-sans text-base font-normal leading-relaxed text-inherit antialiased">
          {{ coinData.marketcap }}
        </p>
      </div>
    </div>
  </div>
</template>

<style scoped>
.read-the-docs {
  color: #888;
}
</style>
