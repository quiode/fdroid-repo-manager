<script setup lang="ts">
function onSettings() {
  alert("settings");
}
</script>

<template>
  <nav class="row m-0">
    <div class="col-5"></div>
    <h1 class="col-2 text-center h-100">
      <router-link to="/">
        Apps
      </router-link>
    </h1>
    <div class="col-4"></div>
    <div class="col-1">
      <div class="d-flex justify-content-center align-content-end w-100 h-100 pb-2">
        <h1 @click="onSettings" style="height: min-content;" class="my-auto pointer"><i class="bi bi-gear-fill"></i></h1>
      </div>
    </div>
  </nav>
</template>

<style scoped>
nav {
  width: 100vw;
  height: 5vh;
  position: relative;
  background-color: lightgray;
}

h1 {
  font-size: 3vh !important;
}

a {
  color: inherit;
}
</style>