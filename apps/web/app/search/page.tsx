"use client"
import { Header } from "@/components/header";
import { useArticlePage } from "@/service";
import { Pagination, PaginationContent, PaginationItem, PaginationLink } from "@workspace/ui/components/pagination";
import { datetimeRender, safeDomString } from "@workspace/ui/lib/utils";
import { useRouter, useSearchParams } from "next/navigation";
import { Suspense, useCallback, useEffect, useMemo, useRef, useState } from "react";
const MAX_PAGE_BUTTONS = 5;
function Base() {
  const router = useRouter();
  const searchParams = useSearchParams();
  const page = useMemo(() => parseInt(searchParams.get("page") || "1", 10), [searchParams]);
  const analyze = useMemo(() => searchParams.get("analyze"), [searchParams]);
  const pageSize = useMemo(() => parseInt(searchParams.get("page_size") || "10", 10), [searchParams]);
  const [query, setQuery] = useState(new URLSearchParams());
  useEffect(() => {
    const nextSearch = new URLSearchParams();
    nextSearch.append("page", page.toString());
    nextSearch.append("page_size", pageSize.toString());
    nextSearch.append("time", new Date().getTime().toString());
    if (analyze) {
      nextSearch.append("analyze", analyze);
    }
    setQuery(nextSearch);
  }, [analyze, page, pageSize]);
  const { data: articles } = useArticlePage(query);
  const totalRef = useRef(0);
  const total = useMemo(() => {
    const t = articles?.hits?.length || totalRef.current;
    totalRef.current = t;
    return t
  }, [articles?.hits?.length])
  const pageList = useMemo(() => {
    const current = (page - 1) * pageSize;
    const right = total - current;
    const rightPage = Math.ceil(right / pageSize);
    const result: number[] = [page];
    for (let i = 0; i < rightPage; i++) {
      result.push(page + i + 1);
    }
    if (result.length < MAX_PAGE_BUTTONS) {
      for (let index = 0; index < page; index++) {
        const target = page - index - 1;
        if (target < 1) {
          break;
        }
        result.unshift(target);
        if (result.length == MAX_PAGE_BUTTONS) {
          break;
        }
      }

    }
    return result;
  }, [page, pageSize, total]);
  const handlePageChange = useCallback((pageNum: number) => {
    const nextSearch = new URLSearchParams(query);
    nextSearch.set("page", pageNum.toString());
    nextSearch.append("time", new Date().getTime().toString());
    router.push(`?${nextSearch.toString()}`);
  }, [query, router]);
  const handleOpen = useCallback((id: string) => {
    globalThis?.window.open(`/note?id=${id}`);
  }, [])
  return (
    <div className="flex flex-col h-screen">
      <Header />
      <div className="flex-1 h-0 overflow-y-auto">
        {articles?.hits?.map((note) => {
          const title = note.highlight?.title?.join("...") || note._source.title;
          const content = note.highlight?.content?.join("...") || note._source.content;
          return <div key={`analyze_${note._id}`} className="p-4 border-b ">
            <h2 className="text-xl font-bold cursor-pointer" dangerouslySetInnerHTML={{ __html: safeDomString(title) }}  onClick={()=>handleOpen(note._id)}/>
            <p className="text-gray-600" dangerouslySetInnerHTML={{ __html: safeDomString(content) }} />
            <div className="text-sm text-gray-500">
              {datetimeRender(note._source.create_time)} by {note._source.account}
            </div>
          </div>
        })}
      </div>
      <div className="h-14 flex items-center justify-center border-t">
        <Pagination>
          <PaginationContent>

            {pageList.map((pageNum) => (
              <PaginationItem key={pageNum}>
                <PaginationLink onClick={() => handlePageChange(pageNum)} isActive={page === pageNum}>{pageNum}</PaginationLink>
              </PaginationItem>
            ))}

          </PaginationContent>
        </Pagination>
      </div>
    </div>
  )
}

export default function Page() {
    return <Suspense>
        <Base />
    </Suspense>
}