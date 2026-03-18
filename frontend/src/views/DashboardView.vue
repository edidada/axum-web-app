<template>
  <div class="dashboard">
    <header class="header">
      <div class="header-left">
        <h1>后台管理系统</h1>
      </div>
      <div class="header-right">
        <span class="username">{{ authStore.username }}</span>
        <button @click="handleLogout" class="logout-btn">退出</button>
      </div>
    </header>

    <nav class="nav-menu">
      <button
        v-for="item in menuItems"
        :key="item.id"
        :class="['nav-item', { active: activeTab === item.id }]"
        @click="activeTab = item.id"
      >
        {{ item.label }}
      </button>
    </nav>

    <main class="content">
      <div class="api-section">
        <div class="api-header">
          <h2>{{ currentMenuItem.label }}</h2>
          <button @click="fetchData" :disabled="isLoading">
            {{ isLoading ? '加载中...' : '获取数据' }}
          </button>
        </div>

        <div v-if="errorMessage" class="error-message">
          {{ errorMessage }}
        </div>

        <div v-if="data.length > 0" class="data-table">
          <table>
            <thead>
              <tr>
                <th v-for="header in tableHeaders" :key="header">{{ header }}</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="(row, index) in data" :key="index">
                <td v-for="header in tableHeaders" :key="header">
                  {{ formatValue(row[header]) }}
                </td>
              </tr>
            </tbody>
          </table>
        </div>

        <div v-if="data.length === 0 && !isLoading && !errorMessage" class="empty-state">
          暂无数据，请点击"获取数据"按钮
        </div>
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import { useAuthStore } from '../stores/auth';
import { apiService } from '../services/api';

const router = useRouter();
const authStore = useAuthStore();

const activeTab = ref('dict');
const isLoading = ref(false);
const errorMessage = ref('');
const data = ref<any[]>([]);

const menuItems = [
  { id: 'dict', label: '字典管理', action: 'getSysDictList' },
  { id: 'address', label: '地址簿', action: 'getAddressBookPage' },
  { id: 'category', label: '分类管理', action: 'getCategoryPage' },
  { id: 'dish', label: '菜品管理', action: 'getDishPage' },
];

const currentMenuItem = computed(() => {
  return menuItems.find(item => item.id === activeTab.value) || menuItems[0];
});

const tableHeaders = computed(() => {
  if (data.value.length === 0) return [];
  return Object.keys(data.value[0]);
});

async function handleLogout() {
  await authStore.logout();
  router.push('/login');
}

async function fetchData() {
  isLoading.value = true;
  errorMessage.value = '';
  data.value = [];

  try {
    let response;
    const action = currentMenuItem.value.action;

    switch (action) {
      case 'getSysDictList':
        response = await apiService.getSysDictList();
        break;
      case 'getAddressBookPage':
        response = await apiService.getAddressBookPage();
        break;
      case 'getCategoryPage':
        response = await apiService.getCategoryPage();
        break;
      case 'getDishPage':
        response = await apiService.getDishPage();
        break;
      default:
        throw new Error('Unknown action');
    }

    if (response.success) {
      data.value = response.records || [];
    } else {
      errorMessage.value = response.message || '获取数据失败';
    }
  } catch (error) {
    errorMessage.value = '网络请求失败';
    console.error('API error:', error);
  } finally {
    isLoading.value = false;
  }
}

function formatValue(value: any): string {
  if (value === null || value === undefined) return '-';
  if (typeof value === 'object') return JSON.stringify(value);
  return String(value);
}

onMounted(() => {
  if (!authStore.isAuthenticated) {
    router.push('/login');
  }
});
</script>

<style scoped>
.dashboard {
  min-height: 100vh;
  background: #f5f5f5;
}

.header {
  background: white;
  padding: 1rem 2rem;
  display: flex;
  justify-content: space-between;
  align-items: center;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.header-left h1 {
  margin: 0;
  font-size: 1.5rem;
  color: #333;
}

.header-right {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.username {
  font-weight: 500;
  color: #555;
}

.logout-btn {
  padding: 0.5rem 1rem;
  background: #e53e3e;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  transition: background 0.2s;
}

.logout-btn:hover {
  background: #c53030;
}

.nav-menu {
  background: white;
  padding: 1rem 2rem;
  display: flex;
  gap: 1rem;
  border-bottom: 1px solid #e0e0e0;
}

.nav-item {
  padding: 0.5rem 1rem;
  background: none;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  color: #555;
  font-weight: 500;
  transition: all 0.2s;
}

.nav-item:hover {
  background: #f0f0f0;
}

.nav-item.active {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
}

.content {
  padding: 2rem;
}

.api-section {
  background: white;
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  padding: 1.5rem;
}

.api-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1.5rem;
}

.api-header h2 {
  margin: 0;
  color: #333;
}

.api-header button {
  padding: 0.5rem 1rem;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  transition: opacity 0.2s;
}

.api-header button:hover:not(:disabled) {
  opacity: 0.9;
}

.api-header button:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.error-message {
  color: #e53e3e;
  padding: 1rem;
  background: #fff5f5;
  border: 1px solid #feb2b2;
  border-radius: 4px;
  margin-bottom: 1rem;
}

.data-table {
  overflow-x: auto;
}

table {
  width: 100%;
  border-collapse: collapse;
}

th,
td {
  padding: 0.75rem;
  text-align: left;
  border-bottom: 1px solid #e0e0e0;
}

th {
  background: #f9f9f9;
  font-weight: 500;
  color: #333;
}

tr:hover {
  background: #f9f9f9;
}

.empty-state {
  text-align: center;
  padding: 3rem;
  color: #999;
}
</style>