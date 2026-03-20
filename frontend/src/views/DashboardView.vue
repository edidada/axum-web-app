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
      <div v-for="group in menuGroups" :key="group.name" class="menu-group">
        <div class="menu-group-title">{{ group.name }}</div>
        <div class="menu-group-items">
          <button
            v-for="item in group.items"
            :key="item.id"
            :class="['nav-item', { active: activeTab === item.id }]"
            @click="activeTab = item.id"
          >
            {{ item.label }}
          </button>
        </div>
      </div>
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
                <th v-for="header in displayHeaders" :key="header">{{ headerLabels[header] || header }}</th>
                <th>操作</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="(row, index) in data" :key="index">
                <td>{{ index + 1 }}</td>
                <td v-for="header in displayHeaders" :key="header">
                  {{ formatValue(row[header], header) }}
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
  { id: 'dict', label: '字典管理', action: 'getSysDictList', type: 'sys-dict', group: '基础管理' },
  { id: 'address', label: '地址簿', action: 'getAddressBookPage', type: 'address-book', group: '基础管理' },
  { id: 'category', label: '分类管理', action: 'getCategoryPage', type: 'category', group: '基础管理' },
  { id: 'dish', label: '菜品管理', action: 'getDishPage', type: 'dish', group: '基础管理' },
  { id: 'dish_flavor', label: '菜品口味', action: 'getDishFlavorPage', type: 'dish-flavor', group: '基础管理' },
  { id: 'employee', label: '员工管理', action: 'getEmployeePage', type: 'employee', group: '基础管理' },
  { id: 'user', label: '用户管理', action: 'getUserPage', type: 'user', group: '基础管理' },
  { id: 'setmeal', label: '套餐管理', action: 'getSetmealPage', type: 'setmeal', group: '基础管理' },
  { id: 'setmeal_dish', label: '套餐菜品', action: 'getSetmealDishPage', type: 'setmeal-dish', group: '基础管理' },
  { id: 'orders', label: '订单管理', action: 'getOrdersPage', type: 'orders', group: '业务管理' },
  { id: 'order_detail', label: '订单详情', action: 'getOrderDetailPage', type: 'order-detail', group: '业务管理' },
  { id: 'shopping_cart', label: '购物车', action: 'getShoppingCartPage', type: 'shopping-cart', group: '业务管理' },
  // 报表查询菜单
  { id: 'join_order_details', label: '订单详情报表', action: 'getOrderWithDetails', type: 'join', isJoin: true, group: '报表查询' },
  { id: 'join_setmeal_dishes', label: '套餐菜品报表', action: 'getSetmealWithDishes', type: 'join', isJoin: true, group: '报表查询' },
  { id: 'join_shopping_cart', label: '购物车报表', action: 'getShoppingCartWithItems', type: 'join', isJoin: true, group: '报表查询' },
  { id: 'join_dish_full', label: '菜品信息报表', action: 'getDishFullInfo', type: 'join', isJoin: true, group: '报表查询' },
  { id: 'join_user_stats', label: '用户统计报表', action: 'getUserWithOrderStats', type: 'join', isJoin: true, group: '报表查询' },
  { id: 'join_address_user', label: '地址用户报表', action: 'getAddressBookWithUser', type: 'join', isJoin: true, group: '报表查询' },
];

const menuGroups = computed(() => {
  const groups: Record<string, any[]> = {};
  menuItems.forEach(item => {
    const group = item.group || '其他';
    if (!groups[group]) {
      groups[group] = [];
    }
    groups[group].push(item);
  });
  return Object.keys(groups).map(name => ({
    name,
    items: groups[name]
  }));
});

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

