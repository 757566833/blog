"use client"
import { useAddArticleScore, useArticle, useArticleScoreAverage, useArticleScorePage, usePage } from "@/service";
import { Button } from "@workspace/ui/components/button";
import { Input } from "@workspace/ui/components/input";
import { MarkdownPreview } from "@workspace/ui/components/markdown-preview";
import { Textarea } from "@workspace/ui/components/textarea";
import { datetimeRender } from "@workspace/ui/lib/utils";
import { useSearchParams } from "next/navigation";
import { Suspense, useCallback, useEffect, useMemo, useState } from "react";

function Base() {
    const searchParams = useSearchParams();
    const id = useMemo(() => searchParams.get("id"), [searchParams]);
    const { data } = useArticle(id)

    const { data: average } = useArticleScoreAverage(id);

    const { page, pageSize } = usePage();
    const [query, setQuery] = useState(new URLSearchParams());
    useEffect(() => {
        const nextSearch = new URLSearchParams();
        nextSearch.append("page", page.toString());
        nextSearch.append("page_size", pageSize.toString());
        nextSearch.append("time", new Date().getTime().toString());
        if (id) {
            nextSearch.append("article_id", id);

            setQuery(nextSearch);
        }

    }, [id, page, pageSize]);
    const { data: scores } = useArticleScorePage(query);


    return (
        <div className="flex flex-col h-screen">
            <div className="h-14 flex justify-center items-center px-4 bg-gray-100 border-b">
                {data?._source?.title || ""} {average}
            </div>
            <MarkdownPreview value={data?._source?.content || ""} />
            <div className="p-4">
                <div className="text-sm text-gray-500">
                    {data?._source?.create_time} by {data?._source?.account}
                </div>
            </div>
            <div className="p-4">
                {scores?.items?.map((score) => (
                    <div key={score.id} className="mb-4 p-2 border rounded">
                        <div className="text-sm text-gray-500">
                            {datetimeRender(score?.create_time)} by {score?.account} score: {score?.score}
                        </div>
                        <div className="mt-2">{score?.comment}</div>
                    </div>
                ))}
            </div>
            <CommentInput articleId={id} />

        </div>
    )
}


function CommentInput({ articleId }: { articleId: string | null }) {
    const [comment, setComment] = useState("");
    const [score, setScore] = useState(0);
    const [add, addLoading] = useAddArticleScore();

    const handleSubmit = useCallback(async () => {
        if (comment.trim() && articleId) {
            await add({
                article_id: articleId,
                score: score,
                comment: comment
            });
            setComment("");
        }
    }, [add, articleId, comment, score]);

    return (
        <div className="p-4 border-t flex flex-col gap-2">
            <Input value={score} onChange={(e) => setScore(Number(e.target.value))} placeholder="Score" type="number" />
            <Textarea
                value={comment}
                onChange={(e) => setComment(e.target.value)}
                placeholder="Write your comment here..."
                className="mb-2"
            />
            <Button onClick={handleSubmit} loading={addLoading}>
                提交
            </Button>
        </div>
    );
}



export default function Page() {
    return <Suspense>
        <Base />
    </Suspense>
}