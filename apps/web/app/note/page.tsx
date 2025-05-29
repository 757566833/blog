"use client"
import { useNote } from "@/service";
import { MarkdownPreview } from "@workspace/ui/components/markdown-preview";
import { useSearchParams } from "next/navigation";
import { Suspense, useMemo } from "react";

function Base() {
    const searchParams = useSearchParams();
    const id = useMemo(() => searchParams.get("id"), [searchParams]);
    const { data } = useNote(id)


    return (
        <div className="flex flex-col h-screen">
            <div className="h-14 flex justify-center items-center px-4 bg-gray-100 border-b">
                {data?._source?.title || ""}
            </div>
            <MarkdownPreview value={data?._source?.content || ""} />
        </div>
    )
}
export default function Page() {
    return <Suspense>
        <Base />
    </Suspense>
}