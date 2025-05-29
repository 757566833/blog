"use client"
import { memo, useCallback, useEffect, useState } from "react";
import { UserCenter } from "../user-center";
import { Input } from "@workspace/ui/components/input";
import { useRouter, useSearchParams } from "next/navigation";
import Link from "next/link";


const Base = () => {
    const searchParams = useSearchParams();
    const router = useRouter();
    const initialAnalyze = searchParams.get("analyze") || "";

    const [analyze, setAnalyze] = useState("");
    const handleAnalyzeChange = useCallback((e: React.ChangeEvent<HTMLInputElement>) => {
        setAnalyze(e.target.value);
    }, []);
    useEffect(() => {
        setAnalyze(initialAnalyze);
    }, [initialAnalyze])

    const handleSubmit = useCallback((e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        const searchParams = new URLSearchParams();
        searchParams.append("analyze", analyze);
        searchParams.append("time", new Date().getTime().toString());
        router.push(`/search?${searchParams.toString()}`);
    }, [analyze, router]);
    return (
        <div className="h-14 border-b flex justify-between items-center px-4 gap-8">
            <Link className="text-lg font-bold" href="/">
                博客论坛
            </Link>
            <div className="flex-1">
                <form onSubmit={handleSubmit}>
                    <Input placeholder="搜索" value={analyze} onChange={handleAnalyzeChange} />
                </form>

            </div>

            <div>
                <UserCenter />
            </div>
        </div>
    )
}

export const Header = memo(Base) 