const headerLabels = computed(() => {
  const labelMap: Record<string, Record<string, string>> = {
    dict: {
      code: '编码',
      name: '名称',
      value: '值',
      sort: '排序',
      remark: '备注',
      state: '状态',
      create_date: '创建时间',
    },
    address: {
      consignee: '收货人',
      sex: '性别',
      phone: '电话',
      province_code: '省份编码',
      province_name: '省份名称',
      city_code: '城市编码',
      city_name: '城市名称',
      district_code: '区县编码',
      district_name: '区县名称',
      detail: '详细地址',
      label: '标签',
      is_default: '默认地址',
      user_id: '用户ID',
      create_time: '创建时间',
      update_time: '更新时间',
      create_user: '创建用户',
      update_user: '更新用户',
      is_deleted: '删除标记',
    },
    category: {
      name: '分类名称',
      sort: '排序',
      type: '类型',
      category_type: '分类类型',
      create_time: '创建时间',
      update_time: '更新时间',
      create_user: '创建用户',
      update_user: '更新用户',
    },
    dish: {
      name: '菜品名称',
      category_id: '分类ID',
      price: '价格',
      code: '编码',
      image: '图片',
      description: '描述',
      status: '状态',
      sort: '排序',
      create_time: '创建时间',
      update_time: '更新时间',
      create_user: '创建用户',
      update_user: '更新用户',
      is_deleted: '删除标记',
    },
    dish_flavor: {
      dish_id: '菜品ID',
      name: '口味名称',
      value: '口味值',
      create_time: '创建时间',
      update_time: '更新时间',
      create_user: '创建用户',
      update_user: '更新用户',
      is_deleted: '删除标记',
    },
    employee: {
      name: '姓名',
      username: '用户名',
      password: '密码',
      phone: '电话',
      sex: '性别',
      id_number: '身份证号',
      status: '状态',
      create_time: '创建时间',
      update_time: '更新时间',
      create_user: '创建用户',
      update_user: '更新用户',
    },
    user: {
      openid: '微信OpenID',
      name: '姓名',
      phone: '电话',
      sex: '性别',
      id_number: '身份证号',
      avatar: '头像',
      create_time: '创建时间',
      update_time: '更新时间',
    },
    setmeal: {
      category_id: '分类ID',
      name: '套餐名称',
      price: '价格',
      status: '状态',
      code: '编码',
      description: '描述',
      image: '图片',
      create_time: '创建时间',
      update_time: '更新时间',
      create_user: '创建用户',
      update_user: '更新用户',
      is_deleted: '删除标记',
    },
    setmeal_dish: {
      setmeal_id: '套餐ID',
      dish_id: '菜品ID',
      name: '菜品名称',
      price: '价格',
      copies: '份数',
      create_time: '创建时间',
      update_time: '更新时间',
      create_user: '创建用户',
      update_user: '更新用户',
    },
    orders: {
      number: '订单号',
      status: '订单状态',
      user_id: '用户ID',
      address_book_id: '地址簿ID',
      order_time: '下单时间',
      checkout_time: '结账时间',
      pay_method: '支付方式',
      pay_status: '支付状态',
      amount: '金额',
      remark: '备注',
      phone: '电话',
      address: '地址',
      user_name: '用户名',
      consignee: '收货人',
      create_time: '创建时间',
      update_time: '更新时间',
    },
    order_detail: {
      name: '商品名称',
      image: '商品图片',
      order_id: '订单ID',
      dish_id: '菜品ID',
      setmeal_id: '套餐ID',
      dish_flavor: '口味',
      number: '数量',
      amount: '金额',
      create_time: '创建时间',
    },
    shopping_cart: {
      name: '商品名称',
      image: '商品图片',
      user_id: '用户ID',
      dish_id: '菜品ID',
      setmeal_id: '套餐ID',
      dish_flavor: '口味',
      number: '数量',
      amount: '金额',
      create_time: '创建时间',
    },
    join_order_details: {
      order_id: '订单ID',
      order_number: '订单号',
      order_status: '订单状态',
      user_id: '用户ID',
      order_time: '下单时间',
      pay_method: '支付方式',
      pay_status: '支付状态',
      order_amount: '订单金额',
      order_remark: '订单备注',
      consignee: '收货人',
      phone: '电话',
      address: '地址',
      detail_id: '详情ID',
      dish_id: '菜品ID',
      setmeal_id: '套餐ID',
      dish_name: '菜品名称',
      dish_image: '菜品图片',
      dish_flavor: '口味',
      number: '数量',
      amount: '金额',
    },
    join_setmeal_dishes: {
      setmeal_id: '套餐ID',
      setmeal_name: '套餐名称',
      setmeal_price: '套餐价格',
      setmeal_status: '套餐状态',
      setmeal_code: '套餐编码',
      setmeal_description: '套餐描述',
      setmeal_image: '套餐图片',
      category_id: '分类ID',
      category_name: '分类名称',
      sd_id: '关联ID',
      copies: '份数',
      dish_id: '菜品ID',
      dish_name: '菜品名称',
      dish_price: '菜品价格',
    },
    join_shopping_cart: {
      cart_id: '购物车ID',
      user_id: '用户ID',
      dish_flavor: '口味',
      number: '数量',
      cart_amount: '购物车金额',
      create_time: '创建时间',
      item_id: '商品ID',
      item_name: '商品名称',
      item_image: '商品图片',
      item_price: '商品价格',
      item_type: '商品类型',
    },
    join_dish_full: {
      dish_id: '菜品ID',
      dish_name: '菜品名称',
      dish_price: '菜品价格',
      dish_code: '菜品编码',
      dish_image: '菜品图片',
      dish_description: '菜品描述',
      dish_status: '菜品状态',
      dish_sort: '菜品排序',
      category_id: '分类ID',
      category_name: '分类名称',
      flavors: '口味',
    },
    join_user_stats: {
      user_id: '用户ID',
      openid: '微信OpenID',
      user_name: '用户姓名',
      phone: '电话',
      sex: '性别',
      order_count: '订单数量',
      total_amount: '订单总额',
    },
    join_address_user: {
      id: '地址ID',
      user_id: '用户ID',
      user_name: '用户姓名',
      user_phone: '用户电话',
      consignee: '收货人',
      sex: '性别',
      phone: '收货人电话',
      province_code: '省份编码',
      province_name: '省份名称',
      city_code: '城市编码',
      city_name: '城市名称',
      district_code: '区县编码',
      district_name: '区县名称',
      detail: '详细地址',
      label: '标签',
      is_default: '默认地址',
    },
  };
  return labelMap[activeTab.value] || {};
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
    dish_flavor: [
      { key: 'dish_id', label: '菜品ID' },
      { key: 'name', label: '口味名称' },
      { key: 'value', label: '口味值' },
    ],
    employee: [
      { key: 'name', label: '姓名' },
      { key: 'username', label: '用户名' },
      { key: 'password', label: '密码', type: 'password' },
      { key: 'phone', label: '电话' },
      { key: 'sex', label: '性别', type: 'number' },
      { key: 'id_number', label: '身份证号' },
      { key: 'status', label: '状态', type: 'number' },
    ],
    user: [
      { key: 'openid', label: '微信OpenID' },
      { key: 'name', label: '姓名' },
      { key: 'phone', label: '电话' },
      { key: 'sex', label: '性别', type: 'number' },
      { key: 'id_number', label: '身份证号' },
      { key: 'avatar', label: '头像' },
    ],
    setmeal: [
      { key: 'category_id', label: '分类ID' },
      { key: 'name', label: '套餐名称' },
      { key: 'price', label: '价格', type: 'number' },
      { key: 'status', label: '状态', type: 'number' },
      { key: 'code', label: '编码' },
      { key: 'description', label: '描述', type: 'textarea' },
      { key: 'image', label: '图片' },
    ],
    setmeal_dish: [
      { key: 'setmeal_id', label: '套餐ID' },
      { key: 'dish_id', label: '菜品ID' },
      { key: 'name', label: '菜品名称' },
      { key: 'price', label: '价格', type: 'number' },
      { key: 'copies', label: '份数', type: 'number' },
    ],
    orders: [
      { key: 'number', label: '订单号' },
      { key: 'status', label: '订单状态', type: 'number' },
      { key: 'user_id', label: '用户ID' },
      { key: 'address_book_id', label: '地址簿ID' },
      { key: 'pay_method', label: '支付方式', type: 'number' },
      { key: 'pay_status', label: '支付状态', type: 'number' },
      { key: 'amount', label: '金额', type: 'number' },
      { key: 'remark', label: '备注', type: 'textarea' },
      { key: 'phone', label: '电话' },
      { key: 'address', label: '地址', type: 'textarea' },
      { key: 'user_name', label: '用户名' },
      { key: 'consignee', label: '收货人' },
    ],
    order_detail: [
      { key: 'name', label: '商品名称' },
      { key: 'image', label: '商品图片' },
      { key: 'order_id', label: '订单ID' },
      { key: 'dish_id', label: '菜品ID' },
      { key: 'setmeal_id', label: '套餐ID' },
      { key: 'dish_flavor', label: '口味' },
      { key: 'number', label: '数量', type: 'number' },
      { key: 'amount', label: '金额', type: 'number' },
    ],
    shopping_cart: [
      { key: 'name', label: '商品名称' },
      { key: 'image', label: '商品图片' },
      { key: 'user_id', label: '用户ID' },
      { key: 'dish_id', label: '菜品ID' },
      { key: 'setmeal_id', label: '套餐ID' },
      { key: 'dish_flavor', label: '口味' },
      { key: 'number', label: '数量', type: 'number' },
      { key: 'amount', label: '金额', type: 'number' },
    ],
    // JOIN 查询字段配置
    join_order_details: [],
    join_setmeal_dishes: [],
    join_shopping_cart: [],
    join_dish_full: [],
    join_user_stats: [],
    join_address_user: [],
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
      case 'getDishFlavorPage':
        response = await apiService.getDishFlavorPage();
        break;
      case 'getEmployeePage':
        response = await apiService.getEmployeePage();
        break;
      case 'getUserPage':
        response = await apiService.getUserPage();
        break;
      case 'getSetmealPage':
        response = await apiService.getSetmealPage();
        break;
      case 'getSetmealDishPage':
        response = await apiService.getSetmealDishPage();
        break;
      case 'getOrdersPage':
        response = await apiService.getOrdersPage();
        break;
      case 'getOrderDetailPage':
        response = await apiService.getOrderDetailPage();
        break;
      case 'getShoppingCartPage':
        response = await apiService.getShoppingCartPage();
        break;
      // JOIN 查询
      case 'getOrderWithDetails':
        response = await apiService.getOrderWithDetails();
        break;
      case 'getSetmealWithDishes':
        response = await apiService.getSetmealWithDishes();
        break;
      case 'getShoppingCartWithItems':
        response = await apiService.getShoppingCartWithItems();
        break;
      case 'getDishFullInfo':
        response = await apiService.getDishFullInfo();
        break;
      case 'getUserWithOrderStats':
        response = await apiService.getUserWithOrderStats();
        break;
      case 'getAddressBookWithUser':
        response = await apiService.getAddressBookWithUser();
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

function formatDateTime(value: any): string {
  if (!value) return '-';
  
  try {
    let date: Date;
    
    // 处理数字类型（时间戳）
    if (typeof value === 'number') {
      // 如果是13位时间戳（毫秒），转换为Date
      date = new Date(value);
    } else if (typeof value === 'string') {
      // 处理ISO格式字符串
      date = new Date(value);
    } else {
      return String(value);
    }
    
    // 检查日期是否有效
    if (isNaN(date.getTime())) {
      return String(value);
    }
    
    // 格式化为 yyyyMMdd hhmmss
    const year = date.getFullYear();
    const month = String(date.getMonth() + 1).padStart(2, '0');
    const day = String(date.getDate()).padStart(2, '0');
    const hours = String(date.getHours()).padStart(2, '0');
    const minutes = String(date.getMinutes()).padStart(2, '0');
    const seconds = String(date.getSeconds()).padStart(2, '0');
    
    return `${year}${month}${day} ${hours}${minutes}${seconds}`;
  } catch (e) {
    return String(value);
  }
}

function formatValue(value: any, header?: string): string {
  if (value === null || value === undefined) return '-';
  
  // 检查是否是时间字段
  if (header && (header.includes('time') || header.includes('Time'))) {
    return formatDateTime(value);
  }
  
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
    
    const submitData = { ...formData.value };
    
    if (submitData.amount !== undefined && submitData.amount !== null && submitData.amount !== '') {
      submitData.amount = String(submitData.amount);
    }
    if (submitData.price !== undefined && submitData.price !== null && submitData.price !== '') {
      submitData.price = String(submitData.price);
    }
    
    let response;
    
    if (isEdit.value && editingId.value) {
      submitData.id = editingId.value;
      response = await apiService.updateRecord(type, submitData);
    } else {
      response = await apiService.createRecord(type, submitData);
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
  padding: 1rem;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  display: flex;
  flex-wrap: wrap;
  gap: 1rem;
}

.menu-group {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.menu-group-title {
  font-size: 0.85rem;
  color: #888;
  font-weight: 600;
  padding: 0.25rem 0.5rem;
  border-bottom: 1px solid #eee;
}

.menu-group-items {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
}

.nav-item {
  padding: 0.5rem 1rem;
  border: none;
  background: #f8f9fa;
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
