<script setup lang="ts">
import { useAuthStore } from '@/stores/auth';
import { computed, ref } from 'vue';
import router from '../router/index';

const auth = useAuthStore();
const password = ref('');
const authenticating = ref(false);
const wrong_password = ref(null as (null | string));
const invalid_password = computed(() => wrong_password.value == password.value);

async function login() {
  authenticating.value = true;
  await auth.authenticate(password.value);
  
  if (auth.authenticated) {
    router.push('/');
    wrong_password.value = null;
  } else {
    wrong_password.value = password.value;
  }
  authenticating.value = false;
}

</script>

<template>
  <main>
    <div id="login-container" class="container d-flex justify-content-center align-items-center">
      <div class="container">
        <div class="row justify-content-center">
          <div class="col-2">
            <label for="password" class="col-form-label">Password</label>
          </div>
          <div class="col-3">
            <input v-model="password" type="password" id="password" class="form-control">
          </div>
        </div>
        <div class="row justify-content-center">
          <button :disabled="authenticating || invalid_password" @click="login" class="btn btn-primary col-5 my-2">Login</button>
        </div>
        <div v-if="wrong_password" class="row justify-content-center">
          <div class="col-5 text-danger text-center">
            Wrong Password!
          </div>
        </div>
      </div>
    </div>
  </main>
</template>

<style scoped>
#login-container {
  height: 100vh;
}
</style>