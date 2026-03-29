import time
import random
import threading
from typing import Optional


class IDGenerator:
    """
    分布式ID生成器
    支持自定义前缀和总长度
    格式: 前缀-时间戳(16进制)-随机数-序列号
    示例: USER-019D3A428AA0-7999-000
    """
    
    _instance = None
    _lock = threading.Lock()
    
    def __new__(cls, prefix: str = "", total_len: int = 20):
        if cls._instance is None:
            with cls._lock:
                if cls._instance is None:
                    cls._instance = super().__new__(cls)
                    cls._instance._initialized = False
        return cls._instance
    
    def __init__(self, prefix: str = "", total_len: int = 20):
        if self._initialized:
            return
            
        self._lock = threading.Lock()
        self.prefix = prefix
        self.total_len = max(10, min(64, total_len))
        self.sequence = 0
        self.last_time = 0
        
        # 计算各部分长度
        self._calculate_lengths()
        self._initialized = True
    
    def _calculate_lengths(self):
        """计算各部分长度"""
        prefix_len = len(self.prefix)
        separator_len = 1 if prefix_len > 0 else 0
        
        # 可用长度 = 总长度 - 前缀长度 - 分隔符长度
        available_len = self.total_len - prefix_len - separator_len
        
        # 序列号长度（至少3位）
        self.seq_len = 4 if available_len > 20 else 3
        
        # 随机数长度（至少4位）
        self.random_len = 6 if available_len > 25 else 4
        
        # 时间戳长度 = 剩余长度
        self.time_len = available_len - self.random_len - self.seq_len
        if self.time_len < 8:
            self.time_len = 8
            # 如果不够，减少随机数长度
            if available_len < self.time_len + self.random_len + self.seq_len:
                self.random_len = max(2, available_len - self.time_len - self.seq_len)
    
    def generate(self, prefix: Optional[str] = None) -> str:
        """生成ID"""
        with self._lock:
            now = int(time.time() * 1000)  # 毫秒时间戳
            
            # 如果是同一毫秒，递增序列号
            if now == self.last_time:
                self.sequence += 1
            else:
                self.sequence = 0
                self.last_time = now
            
            use_prefix = prefix if prefix is not None else self.prefix
            
            # 构建ID
            parts = []
            
            # 1. 前缀
            if use_prefix:
                parts.append(use_prefix)
            
            # 2. 时间戳（16进制，大写）
            time_hex = format(now, f'0{self.time_len}X')
            if len(time_hex) > self.time_len:
                time_hex = time_hex[-self.time_len:]
            parts.append(time_hex)
            
            # 3. 随机数
            random_num = random.randint(0, 10 ** self.random_len - 1)
            random_str = format(random_num, f'0{self.random_len}d')
            parts.append(random_str)
            
            # 4. 序列号
            seq_num = self.sequence % (10 ** self.seq_len)
            seq_str = format(seq_num, f'0{self.seq_len}d')
            parts.append(seq_str)
            
            return '-'.join(parts)
    
    def generate_with_prefix(self, prefix: str) -> str:
        """使用指定前缀生成ID"""
        return self.generate(prefix)
    
    def get_info(self) -> dict:
        """获取生成器信息"""
        return {
            "prefix": self.prefix,
            "total_length": self.total_len,
            "time_length": self.time_len,
            "random_length": self.random_len,
            "seq_length": self.seq_len
        }


# 默认生成器实例
_default_generator = IDGenerator("", 20)


def generate_id(prefix: Optional[str] = None) -> str:
    """使用默认生成器生成ID"""
    return _default_generator.generate(prefix)


def generate_id_with_prefix(prefix: str) -> str:
    """使用指定前缀生成ID"""
    return _default_generator.generate_with_prefix(prefix)


def generate_id_with_config(prefix: str, total_len: int) -> str:
    """使用完整配置生成ID"""
    gen = IDGenerator(prefix, total_len)
    return gen.generate()
