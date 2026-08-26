const API_BASE_URL = 'http://localhost:9002';

export interface LoginRequest {
  username: string;
  password: string;
}

export interface LoginResponse {
  success: boolean;
  message: string;
  username?: string;
  role?: string;
  code?: number;
  status?: string;
}

export interface CurrentUser {
  username: string;
  role: string;
  expires_at: number;
}

export interface ApiResponse<T> {
  success: boolean;
  message: string;
  data?: T;
  code?: number;
  status?: string;
}

export interface PaginatedResponse<T> {
  success: boolean;
  message: string;
  total?: number;
  records?: T[];
  code?: number;
  status?: string;
}

class ApiService {
  private getCookie(name: string): string | null {
    const prefix = `${name}=`;
    const item = document.cookie.split(';').map(value => value.trim()).find(value => value.startsWith(prefix));
    return item ? item.slice(prefix.length) : null;
  }

  private getHeaders(): HeadersInit {
    const headers: HeadersInit = {
      'Content-Type': 'application/json',
    };
    // JWT 在 HttpOnly cookie 中，JavaScript 不可读取；这里只读取独立的 CSRF cookie。
    const csrfToken = this.getCookie('csrf_token');
    if (csrfToken) {
      headers['X-CSRF-Token'] = csrfToken;
    }
    return headers;
  }

  async login(username: string, password: string): Promise<LoginResponse> {
    try {
      const response = await fetch(`${API_BASE_URL}/api/login`, {
        method: 'POST',
        headers: this.getHeaders(),
        body: JSON.stringify({ username, password }),
        credentials: 'include',
      });

      const data = await response.json();
      
      const result: LoginResponse = {
        success: data.code === 0 || data.status === 'success',
        message: data.message || (data.status === 'success' ? '登录成功' : '登录失败'),
        username: data.username,
        role: data.role,
      };
      
      return result;
    } catch (error) {
      console.error('API login error:', error);
      return {
        success: false,
        message: '登录请求失败: ' + (error as Error).message,
      };
    }
  }

  async me(): Promise<ApiResponse<CurrentUser>> {
    const response = await fetch(`${API_BASE_URL}/api/me`, {
      method: 'GET',
      headers: this.getHeaders(),
      credentials: 'include',
    });
    if (!response.ok) {
      return { success: false, message: '会话无效' };
    }
    const data = await response.json();
    return {
      success: data.status === 'success',
      message: data.status === 'success' ? '会话有效' : '会话无效',
      data: {
        username: data.username,
        role: data.role,
        expires_at: data.expires_at,
      },
    };
  }

  async logout(): Promise<ApiResponse<any>> {
    const response = await fetch(`${API_BASE_URL}/api/logout`, {
      method: 'POST',
      headers: this.getHeaders(),
      credentials: 'include',
    });

    const data = await response.json().catch(() => ({}));
    
    const result: ApiResponse<any> = {
      success: data.code === 0 || data.status === 'success',
      message: data.message || (data.status === 'success' ? '退出成功' : '退出失败'),
    };
    
    return result;
  }

  async uploadImageFromUrl(imageUrl: string): Promise<{ success: boolean; message: string; url?: string }> {
    const response = await fetch(`${API_BASE_URL}/api/image/upload-url?url=${encodeURIComponent(imageUrl)}`, {
      method: 'POST',
      headers: this.getHeaders(),
      credentials: 'include',
    });
    const data = await response.json();
    return {
      success: response.ok && data.success === true,
      message: data.message || (response.ok ? '上传成功' : '上传失败'),
      url: data.url,
    };
  }

  async getSysDictList(params: any = {}): Promise<PaginatedResponse<any>> {
    const response = await fetch(`${API_BASE_URL}/api/sys-dict/list`, {
      method: 'POST',
      headers: this.getHeaders(),
      body: JSON.stringify(params),
      credentials: 'include',
    });

    const data = await response.json();
    
    return {
      success: Array.isArray(data),
      message: Array.isArray(data) ? '获取成功' : '获取失败',
      records: Array.isArray(data) ? data : [],
    };
  }

  async getAddressBookPage(params: any = {}, page: number = 1, page_size: number = 10): Promise<PaginatedResponse<any>> {
    const response = await fetch(`${API_BASE_URL}/address-book/select-page`, {
      method: 'POST',
      headers: this.getHeaders(),
      body: JSON.stringify({ ...params, page, page_size }),
      credentials: 'include',
    });

    const data = await response.json();
    
    return {
      success: Array.isArray(data.records),
      message: Array.isArray(data.records) ? '获取成功' : '获取失败',
      total: data.total,
      records: data.records || [],
    };
  }

