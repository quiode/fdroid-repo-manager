<script setup lang="ts">
import { computed } from 'vue';
import { useAppStore } from '../stores/app';

const appStore = useAppStore();
const noApps = computed(() => appStore.apps.length == 0);

function onUpload() {
 alert("upload");
}
</script>

<template>
  <div class="container" id="app-container">
    <div v-if="noApps" class="d-flex justify-content-center align-items-center w-100 h-100">
        <button @click="onUpload" class="btn btn-primary h-25 w-75"><h2><i class="bi bi-cloud-arrow-up me-3"></i>Upload App</h2></button>
    </div>
  </div>
</template>

<style scoped>
#app-container{
  width: 100vw;
  height: 90vh;
}
</style>