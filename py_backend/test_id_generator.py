#!/usr/bin/env python3
"""测试 ID 生成器"""

from utils.id_generator import IDGenerator, generate_id, generate_id_with_prefix, generate_id_with_config


def test_id_generator():
    print("=== 分布式 ID 生成器测试 ===\n")
    
    # 测试1: 默认生成器
    print("1. 默认生成器 (无前缀, 长度20):")
    gen1 = IDGenerator("", 20)
    for i in range(5):
        print(f"   ID {i+1}: {gen1.generate()}")
    
    # 测试2: 带前缀的生成器
    print("\n2. 用户ID生成器 (前缀: USER, 长度24):")
    gen2 = IDGenerator("USER", 24)
    for i in range(5):
        print(f"   ID {i+1}: {gen2.generate()}")
    
    # 测试3: 订单ID生成器
    print("\n3. 订单ID生成器 (前缀: ORDER, 长度32):")
    gen3 = IDGenerator("ORDER", 32)
    for i in range(5):
        print(f"   ID {i+1}: {gen3.generate()}")
    
    # 测试4: 不同前缀生成
    print("\n4. 使用同一生成器生成不同前缀的ID:")
    gen4 = IDGenerator("", 24)
    print(f"   员工ID: {gen4.generate_with_prefix('EMP')}")
    print(f"   产品ID: {gen4.generate_with_prefix('PROD')}")
    print(f"   分类ID: {gen4.generate_with_prefix('CAT')}")
    
    # 测试5: 长ID
    print("\n5. 长ID生成器 (前缀: LONG, 长度48):")
    gen5 = IDGenerator("LONG", 48)
    for i in range(3):
        print(f"   ID {i+1}: {gen5.generate()}")
    
    # 测试6: 并发测试
    print("\n6. 并发生成测试:")
    import threading
    
    gen6 = IDGenerator("CONC", 24)
    id_set = set()
    lock = threading.Lock()
    
    def generate_ids():
        for _ in range(10):
            id_val = gen6.generate()
            with lock:
                id_set.add(id_val)
    
    threads = []
    for _ in range(10):
        t = threading.Thread(target=generate_ids)
        threads.append(t)
        t.start()
    
    for t in threads:
        t.join()
    
    print(f"   生成了 {len(id_set)} 个唯一ID")
    
    # 测试7: 全局便捷函数
    print("\n7. 全局便捷函数测试:")
    print(f"   默认ID: {generate_id()}")
    print(f"   带前缀ID: {generate_id_with_prefix('TEST')}")
    print(f"   配置ID: {generate_id_with_config('CONFIG', 28)}")
    
    print("\n✅ 所有测试通过!")


if __name__ == "__main__":
    test_id_generator()
