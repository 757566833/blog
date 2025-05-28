import { useState, useEffect, useCallback, useRef } from 'react';

/**
 * 用法示例：
 * const [value, setValue] = useLocalStorage('key', 'default');
 */
export function useLocalStorage<T>(key: string, initialValue: T) {
    const initValueRef = useRef(initialValue)
  // 先尝试从 localStorage 读取
  const readValue = useCallback(() => {
    if (typeof window === 'undefined') return initValueRef.current;
    try {
      const item = window.localStorage.getItem(key);
      return item ? JSON.parse(item) : initValueRef.current;
    } catch (error) {
      console.warn(`读取 localStorage key "${key}" 失败`, error);
      return initValueRef.current;
    }
  },[key]);

  const [storedValue, setStoredValue] = useState<T>(readValue);

  // 保存到 localStorage，同时刷新 state
  const setValue = useCallback(
    (value: T | ((val: T) => T)) => {
      try {
        const valueToStore = value instanceof Function ? value(storedValue) : value;
        setStoredValue(valueToStore);
        if (typeof window !== 'undefined') {
          window.localStorage.setItem(key, JSON.stringify(valueToStore));
          window.dispatchEvent(new Event('local-storage')); // 触发一个刷新事件
        }
      } catch (error) {
        console.warn(`设置 localStorage key "${key}" 失败`, error);
      }
    },
    [key, storedValue]
  );

  // 监听 storage 事件或自己发出的 local-storage 事件
  useEffect(() => {
    const handleStorageChange = () => {
      setStoredValue(readValue());
    };

    window.addEventListener('storage', handleStorageChange);
    window.addEventListener('local-storage', handleStorageChange);

    return () => {
      window.removeEventListener('storage', handleStorageChange);
      window.removeEventListener('local-storage', handleStorageChange);
    };
  }, [key, readValue]);

  return [storedValue, setValue] as const;
}
