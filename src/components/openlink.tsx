import React from "react";
import { open } from "@tauri-apps/plugin-shell";

const OpenLink: React.FC = () => {
  // 为 url 参数指定类型
  const openInBrowser = async (url: string): Promise<void> => {
    await open(url);
  };

  return (
    <div>
      <button onClick={() => openInBrowser("https://outlook.com")}>
        Open Outlook in Browser
      </button>
    </div>
  );
};

export default OpenLink;
