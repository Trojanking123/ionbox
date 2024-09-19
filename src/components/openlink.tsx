import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-shell";
import type React from "react";

const OpenLink: React.FC = () => {
	// 为 url 参数指定类型
	const openInBrowser = async (url: string): Promise<void> => {
		await invoke("new_server", {});
		await open(url);
	};

	return (
		<div>
			<button
				onClick={() =>
					openInBrowser(
						"https://accounts.google.com/o/oauth2/auth?client_id=632451831672-mfg1ol2lofb8ntf9og1eblkmgc81hv70.apps.googleusercontent.com&redirect_uri=http://localhost:8080/callback&scope=https://mail.google.com&response_type=code",
					)
				}
			>
				Open Outlook in Browser
			</button>
		</div>
	);
};

export default OpenLink;
