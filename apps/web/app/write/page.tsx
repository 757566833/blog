"use client"
import { Header } from "@/components/header";
import { zodResolver } from "@hookform/resolvers/zod";
import { Button } from "@workspace/ui/components/button";
import { Card, CardContent } from "@workspace/ui/components/card";
import { Form, FormControl, FormDescription, FormField, FormItem, FormLabel, FormMessage } from "@workspace/ui/components/form";
import { Input } from "@workspace/ui/components/input";
import { MarkdownEditor } from "@workspace/ui/components/markdown-editor";
import React, { useCallback } from "react";
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
export default function Page() {
  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      title: "",
      content: "",
    },
  })
  const onSubmit = useCallback(async (data: z.infer<typeof formSchema>) => {
    console.log(data);
  }, [])
  return (
    <div className="flex flex-col min-h-svh">
      <Header />
      <div className="flex-1 w-full flex flex-col items-center p-4">
        <Form {...form}>
          <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-8">
            <Card className="w-[600px]">
              <CardContent>
                <div className="flex flex-col gap-4">
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
                      <FormItem>
                        <FormLabel>正文</FormLabel>
                        <FormControl>
                          <div className="border rounded-md overflow-hidden max-h-[600px] overflow-y-auto">
                            <MarkdownEditor  {...field} />
                          </div>
                        </FormControl>
                        <FormDescription>

                        </FormDescription>
                        <FormMessage />
                      </FormItem>
                    )}
                  />
                  <Button type="submit" className="w-full">
                    发布
                  </Button>
                </div>

              </CardContent>
            </Card>
          </form>
        </Form>

      </div>
    </div >
  )
}
