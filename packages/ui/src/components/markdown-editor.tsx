import "@milkdown/crepe/theme/common/style.css";
import "@milkdown/crepe/theme/frame.css";
import { Crepe } from "@milkdown/crepe";
import { Milkdown, MilkdownProvider, useEditor } from "@milkdown/react";
import React from "react";

export type MarkdownEditorType = Crepe | null;


const MilkdownEditor: React.FC<{ onChange?: (value: string) => void, ref?: React.RefObject<Crepe | null> | React.RefCallback<Crepe | null> }> = (props) => {
  const { onChange, ref } = props;

  const innerRef = React.useRef<Crepe | null>(null);
  useEditor((root) => {
    const editor = new Crepe({ root })
    editor.on((listener) => {
      listener.markdownUpdated((ctx, markdown) => {
        onChange?.(markdown);
      });
    });
    innerRef.current = editor;
    if (ref) {
      if (typeof ref === "function") {
        ref(editor);
      } else if ("current" in ref) {
        ref.current = editor;
      }
    }

    return editor
  }, [onChange]);
  return <Milkdown />;
};

export const MarkdownEditor: React.FC<{ onChange?: (value: string) => void, ref?: React.RefObject<Crepe | null> | React.RefCallback<Crepe | null> }> = (props) => {
  const { onChange, ref } = props;
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
      <MilkdownEditor onChange={onChange} ref={ref} />
    </MilkdownProvider>
  );
};