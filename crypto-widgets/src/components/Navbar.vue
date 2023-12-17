<script setup>
defineProps({
  msg: String,
})
</script>

<template>
  <nav class="font-sans flex flex-col text-center content-center sm:flex-row sm:text-left sm:justify-between py-2 px-6 bg-gray-100 shadow sm:items-baseline w-full">

  <div class="mb-2 sm:mb-0 flex flex-row
  ">
    <div class="h-10 w-10 self-center mr-2">
      <img class="h-10 w-10 self-center" src="https://csscomps.com/images/csscomps.png" />
    </div>
    <div>
      <a href="/home" class="text-2xl no-underline text-grey-darkest hover:text-blue-dark font-sans font-bold">Coin Website</a><br>
      <span class="text-xs text-grey-dark">Crypto widgets in a few clicks</span>
    </div>
  </div>

  <div class="sm:mb-0 self-center">
    <a href="#" class="text-md no-underline text-black hover:text-blue-dark ml-2 px-1       bg-gray-300 hover:bg-gray-400 rounded-xl p-1">Ticker</a>
    <a href="#" class="text-md no-underline text-grey-darker hover:text-blue-dark ml-2 px-1 bg-gray-300 hover:bg-gray-400 rounded-xl p-1">Converter</a>
    <!-- </div> -->

  </div>
</nav>
</template>

<style scoped>
.read-the-docs {
  color: #888;
}
</style>
