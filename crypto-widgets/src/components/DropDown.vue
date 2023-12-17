<script setup>
defineProps({
    Items: Array,
  Selected: String,
},)
</script>

<template>
  <div class="w-12">
  <!--Selected-->
  {{Selected}}

  <!--Items-->
  <div class="mt-6 absolute bg-white border rounded p-2">
    <li v-for="item in Items">
        {{ item }}
    </li>
  </div>

  </div>
</template>

<style scoped>
.read-the-docs {
  color: #888;
}
</style>
