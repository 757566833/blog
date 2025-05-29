import "@milkdown/crepe/theme/common/style.css";
import "@milkdown/crepe/theme/frame.css";
import { Editor, rootCtx } from "@milkdown/kit/core";
import { listener, listenerCtx } from "@milkdown/kit/plugin/listener";
import { commonmark } from "@milkdown/kit/preset/commonmark";
import { Milkdown, MilkdownProvider, useEditor } from "@milkdown/react";
import { nord } from "@milkdown/theme-nord";
import React from "react";

const MilkdownEditor: React.FC<{ onChange?: (value: string) => void }> = (props) => {
  const { onChange } = props;
  useEditor((root) => {
    return Editor.make()
      .config(nord)
      .config((ctx) => {
        ctx.set(rootCtx, root);
        ctx.get(listenerCtx).markdownUpdated((ctx, markdown) => {
          // Save content to your backend or storage
          onChange?.(markdown);
        });
      })
      .use(commonmark)
      .use(listener)
  }, [onChange]);
  return <Milkdown />;
};

export const MarkdownEditor: React.FC<{ onChange?: (value: string) => void }> = (props) => {
  const { onChange } = props;
  return (
    <MilkdownProvider>
      <style>{`
       [data-milkdown-root] {
  /* 你的样式写这里 */
  height: 100%;
  width: 100%;
  /* 其他样式 */
  flex: 1;
}
        `}</style>
      <MilkdownEditor onChange={onChange} />
    </MilkdownProvider>
  );
};