"use client"
import { useLogout, useTokenInfo } from "@/service";
import { Avatar, AvatarFallback, AvatarImage } from "@workspace/ui/components/avatar";
import { Button } from "@workspace/ui/components/button"
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuShortcut, DropdownMenuTrigger } from "@workspace/ui/components/dropdown-menu";
import { generateAvatar } from "@workspace/ui/lib/utils";
import { LogOutIcon, NotebookPenIcon } from "lucide-react";
import Link from "next/link";
import { useRouter } from "next/navigation";
import { memo, useMemo } from "react";


const Base = () => {
  const { data, isValidating } = useTokenInfo();
  console.log(data)

  const avatar = useMemo(() => {
    if (data?.account) {
      return generateAvatar(data.account);
    }
    return ""
  }, [data?.account]);
  const [logout] = useLogout();
  const route = useRouter()
  if (isValidating) {
    return <></>
  }
  return (
    <div>
      {avatar ? <DropdownMenu>
        <DropdownMenuTrigger asChild><Avatar>
          <AvatarImage src={avatar} alt={data?.account || ""} />
          <AvatarFallback>{data?.account || ""}</AvatarFallback>
        </Avatar>
        </DropdownMenuTrigger>
        <DropdownMenuContent className="w-24">
          <DropdownMenuItem onClick={() => route.push("/write")}>
            写文章
            <DropdownMenuShortcut>
              <NotebookPenIcon />
            </DropdownMenuShortcut>
          </DropdownMenuItem>
          <DropdownMenuItem onClick={() => logout()}>
            退出
            <DropdownMenuShortcut>
              <LogOutIcon />
            </DropdownMenuShortcut>
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu> : <Button asChild >
        <Link href="/login">Login</Link>
      </Button>}
    </div>
  )
}

export const UserCenter = memo(Base) 