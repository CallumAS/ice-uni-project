<script setup>
import { ref, onMounted, onBeforeUnmount } from 'vue';
import Widget from './widget/widget.vue'
defineProps({
  Open: Boolean,
  Selected: Array,
})
</script>

<template>
<div :class="{ 'open': Open, 'sidebar': true}" class="overflow-hidden bg-gray-100 sm:w-screen h-full fixed">
<button @click="Open = !Open">CLOSE</button>
<h1>Your code</h1>

<div class="bg-gray-500 w-96 h-24 rounded-xl">
OPTIONS
<input type="color" id="html5colorpicker" onchange="clickColor(0, -1, -1, 5)" value="#ff0000" style="width:85%;">
<input type="color" id="html5colorpicker" onchange="clickColor(0, -1, -1, 5)" value="#ff0000" style="width:85%;">
<input type="color" id="html5colorpicker" onchange="clickColor(0, -1, -1, 5)" value="#ff0000" style="width:85%;">
</div>
<div class="bg-gray-500 w-96 h-24 rounded-xl">
EXAMPLE
</div>

<div class="bg-gray-500 w-96 h-24 rounded-xl">
CODE
</div>
<Widget/>

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
