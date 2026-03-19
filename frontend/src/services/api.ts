const API_BASE_URL = 'http://localhost:9001';

export interface LoginRequest {
  username: string;
  password: string;
}

export interface LoginResponse {
  success: boolean;
  message: string;
  session_id?: string;
  code?: number;
  status?: string;
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
  private sessionId: string | null = null;

  setSessionId(sessionId: string) {
    this.sessionId = sessionId;
  }

  clearSessionId() {
    this.sessionId = null;
  }

  private getHeaders(): HeadersInit {
    const headers: HeadersInit = {
      'Content-Type': 'application/json',
    };
    return headers;
  }

  async login(username: string, password: string): Promise<LoginResponse> {
    console.log('API login called with:', { username, password });
    try {
      const response = await fetch(`${API_BASE_URL}/api/login`, {
        method: 'POST',
        headers: this.getHeaders(),
        body: JSON.stringify({ username, password }),
        credentials: 'include',
      });

      console.log('API login response status:', response.status);
      console.log('API login response headers:', response.headers);

      const data = await response.json();
      console.log('API login response data:', data);
      
      const result: LoginResponse = {
        success: data.code === 0 || data.status === 'success',
        message: data.message || (data.status === 'success' ? '登录成功' : '登录失败'),
        session_id: data.session_id,
      };
      
      console.log('API login result:', result);
      
      if (result.success && result.session_id) {
        this.setSessionId(result.session_id);
      }
      
      return result;
    } catch (error) {
      console.error('API login error:', error);
      return {
        success: false,
        message: '登录请求失败: ' + (error as Error).message,
      };
    }
  }

  async logout(): Promise<ApiResponse<any>> {
    const response = await fetch(`${API_BASE_URL}/api/logout`, {
      method: 'POST',
      headers: this.getHeaders(),
      credentials: 'include',
    });

    const data = await response.json();
    
    const result: ApiResponse<any> = {
      success: data.code === 0 || data.status === 'success',
      message: data.message || (data.status === 'success' ? '退出成功' : '退出失败'),
    };
    
    this.clearSessionId();
    return result;
  }

  async getSysDictList(): Promise<PaginatedResponse<any>> {
    const response = await fetch(`${API_BASE_URL}/api/sys-dict/list`, {
      method: 'POST',
      headers: this.getHeaders(),
      body: JSON.stringify({}),
      credentials: 'include',
    });

    const data = await response.json();
    
    return {
      success: Array.isArray(data),
      message: Array.isArray(data) ? '获取成功' : '获取失败',
      records: Array.isArray(data) ? data : [],
    };
  }

  async getAddressBookPage(page: number = 1, pageSize: number = 10): Promise<PaginatedResponse<any>> {
    const response = await fetch(`${API_BASE_URL}/address-book/select-page`, {
      method: 'POST',
      headers: this.getHeaders(),
      body: JSON.stringify({ page, pageSize }),
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

  async getCategoryPage(page: number = 1, pageSize: number = 10): Promise<PaginatedResponse<any>> {
    const response = await fetch(`${API_BASE_URL}/category/select-page`, {
      method: 'POST',
      headers: this.getHeaders(),
      body: JSON.stringify({ page, pageSize }),
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

  async getDishPage(page: number = 1, pageSize: number = 10): Promise<PaginatedResponse<any>> {
    const response = await fetch(`${API_BASE_URL}/dish/select-page`, {
      method: 'POST',
      headers: this.getHeaders(),
      body: JSON.stringify({ page, pageSize }),
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

  async getDishFlavorPage(page: number = 1, pageSize: number = 10): Promise<PaginatedResponse<any>> {
    const response = await fetch(`${API_BASE_URL}/dish-flavor/select-page`, {
      method: 'POST',
      headers: this.getHeaders(),
      body: JSON.stringify({ page, pageSize }),
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

  async getEmployeePage(page: number = 1, pageSize: number = 10): Promise<PaginatedResponse<any>> {
    const response = await fetch(`${API_BASE_URL}/employee/select-page`, {
      method: 'POST',
      headers: this.getHeaders(),
      body: JSON.stringify({ page, pageSize }),
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

  async getUserPage(page: number = 1, pageSize: number = 10): Promise<PaginatedResponse<any>> {
    const response = await fetch(`${API_BASE_URL}/user/select-page`, {
      method: 'POST',
      headers: this.getHeaders(),
      body: JSON.stringify({ page, pageSize }),
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

  async getSetmealPage(page: number = 1, pageSize: number = 10): Promise<PaginatedResponse<any>> {
    const response = await fetch(`${API_BASE_URL}/setmeal/select-page`, {
      method: 'POST',
      headers: this.getHeaders(),
      body: JSON.stringify({ page, pageSize }),
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

  async getSetmealDishPage(page: number = 1, pageSize: number = 10): Promise<PaginatedResponse<any>> {
    const response = await fetch(`${API_BASE_URL}/setmeal-dish/select-page`, {
      method: 'POST',
      headers: this.getHeaders(),
      body: JSON.stringify({ page, pageSize }),
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

  async getOrdersPage(page: number = 1, pageSize: number = 10): Promise<PaginatedResponse<any>> {
    const response = await fetch(`${API_BASE_URL}/orders/select-page`, {
      method: 'POST',
      headers: this.getHeaders(),
      body: JSON.stringify({ page, pageSize }),
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

  async getOrderDetailPage(page: number = 1, pageSize: number = 10): Promise<PaginatedResponse<any>> {
    const response = await fetch(`${API_BASE_URL}/order-detail/select-page`, {
      method: 'POST',
      headers: this.getHeaders(),
      body: JSON.stringify({ page, pageSize }),
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

  async getShoppingCartPage(page: number = 1, pageSize: number = 10): Promise<PaginatedResponse<any>> {
    const response = await fetch(`${API_BASE_URL}/shopping-cart/select-page`, {
      method: 'POST',
      headers: this.getHeaders(),
      body: JSON.stringify({ page, pageSize }),
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
    const response = await fetch(`${API_BASE_URL}/join/order-with-details`, {
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
    const response = await fetch(`${API_BASE_URL}/join/setmeal-with-dishes`, {
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
    const response = await fetch(`${API_BASE_URL}/join/shopping-cart-with-items`, {
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
    const response = await fetch(`${API_BASE_URL}/join/dish-full-info`, {
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
    const response = await fetch(`${API_BASE_URL}/join/user-with-order-stats`, {
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
    const response = await fetch(`${API_BASE_URL}/join/address-book-with-user`, {
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
