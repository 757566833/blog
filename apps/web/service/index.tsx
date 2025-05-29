import { toast } from "@workspace/ui/components/sonner";
import { useLoading } from "@workspace/ui/hooks/use-loading";
import { LoginRequest, UserEntry } from "@workspace/ui/types/auth";
import { useCallback, useMemo, useState } from "react";
import { ESNoteEntry, ESAnalyzeNoteHighlight } from "@workspace/ui/types/note";

import useSWR from "swr";
export interface Response<T> {
  code: number;
  data: T;
  message: string;
}

export const usePage = () => {
  const [page, setPage] = useState(1);
  const [pageSize, setPageSize] = useState(20);
  const next = useCallback(() => {
    setPage((p) => p + 1);
  }, []);
  const pre = useCallback(() => {
    setPage((p) => {
      return p <= 1 ? 1 : p - 1;
    });
  }, []);
  return {
    page,
    pageSize,
    setPage,
    setPageSize,
    next,
    pre,
  };
};

export const fetcher = async (...args: Parameters<typeof fetch>) => {
  const [url, options, ...other] = args;
  if (!url) {
    return;
  }
  const response = await fetchWrapper(url, options, ...other);
  if (response) {
    if (response && response?.status >= 200 && response?.status < 300) {
      const json = await response?.json();
      if (json?.code == 0) {
        return json.data;
      } else {
        toast.error(json.message, {
          duration: 10000,
        });
      }
    } else {
      toast.error(` http error ${url} ${response?.status} `);
    }
  }
};
export const commonFetcher = async (...args: Parameters<typeof fetch>) => {
  const [url, options, ...other] = args;
  if (!url) {
    return;
  }
  const response = await fetchWrapper(url, options, ...other);
  return response;
};
const fetchWrapper = async (...args: Parameters<typeof fetch>) => {
  let res;
  const url = args[0];
  if (typeof url === "string") {
    let _url: string = url;
    if (!_url.startsWith("http")) {
      _url = `${globalThis?.location.origin}${_url}`;
    }
    const search = new URL(_url);
    args[0] = search.toString();
  } else if (url instanceof URL) {
    args[0] = url.toString();
  } else {
    let _url: string = url.url;
    if (!_url.startsWith("http")) {
      _url = `${globalThis?.location.origin}${_url}`;
    }
    const search = new URL(_url);
    args[0] = search.toString();
  }
  try {
    args[1] = {
      headers: {
        "Content-Type": "application/json",
      },
      ...args[1],
    };
    res = await fetch(...args);
  } catch (error) {
    console.error(error);

    return;
  }

  if (res.status == 401) {
    if (!globalThis?.location?.href?.includes?.("login")) {
      globalThis?.location.assign(
        `/login`
      );
    }

    return;
  }
  if (res.status == 403) {
    if (!globalThis?.location?.href?.includes?.("403")) {
      globalThis?.location.replace("/403");
    }

    return;
  }
  if (res.status >= 400) {
    // message.error("服务异常");
    return;
  }

  return res;
};




export interface IEsHits<T> {
  _index: string;
  _id: string;
  _score: number;
  _source: T;
}
export interface IEsAnalyzeHits<T, A> {
  _index: string;
  _id: string;
  _score: number;
  _source: T;
  highlight?: A;
}
export interface IChat {
  affinity: number;
  create_time: number;
  title: string;
  uid: string;
}




export interface IEsSearchResponse<T> {
  total: {
    value: number;
    relation: string;
  };
  max_score: number;
  hits: IEsHits<T>[];
}

export interface IEsAnalyzeSearchResponse<T, A> {
  total: {
    value: number;
    relation: string;
  };
  max_score: number;
  hits: IEsAnalyzeHits<T, A>[];
}
export interface IEsDetailResponse<T> {
  found: boolean;
  _id: string;
  _index: string;
  __primary_term: number;
  _seq_no: number;
  _source?: T;
}

export const login = async (data?: LoginRequest) => {
  if (!data) {
    return;
  }
  const res: { account: string } = await fetcher("/api/auth/v1/user/login", {
    method: "POST",
    body: JSON.stringify(data),
  });
  return res;
}
export const useLogin = () => {
  return useLoading(login)
}
export const useUserInfo = () => {
  return useSWR<UserEntry | undefined>(
    "/api/auth/v1/user/info",
    fetcher,
  );
}
export const fetchNotes = async (url?: string) => {
  if (url) {
    const response: IEsAnalyzeSearchResponse<
      ESNoteEntry,
      ESAnalyzeNoteHighlight
    > = await fetcher(url);
    return response;
  }
};
export const useNotePage = (params: URLSearchParams) => {
  const url = useMemo(() => {
    const paramsStr = params.toString();
    if (paramsStr) {
      return `/api/note/v1/page?${params.toString()}`;
    }
  }, [params]);
  return useSWR<
    IEsAnalyzeSearchResponse<ESNoteEntry, ESAnalyzeNoteHighlight> | undefined
  >(url, fetchNotes, {
    revalidateOnFocus: false,
  });
};