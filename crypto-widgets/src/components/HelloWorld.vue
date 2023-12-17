<script setup>
import { ref } from 'vue'
import Dropdown from './DropDown.vue'
import Coin from './Coin.vue'
defineProps({
  msg: String,
})

const count = ref(0)
const selected = ref("All")

const defaultBoxTheme = 'm-4 mt-0 ml-0 bg-gray-100 rounded p-2 col-span-3'
const defaultHeaderTheme = 'text-gray-900 text-l font-semibold	subpixel-antialiased	font-mono	'

</script>

<template>
  <div class="h-full grid grid-rows-3 grid-flow-col mt-4">
    <div id="selected" v-bind:class="`${defaultBoxTheme}`">
    <h1 v-bind:class="`${defaultHeaderTheme}`">Selected Coins</h1>

    <button v-bind:class="`${defaultHeaderTheme} bg-gray-300 hover:bg-gray-400 rounded-xl p-2`">Create Widget</button>
    </div>
   
    <div id="results" v-bind:class="`${defaultBoxTheme} row-span-2`">
     <div id="filter" v-bind:class="``">
        <h1 v-bind:class="`${defaultHeaderTheme}`">Search Coins</h1>

        <div class="flex">
        <Dropdown v-bind:Selected="selected" :Items="[234, 266, 273]"/>
        <input type="text" placeholder="coinname"/>
        </div>
    </div>
        <h1 v-bind:class="`${defaultHeaderTheme}`">Results</h1>
        <div>
        <Coin/>
        </div>
    </div>

  </div>
</template>

<style scoped>
.read-the-docs {
  color: #888;
}
</style>
