import { lorelei } from '@dicebear/collection';
import { createAvatar } from '@dicebear/core';
import { clsx, type ClassValue } from "clsx";
import dayjs from "dayjs";
import DOMPurify from "dompurify";
import { twMerge } from "tailwind-merge";

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


function normalizeToDayjs(value: string | number): dayjs.Dayjs | null {
  if (value === null || value === undefined || value === "") return null;

  const num = Number(value);
  if (!isNaN(num)) {
    // 判断是秒级还是毫秒级时间戳
    const date = num < 1e12 ? dayjs.unix(num) : dayjs(num);
    return date.isValid() ? date : null;
  }

  const parsed = dayjs(value);
  return parsed.isValid() ? parsed : null;
}

export function timeRender(value?: string | number | null): string {
  if (value == null) return "-";
  const d = normalizeToDayjs(value);
  return d ? d.format("HH:mm:ss") : "-";
}

export function dateRender(value?: string | number | null): string {
  if (value == null) return "-";
  const d = normalizeToDayjs(value);
  return d ? d.format("YYYY-MM-DD") : "-";
}

export function datetimeRender(value?: string | number | null): string {
  if (value == null) return "-";
  const d = normalizeToDayjs(value);
  return d ? d.format("YYYY-MM-DD HH:mm:ss") : "-";
}

export function hourMinuteRender(value?: string | number | null): string {
  if (value == null) return "-";
  const d = normalizeToDayjs(value);
  return d ? d.format("HH:mm") : "-";
}

export function safeDomString(htmlString: string) {
  const cleanHtml = DOMPurify.sanitize(htmlString);
  return cleanHtml;
}