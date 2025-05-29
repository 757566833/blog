"use client"
import { useUserInfo } from "@/service";
import { Button } from "@workspace/ui/components/button"
import Link from "next/link";
import { memo, useMemo } from "react";

const Base = () => {
  const { data ,isValidating} = useUserInfo();
  console.log(data)
  if(isValidating){
    return <></>
  }
  return (
    <div>
      {data?data.account:<Button asChild >
        <Link href="/login">Login</Link>
      </Button>}
    </div>
  )
}

export const UserCenter = memo(Base) 