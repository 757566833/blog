"use client"
import { Header } from "@/components/header";
import { useAddNote } from "@/service";
import { zodResolver } from "@hookform/resolvers/zod";
import { Button } from "@workspace/ui/components/button";
import { Form, FormControl, FormDescription, FormField, FormItem, FormLabel, FormMessage } from "@workspace/ui/components/form";
import { Input } from "@workspace/ui/components/input";
import { MarkdownEditor } from "@workspace/ui/components/markdown-editor";
import { toast } from "@workspace/ui/components/sonner";
import React, { Suspense, useCallback } from "react";
import { useForm } from "react-hook-form"
import { z } from "zod"

const formSchema = z.object({
  title: z.string().min(3, {
    message: "最少6个字符",
  }).max(50, {
    message: "最多50个字符",
  }),
  content: z.string().min(1, {
    message: "最少1个字符",
  }),
})
function Base() {
  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      title: "",
      content: "",
    },
  })
  const [add, addLoading] = useAddNote();
  const onSubmit = useCallback(async (data: z.infer<typeof formSchema>) => {
    const response = await add(data);
    if (response) {
      toast.success("发布成功");
    }
  }, [add])
  return (
    <div className="flex flex-col min-h-svh">
      <Header />
      <div className="flex-1 w-full flex flex-col items-center p-4 ">
        <Form {...form}>
          <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-8 w-full h-full flex-1 flex">

            <div className="flex flex-col gap-4 flex-1 ">
              <FormField
                control={form.control}
                name="title"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>标题</FormLabel>
                    <FormControl>
                      <Input placeholder="标题" {...field} />
                    </FormControl>
                    <FormDescription>

                    </FormDescription>
                    <FormMessage />
                  </FormItem>
                )}
              />
              <FormField
                control={form.control}
                name="content"
                render={({ field }) => (
                  <FormItem className="flex-1 flex flex-col">
                    <FormLabel>正文</FormLabel>
                    <FormControl>
                      <div className="border rounded-md overflow-hidden flex-1 overflow-y-auto">
                        <MarkdownEditor  {...field} />
                      </div>
                    </FormControl>
                    <FormDescription>

                    </FormDescription>
                    <FormMessage />
                  </FormItem>
                )}
              />
              <Button type="submit" className="w-full" loading={addLoading}>
                发布
              </Button>
            </div>


          </form>
        </Form>

      </div>
    </div >
  )
}

export default function Page() {
    return <Suspense>
        <Base />
    </Suspense>
}