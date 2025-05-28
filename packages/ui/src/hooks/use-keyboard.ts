import { useEffect } from "react";

export const useKeyDown = (callback: (ev: KeyboardEvent) => void) => {
  useEffect(() => {
    globalThis?.window?.addEventListener("keydown", callback);
    return () => globalThis?.window?.removeEventListener("keydown", callback);
  }, [callback]);
};

export const useKeyUp = (callback: (ev: KeyboardEvent) => void) => {
  useEffect(() => {
    globalThis?.window?.addEventListener("keyup", callback);
    return () => globalThis?.window?.removeEventListener("keydown", callback);
  }, [callback]);
};
