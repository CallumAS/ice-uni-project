<script setup>
defineProps({
  msg: String,
})
</script>

<template>
<footer class="bg-gray-100">
    <div class="container px-6 py-8 mx-auto">
        <div class="flex flex-col items-center text-center">
            <p class="max-w-md mx-auto mt-4 text-gray-500 dark:text-gray-400">Better thing here.</p>

            <div class="flex flex-col mt-4 sm:flex-row sm:items-center sm:justify-center">
                <button class="w-full px-5 py-2 text-sm tracking-wide text-white capitalize transition-colors duration-300 transform bg-blue-600 rounded-md sm:mx-2 sm:order-2 sm:w-auto hover:bg-blue-500 focus:outline-none focus:ring focus:ring-blue-300 focus:ring-opacity-80">Get started</button>
            </div>
        </div>
        <div class="flex flex-col items-center sm:flex-row sm:justify-between">
            <p class="text-sm text-gray-500">©Copyright 2023.</p>
        </div>
    </div>
</footer>
</template>

<style scoped>
.read-the-docs {
  color: #888;
}
</style>
