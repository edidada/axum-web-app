import { ref, computed } from 'vue';
import { defineStore } from 'pinia';
import { apiService } from '../services/api';

export const useAuthStore = defineStore('auth', () => {
  const isLoggedIn = ref(false);
  const username = ref('');
  const sessionId = ref('');

  const isAuthenticated = computed(() => isLoggedIn.value && !!sessionId.value);

  async function login(user: string, password: string) {
    try {
      console.log('Auth store login called with:', { user, password });
      const response = await apiService.login(user, password);
      console.log('Auth store login response:', response);
      if (response.success && response.session_id) {
        isLoggedIn.value = true;
        username.value = user;
        sessionId.value = response.session_id;
        console.log('Auth store login success:', { sessionId: response.session_id });
        return { success: true, message: response.message };
      }
      console.log('Auth store login failed:', response.message);
      return { success: false, message: response.message || '登录失败' };
    } catch (error) {
      console.error('Auth store login error:', error);
      return { success: false, message: '登录请求失败' };
    }
  }

  async function logout() {
    try {
      await apiService.logout();
    } catch (error) {
      console.error('Logout error:', error);
    } finally {
      isLoggedIn.value = false;
      username.value = '';
      sessionId.value = '';
    }
  }

  return {
    isLoggedIn,
    username,
    sessionId,
    isAuthenticated,
    login,
    logout
  };
});