  async getCategoryPage(params: any = {}, page: number = 1, page_size: number = 10): Promise<PaginatedResponse<any>> {
    const response = await fetch(`${API_BASE_URL}/category/select-page`, {
      method: 'POST',
      headers: this.getHeaders(),
      body: JSON.stringify({ ...params, page, page_size }),
      credentials: 'include',
    });

    const data = await response.json();
    
    return {
      success: Array.isArray(data.records),
      message: Array.isArray(data.records) ? '获取成功' : '获取失败',
      total: data.total,
      records: data.records || [],
    };
  }

  async getDishPage(params: any = {}, page: number = 1, page_size: number = 10): Promise<PaginatedResponse<any>> {
    const response = await fetch(`${API_BASE_URL}/dish/select-page`, {
      method: 'POST',
      headers: this.getHeaders(),
      body: JSON.stringify({ ...params, page, page_size }),
      credentials: 'include',
    });

    const data = await response.json();
    
    return {
      success: Array.isArray(data.records),
      message: Array.isArray(data.records) ? '获取成功' : '获取失败',
      total: data.total,
      records: data.records || [],
    };
  }

  async getDishFlavorPage(params: any = {}, page: number = 1, page_size: number = 10): Promise<PaginatedResponse<any>> {
    const response = await fetch(`${API_BASE_URL}/dish-flavor/select-page`, {
      method: 'POST',
      headers: this.getHeaders(),
      body: JSON.stringify({ ...params, page, page_size }),
      credentials: 'include',
    });

    const data = await response.json();
    
    return {
      success: Array.isArray(data.records),
      message: Array.isArray(data.records) ? '获取成功' : '获取失败',
      total: data.total,
      records: data.records || [],
    };
  }

  async getEmployeePage(params: any = {}, page: number = 1, page_size: number = 10): Promise<PaginatedResponse<any>> {
    const response = await fetch(`${API_BASE_URL}/employee/select-page`, {
      method: 'POST',
      headers: this.getHeaders(),
      body: JSON.stringify({ ...params, page, page_size }),
      credentials: 'include',
    });

    const data = await response.json();
    
    return {
      success: Array.isArray(data.records),
      message: Array.isArray(data.records) ? '获取成功' : '获取失败',
      total: data.total,
      records: data.records || [],
    };
  }

  async getUserPage(params: any = {}, page: number = 1, page_size: number = 10): Promise<PaginatedResponse<any>> {
    const response = await fetch(`${API_BASE_URL}/user/select-page`, {
      method: 'POST',
      headers: this.getHeaders(),
      body: JSON.stringify({ ...params, page, page_size }),
      credentials: 'include',
    });

    const data = await response.json();
    
    return {
      success: Array.isArray(data.records),
      message: Array.isArray(data.records) ? '获取成功' : '获取失败',
      total: data.total,
      records: data.records || [],
    };
  }

  async getSetmealPage(params: any = {}, page: number = 1, page_size: number = 10): Promise<PaginatedResponse<any>> {
    const response = await fetch(`${API_BASE_URL}/setmeal/select-page`, {
      method: 'POST',
      headers: this.getHeaders(),
      body: JSON.stringify({ ...params, page, page_size }),
      credentials: 'include',
    });

    const data = await response.json();
    
    return {
      success: Array.isArray(data.records),
      message: Array.isArray(data.records) ? '获取成功' : '获取失败',
      total: data.total,
      records: data.records || [],
    };
  }

  async getSetmealDishPage(params: any = {}, page: number = 1, page_size: number = 10): Promise<PaginatedResponse<any>> {
    const response = await fetch(`${API_BASE_URL}/setmeal-dish/select-page`, {
      method: 'POST',
      headers: this.getHeaders(),
      body: JSON.stringify({ ...params, page, page_size }),
      credentials: 'include',
    });

    const data = await response.json();
    
    return {
      success: Array.isArray(data.records),
      message: Array.isArray(data.records) ? '获取成功' : '获取失败',
      total: data.total,
      records: data.records || [],
    };
  }

  async getOrdersPage(params: any = {}, page: number = 1, page_size: number = 10): Promise<PaginatedResponse<any>> {
    const response = await fetch(`${API_BASE_URL}/orders/select-page`, {
      method: 'POST',
      headers: this.getHeaders(),
      body: JSON.stringify({ ...params, page, page_size }),
      credentials: 'include',
    });

    const data = await response.json();
    
    return {
      success: Array.isArray(data.records),
      message: Array.isArray(data.records) ? '获取成功' : '获取失败',
      total: data.total,
      records: data.records || [],
    };
  }

