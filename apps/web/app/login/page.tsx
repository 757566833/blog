"use client"
import { useLogin } from "@/service"
import { zodResolver } from "@hookform/resolvers/zod"
import { Button } from "@workspace/ui/components/button"
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from "@workspace/ui/components/card"
import { Form, FormControl, FormDescription, FormField, FormItem, FormLabel, FormMessage } from "@workspace/ui/components/form"
import { Input } from "@workspace/ui/components/input"
import { Label } from "@workspace/ui/components/label"
import { GalleryVerticalEnd } from "lucide-react"
import { useCallback } from "react"
import { useForm } from "react-hook-form"
import { z } from "zod"

const formSchema = z.object({
  account: z.string().min(6, {
    message: "最少6个字符",
  }).max(50, {
    message: "最多50个字符",
  }),
  password: z.string().min(6, {
    message: "最少6个字符",
  }).max(50, {
    message: "最多50个字符",
  }),
})

export default function Page() {
  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      account: "",
      password: "",
    },
  })
  const [login, loginLoading] = useLogin();
  const onSubmit = useCallback(async (data: z.infer<typeof formSchema>) => {
    console.log("Form submitted:", data)
    // 在这里处理登录逻辑
    const response = await login(data);
    if(response){
;
      // 例如，重定向到主页或显示成功消息
      window.location.href = "/";
    }
  }, [])
  return (
    <div className="flex items-center justify-center min-h-svh">
      <Form {...form}>
        <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-8">
          <div className={"flex flex-col gap-6 w-[400px] max-w-full"}>
            <div className="flex flex-col gap-6">
              <div className="flex flex-col items-center gap-2">
                <a
                  href="#"
                  className="flex flex-col items-center gap-2 font-medium"
                >
                  <div className="flex h-8 w-8 items-center justify-center rounded-md">
                    <GalleryVerticalEnd className="size-6" />
                  </div>
                  <span className="sr-only">Acme Inc.</span>
                </a>
                <h1 className="text-xl font-bold">Welcome to blog</h1>

              </div>
              <div className="flex flex-col gap-4">
                <FormField
                  control={form.control}
                  name="account"
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>账号</FormLabel>
                      <FormControl>
                        <Input placeholder="账号" {...field} />
                      </FormControl>
                      <FormDescription>

                      </FormDescription>
                      <FormMessage />
                    </FormItem>
                  )}
                />
                <FormField
                  control={form.control}
                  name="password"
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>密码</FormLabel>
                      <FormControl>
                        <Input type="password" placeholder="密码" {...field} />
                      </FormControl>
                      <FormDescription>

                      </FormDescription>
                      <FormMessage />
                    </FormItem>
                  )}
                />
                <Button type="submit" className="w-full" loading={loginLoading}>
                  登录/注册
                </Button>
              </div>

            </div>

          </div>
        </form>
      </Form>
    </div >
    // <div className="flex items-center justify-center min-h-svh">
    //   <Form {...form}>
    //     <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-8">
    //       <Card className="w-[600px]">
    //         <CardHeader>
    //           <CardTitle>注册/登录</CardTitle>
    //           <CardDescription>
    //             直接输入账号密码
    //           </CardDescription>
    //         </CardHeader>
    //         <CardContent className="space-y-2">
    //           <FormField
    //             control={form.control}
    //             name="account"
    //             render={({ field }) => (
    //               <FormItem>
    //                 <FormLabel>账号</FormLabel>
    //                 <FormControl>
    //                   <Input placeholder="账号" {...field} />
    //                 </FormControl>
    //                 <FormDescription>

    //                 </FormDescription>
    //                 <FormMessage />
    //               </FormItem>
    //             )}
    //           />
    //           <FormField
    //             control={form.control}
    //             name="password"
    //             render={({ field }) => (
    //               <FormItem>
    //                 <FormLabel>密码</FormLabel>
    //                 <FormControl>
    //                   <Input placeholder="密码" {...field} />
    //                 </FormControl>
    //                 <FormDescription>

    //                 </FormDescription>
    //                 <FormMessage />
    //               </FormItem>
    //             )}
    //           />
    //         </CardContent>
    //         <CardFooter>
    //           <Button>确定</Button>
    //         </CardFooter>
    //       </Card>
    //     </form>
    //   </Form>
    // </div>
  )
}
