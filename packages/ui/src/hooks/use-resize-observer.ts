import { useEffect, useRef, useState } from "react";

interface Size {
    width: number;
    height: number;
}

export function useResizeObserver<T extends HTMLElement>(): [React.RefObject<T | null>, Size] {
    const ref = useRef<T>(null);
    const [size, setSize] = useState<Size>({ width: 0, height: 0 });

    useEffect(() => {
        const element = ref.current;
        if (!element) return;

        const observer = new ResizeObserver(([entry]) => {
            const { width, height } = entry?.contentRect || { width: 0, height: 0 };
            setSize({ width, height });
        });

        observer.observe(element);

        return () => {
            observer.disconnect();
        };
    }, []); // 注意：React 严格模式下可能需要避免依赖 ref.current

    return [ref, size];
}
