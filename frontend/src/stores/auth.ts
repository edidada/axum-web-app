import { ref, computed } from 'vue';
import { defineStore } from 'pinia';
import { apiService } from '../services/api';

export const useAuthStore = defineStore('auth', () => {
  const username = ref('');
  const role = ref('');
  const initialized = ref(false);

  const isAuthenticated = computed(() => Boolean(username.value));
  const canManage = computed(() => role.value === 'admin');

  function clearUser() {
    username.value = '';
    role.value = '';
  }

  async function initialize() {
    if (initialized.value) return;
    try {
      const response = await apiService.me();
      if (response.success && response.data) {
        username.value = response.data.username;
        role.value = response.data.role;
      } else {
        clearUser();
      }
    } catch {
      clearUser();
    } finally {
      initialized.value = true;
    }
  }

  async function login(user: string, password: string) {
    try {
      const response = await apiService.login(user, password);
      if (response.success && response.username && response.role) {
        username.value = response.username;
        role.value = response.role;
        initialized.value = true;
        return { success: true, message: response.message };
      }
      return { success: false, message: response.message || '登录失败' };
    } catch {
      return { success: false, message: '登录请求失败' };
    }
  }

  async function logout() {
    try {
      await apiService.logout();
    } finally {
      clearUser();
      initialized.value = true;
    }
  }

  return {
    username,
    role,
    initialized,
    isAuthenticated,
    canManage,
    initialize,
    login,
    logout,
  };
});
