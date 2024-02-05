import { ref } from 'vue';
import { defineStore } from 'pinia';
interface App { };

export const useAppStore = defineStore('apps', () => {
  const apps = ref([] as App[]);

  return { apps };
});