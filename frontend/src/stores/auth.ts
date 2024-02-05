import { axios, default_config } from './../scripts/axios';
import { ref, watch } from 'vue'
import { defineStore } from 'pinia'
import router from '../router/index';

export const useAuthStore = defineStore('auth', () => {
  const authenticated = ref(false);
  const authenticating = ref(false);
  const password = ref('');

  watch(password, async (old_password, new_password) => {
    default_config.headers['RM-PASSWORD'] = new_password;
  });

  const authenticate = async (password_i: string) => {
    if (!authenticating.value) {
      authenticating.value = true;

      const result = await axios.post("/auth", password_i, { ...default_config, responseType: 'text' });
      if (result.data == "true") {
        authenticated.value = true;
        password.value = password_i;
        sessionStorage.setItem("password", password.value);
      } else {
        authenticated.value = false;
        password.value = '';
      }

      authenticating.value = false;
    }
  };

  // try to log in
  const saved_password = sessionStorage.getItem("password");
  if (saved_password) {
    authenticate(saved_password).then(() => router.push('/'));
  }

  return { authenticated, authenticating, password, authenticate };
});