"use client"
import { UserCenter } from "@/components/user-center";
import { useNotePage, usePage, useUserInfo } from "@/service";
import { Button } from "@workspace/ui/components/button"
import { useEffect, useState } from "react";

export default function Page() {
  const { page, pageSize, next, pre } = usePage();
  const [query, setQuery] = useState(new URLSearchParams());
  const [search, setSearch] = useState(new URLSearchParams());
  useEffect(() => {
    const nextSearch = new URLSearchParams(search);
    nextSearch.append("page", page.toString());
    nextSearch.append("page_size", pageSize.toString());
    nextSearch.append("time", new Date().getTime().toString());
    setQuery(nextSearch);
  }, [page, pageSize, search]);

    const { data: embeddings, mutate: mutateEmbeddings } = useNotePage(query);

  return (
    <div className="flex flex-col min-h-svh">
      <div className="h-14 border-b flex justify-between items-center px-4">
        <div className="text-lg font-bold">
          博客论坛
        </div>
        <div>
          <UserCenter />
        </div>
      </div>
      <div className="flex flex-col items-center justify-center gap-4">
        <h1 className="text-2xl font-bold">Hello World</h1>
        <Button size="sm">Button</Button>
      </div>
    </div>
  )
}
