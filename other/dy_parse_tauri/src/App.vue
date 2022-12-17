<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Parse from "./components/Parse.vue";
</script>

<template>
  <div class="container">
    <h1>抖音短视频链接解析工具</h1>
    <div style="font-weight: 600; font-size: 12px;">By 公众号《Rust 学习日记》</div>
    <Parse />
  </div>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
