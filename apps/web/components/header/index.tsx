"use client"
import { memo } from "react";
import { UserCenter } from "../user-center";


const Base = () => {
    return (
        <div className="h-14 border-b flex justify-between items-center px-4">
            <div className="text-lg font-bold">
                博客论坛
            </div>
            <div>
                <UserCenter />
            </div>
        </div>
    )
}

export const Header = memo(Base) 