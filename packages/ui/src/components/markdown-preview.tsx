import "@milkdown/crepe/theme/common/style.css";
import "@milkdown/crepe/theme/frame.css";
import { Crepe } from "@milkdown/crepe";
import { replaceAll } from "@milkdown/kit/utils";
import { Milkdown, MilkdownProvider, useEditor } from "@milkdown/react";
import React, { useEffect } from "react";

export type MarkdownEditorType = Crepe | null;


const MilkdownPreview: React.FC<{ value?: string }> = (props) => {
  const { value } = props;

  const [mounted, setMounted] = React.useState(false);
  const ref = React.useRef<Crepe | null>(null);

  useEditor((root) => {
    const editor = new Crepe({ root })
    editor.setReadonly(true)
    ref.current = editor;
    editor.on((listener) => {
      listener.mounted(() => {
        editor.editor.action(replaceAll(value || "", true));
      })
      listener.mounted(() => {
        editor.editor.action(() => {
          setMounted(true);
        });
      });
    });

    return editor

  }, []);
  useEffect(() => {
    if (mounted) {
      const editor = ref.current;
      if (editor) {
        editor.editor.action(replaceAll(value || "", true));
      }
    }
  }, [mounted, value])
  return <Milkdown />;
};

export const MarkdownPreview: React.FC<{ value?: string }> = (props) => {
  const { value } = props;
  return (
    <MilkdownProvider>
      <MilkdownPreview value={value} />
    </MilkdownProvider>
  );
};