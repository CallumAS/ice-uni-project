<script setup>
defineProps({
  Open: Boolean,
  Selected: Array,
})
</script>

<template>
<div :class="{ 'open': Open, 'sidebar': true}" class="overflow-hidden bg-gray-100">
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


</div>
</template>

<style scoped>
.sidebar {
    -webkit-transition: max-width 1s linear .5s;
    max-width: 0px;
    width: 100%;
}

.open {
        max-width: 40%!important;
}

</style>
