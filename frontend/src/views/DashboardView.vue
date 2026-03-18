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
          <div class="header-actions">
            <button @click="showAddModal" class="btn-primary">
              新增
            </button>
            <button @click="fetchData" :disabled="isLoading" class="btn-secondary">
              {{ isLoading ? '加载中...' : '刷新' }}
            </button>
          </div>
        </div>

        <div v-if="errorMessage" class="error-message">
          {{ errorMessage }}
        </div>

        <div v-if="data.length > 0" class="data-table">
          <table>
            <thead>
              <tr>
                <th>序号</th>
                <th v-for="header in displayHeaders" :key="header">{{ header }}</th>
                <th>操作</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="(row, index) in data" :key="index">
                <td>{{ index + 1 }}</td>
                <td v-for="header in displayHeaders" :key="header">
                  {{ formatValue(row[header]) }}
                </td>
                <td class="action-cell">
                  <button @click="showEditModal(row)" class="btn-edit">编辑</button>
                  <button @click="handleDelete(row)" class="btn-delete">删除</button>
                </td>
              </tr>
            </tbody>
          </table>
        </div>

        <div v-if="data.length === 0 && !isLoading && !errorMessage" class="empty-state">
          暂无数据，请点击"新增"按钮添加数据
        </div>
      </div>
    </main>

    <div v-if="showModal" class="modal-overlay" @click.self="closeModal">
      <div class="modal">
        <div class="modal-header">
          <h3>{{ isEdit ? '编辑' : '新增' }}{{ currentMenuItem.label }}</h3>
          <button @click="closeModal" class="close-btn">&times;</button>
        </div>
        <div class="modal-body">
          <div v-for="field in editableFields" :key="field.key" class="form-group">
            <label :for="field.key">{{ field.label }}</label>
            <input
              v-if="field.type !== 'textarea'"
              :id="field.key"
              v-model="formData[field.key]"
              :type="field.type || 'text'"
              :placeholder="'请输入' + field.label"
            />
            <textarea
              v-else
              :id="field.key"
              v-model="formData[field.key]"
              :placeholder="'请输入' + field.label"
              rows="3"
            ></textarea>
          </div>
        </div>
        <div class="modal-footer">
          <button @click="closeModal" class="btn-cancel">取消</button>
          <button @click="handleSubmit" :disabled="isSubmitting" class="btn-primary">
            {{ isSubmitting ? '提交中...' : '确定' }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue';
import { useRouter } from 'vue-router';
import { useAuthStore } from '../stores/auth';
import { apiService } from '../services/api';

const router = useRouter();
const authStore = useAuthStore();

const activeTab = ref('dict');
const isLoading = ref(false);
const errorMessage = ref('');
const data = ref<any[]>([]);
const showModal = ref(false);
const isEdit = ref(false);
const isSubmitting = ref(false);
const formData = ref<any>({});
const editingId = ref<string | null>(null);

const menuItems = [
  { id: 'dict', label: '字典管理', action: 'getSysDictList', type: 'api/sys-dict' },
  { id: 'address', label: '地址簿', action: 'getAddressBookPage', type: 'address-book' },
  { id: 'category', label: '分类管理', action: 'getCategoryPage', type: 'category' },
  { id: 'dish', label: '菜品管理', action: 'getDishPage', type: 'dish' },
];

const currentMenuItem = computed(() => {
  return menuItems.find(item => item.id === activeTab.value) || menuItems[0];
});

const tableHeaders = computed(() => {
  if (data.value.length === 0) return [];
  return Object.keys(data.value[0]);
});

const displayHeaders = computed(() => {
  return tableHeaders.value.filter(h => h !== 'id');
});

const editableFields = computed(() => {
  const fieldConfigs: Record<string, { key: string; label: string; type?: string }[]> = {
    dict: [
      { key: 'code', label: '编码' },
      { key: 'name', label: '名称' },
      { key: 'value', label: '值' },
      { key: 'sort', label: '排序', type: 'number' },
      { key: 'remark', label: '备注', type: 'textarea' },
    ],
    address: [
      { key: 'consignee', label: '收货人' },
      { key: 'sex', label: '性别' },
      { key: 'phone', label: '电话' },
      { key: 'province_code', label: '省份编码' },
      { key: 'province_name', label: '省份名称' },
      { key: 'city_code', label: '城市编码' },
      { key: 'city_name', label: '城市名称' },
      { key: 'district_code', label: '区县编码' },
      { key: 'district_name', label: '区县名称' },
      { key: 'detail', label: '详细地址', type: 'textarea' },
      { key: 'label', label: '标签' },
    ],
    category: [
      { key: 'name', label: '分类名称' },
      { key: 'sort', label: '排序', type: 'number' },
      { key: 'type', label: '类型', type: 'number' },
    ],
    dish: [
      { key: 'name', label: '菜品名称' },
      { key: 'category_id', label: '分类ID' },
      { key: 'price', label: '价格', type: 'number' },
      { key: 'code', label: '编码' },
      { key: 'image', label: '图片' },
      { key: 'description', label: '描述', type: 'textarea' },
      { key: 'status', label: '状态', type: 'number' },
      { key: 'sort', label: '排序', type: 'number' },
    ],
  };
  return fieldConfigs[activeTab.value] || [];
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

function showAddModal() {
  isEdit.value = false;
  editingId.value = null;
  formData.value = {};
  editableFields.value.forEach(field => {
    formData.value[field.key] = field.type === 'number' ? 0 : '';
  });
  showModal.value = true;
}

function showEditModal(row: any) {
  isEdit.value = true;
  editingId.value = row.id;
  formData.value = { ...row };
  showModal.value = true;
}

function closeModal() {
  showModal.value = false;
  formData.value = {};
  editingId.value = null;
}

async function handleSubmit() {
  if (isSubmitting.value) return;
  
  isSubmitting.value = true;
  try {
    const type = currentMenuItem.value.type;
    let response;
    
    if (isEdit.value && editingId.value) {
      formData.value.id = editingId.value;
      response = await apiService.updateRecord(type, formData.value);
    } else {
      response = await apiService.createRecord(type, formData.value);
    }

    if (response.success) {
      closeModal();
      await fetchData();
    } else {
      errorMessage.value = response.message || '操作失败';
    }
  } catch (error) {
    errorMessage.value = '操作失败';
    console.error('Submit error:', error);
  } finally {
    isSubmitting.value = false;
  }
}

async function handleDelete(row: any) {
  if (!confirm('确定要删除这条记录吗？')) return;
  
  try {
    const id = row.id;
    if (!id) {
      errorMessage.value = '缺少记录ID';
      return;
    }
    
    const type = currentMenuItem.value.type;
    const response = await apiService.deleteRecord(type, id);
    
    if (response.success) {
      await fetchData();
    } else {
      errorMessage.value = response.message || '删除失败';
    }
  } catch (error) {
    errorMessage.value = '删除失败';
    console.error('Delete error:', error);
  }
}

watch(activeTab, () => {
  fetchData();
});

onMounted(() => {
  if (!authStore.isAuthenticated) {
    router.push('/login');
  } else {
    fetchData();
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

.header-actions {
  display: flex;
  gap: 0.5rem;
}

.btn-primary {
  padding: 0.5rem 1rem;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  transition: opacity 0.2s;
}

.btn-primary:hover:not(:disabled) {
  opacity: 0.9;
}

.btn-primary:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.btn-secondary {
  padding: 0.5rem 1rem;
  background: #f0f0f0;
  color: #333;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  transition: background 0.2s;
}

.btn-secondary:hover:not(:disabled) {
  background: #e0e0e0;
}

.btn-secondary:disabled {
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

.action-cell {
  display: flex;
  gap: 0.5rem;
}

.btn-edit {
  padding: 0.25rem 0.75rem;
  background: #3182ce;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 0.875rem;
  transition: background 0.2s;
}

.btn-edit:hover {
  background: #2c5282;
}

.btn-delete {
  padding: 0.25rem 0.75rem;
  background: #e53e3e;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 0.875rem;
  transition: background 0.2s;
}

.btn-delete:hover {
  background: #c53030;
}

.empty-state {
  text-align: center;
  padding: 3rem;
  color: #999;
}

.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal {
  background: white;
  border-radius: 8px;
  width: 90%;
  max-width: 500px;
  max-height: 90vh;
  overflow-y: auto;
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem 1.5rem;
  border-bottom: 1px solid #e0e0e0;
}

.modal-header h3 {
  margin: 0;
  color: #333;
}

.close-btn {
  background: none;
  border: none;
  font-size: 1.5rem;
  cursor: pointer;
  color: #999;
  padding: 0;
  line-height: 1;
}

.close-btn:hover {
  color: #333;
}

.modal-body {
  padding: 1.5rem;
}

.form-group {
  margin-bottom: 1rem;
}

.form-group label {
  display: block;
  margin-bottom: 0.5rem;
  font-weight: 500;
  color: #333;
}

.form-group input,
.form-group textarea {
  width: 100%;
  padding: 0.5rem;
  border: 1px solid #d0d0d0;
  border-radius: 4px;
  font-size: 1rem;
  box-sizing: border-box;
}

.form-group input:focus,
.form-group textarea:focus {
  outline: none;
  border-color: #667eea;
}

.modal-footer {
  display: flex;
  justify-content: flex-end;
  gap: 0.5rem;
  padding: 1rem 1.5rem;
  border-top: 1px solid #e0e0e0;
}

.btn-cancel {
  padding: 0.5rem 1rem;
  background: #f0f0f0;
  color: #333;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  transition: background 0.2s;
}

.btn-cancel:hover {
  background: #e0e0e0;
}
</style>
