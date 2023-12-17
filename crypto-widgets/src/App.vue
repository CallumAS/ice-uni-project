<script setup>
import HelloWorld from './components/HelloWorld.vue'
import Slidebar from './components/Slidebar.vue'
import Navbar from './components/Navbar.vue'
import Footer from './components/Footer.vue'
</script>

<template>
    <Navbar />
    <div class="flex">
    <HelloWorld class="grow" msg="Vite + Vue" />
  <Slidebar :Open="true"/>
    </div>
  <Footer/>
</template>