  async getOrderDetailPage(params: any = {}, page: number = 1, page_size: number = 10): Promise<PaginatedResponse<any>> {
    const response = await fetch(`${API_BASE_URL}/order-detail/select-page`, {
      method: 'POST',
      headers: this.getHeaders(),
      body: JSON.stringify({ ...params, page, page_size }),
      credentials: 'include',
    });

    const data = await response.json();
    
    return {
      success: Array.isArray(data.records),
      message: Array.isArray(data.records) ? '获取成功' : '获取失败',
      total: data.total,
      records: data.records || [],
    };
  }

  async getShoppingCartPage(params: any = {}, page: number = 1, page_size: number = 10): Promise<PaginatedResponse<any>> {
    const response = await fetch(`${API_BASE_URL}/shopping-cart/select-page`, {
      method: 'POST',
      headers: this.getHeaders(),
      body: JSON.stringify({ ...params, page, page_size }),
      credentials: 'include',
    });

    const data = await response.json();
    
    return {
      success: Array.isArray(data.records),
      message: Array.isArray(data.records) ? '获取成功' : '获取失败',
      total: data.total,
      records: data.records || [],
    };
  }

  // JOIN 查询方法
  async getOrderWithDetails(): Promise<PaginatedResponse<any>> {
    const response = await fetch(`${API_BASE_URL}/report/order-details`, {
      method: 'GET',
      headers: this.getHeaders(),
      credentials: 'include',
    });

    const data = await response.json();
    
    return {
      success: data.code === 200,
      message: data.message || '获取成功',
      records: data.data || [],
    };
  }

  async getSetmealWithDishes(): Promise<PaginatedResponse<any>> {
    const response = await fetch(`${API_BASE_URL}/report/setmeal-dishes`, {
      method: 'GET',
      headers: this.getHeaders(),
      credentials: 'include',
    });

    const data = await response.json();
    
    return {
      success: data.code === 200,
      message: data.message || '获取成功',
      records: data.data || [],
    };
  }

  async getShoppingCartWithItems(): Promise<PaginatedResponse<any>> {
    const response = await fetch(`${API_BASE_URL}/report/shopping-cart`, {
      method: 'GET',
      headers: this.getHeaders(),
      credentials: 'include',
    });

    const data = await response.json();
    
    return {
      success: data.code === 200,
      message: data.message || '获取成功',
      records: data.data || [],
    };
  }

  async getDishFullInfo(): Promise<PaginatedResponse<any>> {
    const response = await fetch(`${API_BASE_URL}/report/dish-full-info`, {
      method: 'GET',
      headers: this.getHeaders(),
      credentials: 'include',
    });

    const data = await response.json();
    
    return {
      success: data.code === 200,
      message: data.message || '获取成功',
      records: data.data || [],
    };
  }

  async getUserWithOrderStats(): Promise<PaginatedResponse<any>> {
    const response = await fetch(`${API_BASE_URL}/report/user-order-stats`, {
      method: 'GET',
      headers: this.getHeaders(),
      credentials: 'include',
    });

    const data = await response.json();
    
    return {
      success: data.code === 200,
      message: data.message || '获取成功',
      records: data.data || [],
    };
  }

  async getAddressBookWithUser(): Promise<PaginatedResponse<any>> {
    const response = await fetch(`${API_BASE_URL}/report/address-book`, {
      method: 'GET',
      headers: this.getHeaders(),
      credentials: 'include',
    });

    const data = await response.json();
    
    return {
      success: data.code === 200,
      message: data.message || '获取成功',
      records: data.data || [],
    };
  }

  async createRecord(type: string, data: any): Promise<ApiResponse<any>> {
    const url = `${API_BASE_URL}/${type}/insert`;
    const response = await fetch(url, {
      method: 'POST',
      headers: this.getHeaders(),
      body: JSON.stringify(data),
      credentials: 'include',
    });

    const result = await response.json();
    
    return {
      success: response.ok,
      message: response.ok ? '新增成功' : '新增失败',
      data: result,
    };
  }

  async updateRecord(type: string, data: any): Promise<ApiResponse<any>> {
    const url = `${API_BASE_URL}/${type}/update`;
    const response = await fetch(url, {
      method: 'POST',
      headers: this.getHeaders(),
      body: JSON.stringify(data),
      credentials: 'include',
    });

    const result = await response.json();
    
    return {
      success: response.ok,
      message: response.ok ? '更新成功' : '更新失败',
      data: result,
    };
  }

  async deleteRecord(type: string, id: string): Promise<ApiResponse<any>> {
    const url = `${API_BASE_URL}/${type}/delete`;
    const response = await fetch(url, {
      method: 'POST',
      headers: this.getHeaders(),
      body: JSON.stringify(id),
      credentials: 'include',
    });

    const result = await response.json();
    
    return {
      success: response.ok,
      message: response.ok ? '删除成功' : '删除失败',
      data: result,
    };
  }
}

export const apiService = new ApiService();
