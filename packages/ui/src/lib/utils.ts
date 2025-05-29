import { clsx, type ClassValue } from "clsx"
import { twMerge } from "tailwind-merge"
import { createAvatar } from '@dicebear/core';
import { lorelei } from '@dicebear/collection';

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

export const generateAvatar = (seed: string) => {
  const avatar = createAvatar(lorelei, {
    seed,
    // ... other options
  });
  return avatar.toDataUri();
}
