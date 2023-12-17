<script setup>
defineProps({
  msg: String,
})
</script>

<template>
  <div class="">
  <div class="relative flex max-w-[20rem] flex-col rounded-xl bg-white bg-clip-border text-gray-700 shadow-md">
   <img
      src="https://images.unsplash.com/photo-1522202176988-66273c2fd55f?ixlib=rb-4.0.3&amp;ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&amp;auto=format&amp;fit=crop&amp;w=1471&amp;q=80"
      alt="ui/ux review check"
      class="rounded-t"
    />
  <div class="p-6">
    <h4 class="block font-sans text-xl font-semibold leading-snug tracking-normal text-blue-gray-900 antialiased">
      UI/UX Review Check
    </h4>
    <p class="mt-3 block font-sans font-normal leading-relaxed text-gray-700 antialiased">
      Because it's about motivating the doers. Because I'm here to follow my
      dreams and inspire others.
    </p>
  </div>
  <div class="flex items-center justify-between p-6">
    <p class="block font-sans text-base font-normal leading-relaxed text-inherit antialiased">
      January 10
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
