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
    if (this.sessionId) {
      headers['Cookie'] = `session_id=${this.sessionId}`;
    }
    return headers;
  }

  async login(username: string, password: string): Promise<LoginResponse> {
    const response = await fetch(`${API_BASE_URL}/api/login`, {
      method: 'POST',
      headers: this.getHeaders(),
      body: JSON.stringify({ username, password }),
      credentials: 'include',
    });

    const data = await response.json();
    
    // 处理后端返回的响应格式
    const result: LoginResponse = {
      success: data.code === 0 || data.status === 'success',
      message: data.message || (data.status === 'success' ? '登录成功' : '登录失败'),
      session_id: data.session_id,
    };
    
    if (result.success && result.session_id) {
      this.setSessionId(result.session_id);
    }
    
    return result;
  }

  async logout(): Promise<ApiResponse<any>> {
    const response = await fetch(`${API_BASE_URL}/api/logout`, {
      method: 'POST',
      headers: this.getHeaders(),
      credentials: 'include',
    });

    const data = await response.json();
    
    // 处理后端返回的响应格式
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
    
    // 处理后端返回的响应格式
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
    
    // 处理后端返回的响应格式
    return {
      success: data.code === 0 || data.status === 'success',
      message: data.message || (data.status === 'success' ? '获取成功' : '获取失败'),
      total: data.total,
      records: data.records,
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
    
    // 处理后端返回的响应格式
    return {
      success: data.code === 0 || data.status === 'success',
      message: data.message || (data.status === 'success' ? '获取成功' : '获取失败'),
      total: data.total,
      records: data.records,
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
    
    // 处理后端返回的响应格式
    return {
      success: data.code === 0 || data.status === 'success',
      message: data.message || (data.status === 'success' ? '获取成功' : '获取失败'),
      total: data.total,
      records: data.records,
    };
  }
}

export const apiService = new ApiService();