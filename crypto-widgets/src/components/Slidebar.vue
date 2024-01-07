<script setup>
import { ref, onMounted, onBeforeUnmount, watchEffect } from 'vue';
import Widget from './widget/widget.vue'

const props = defineProps(['Open', 'Selected', 'CoinsData']);

const domain = "http://127.0.0.1:8000/"

const CoinsData = ref(props.CoinsData);
watchEffect(() => {
  CoinsData.value = props.CoinsData;
});
</script>

<template>
<div :class="{ 'open': Open, 'sidebar': true}" class="overflow-hidden bg-gray-100 sm:w-screen h-full fixed">
<button @click="Open = !Open">Close</button>

<div class="p-2 rounded-xl">
<Widget :CoinsData="CoinsData"/>
</div>
<div class="bg-white rounded-lg shadow-md p-6">
    <h1 class="text-2xl font-bold mb-4">Widget</h1>
    <p class="text-black bg-gray-100 leading-relaxed rounded-md p-2">{{ `<coins-widget coins="${CoinsData}"></coins-widget>` }}</p>

    <h1 class="text-2xl font-bold mt-8 mb-4">Style</h1>
    <p class="text-black bg-gray-100 leading-relaxed rounded-md p-2">{{ '<link rel="stylesheet" href="'+domain+'/style.css">' }}</p>

    <h1 class="text-2xl font-bold mt-8 mb-4">Javascript</h1>
    <p class="text-black bg-gray-100 leading-relaxed rounded-md p-2">{{ '<script type="text/javascript" src="'+domain+'/script.js"></script>' }}</p>
</div>



</div>
</template>

<style scoped>
.sidebar {
    -webkit-transition: max-width 1s ease-in-out;
    max-width: 0;
    width: 300vw;
}

.open {
    max-width: 300vw !important;
}
@media (max-width: 767px) {
    .sidebar {
        position: absolute;
        top: 0;
        right: 0;
        width: 100%;
        max-width: 100%;
        transition: right 1s ease-in-out;
        right: -100%;
    }

    .open {
        right: 0;
    }
}


</style>
