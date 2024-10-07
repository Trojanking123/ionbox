import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-shell";
import type React from "react";

type OauthKeys = [string, string, string | null];

async function get_provider_link(provider: string): Promise<OauthKeys> {
	// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
	const keys: OauthKeys = await invoke("get_provider_link", { provider });
	return keys;
}

const OpenLink: React.FC = () => {
	// 为 url 参数指定类型
	const openInBrowser = async (url: string): Promise<void> => {
		await open(url);
	};

	return (
		<div>
			<button
				type="button"
				onClick={async () => {
					const keys: OauthKeys = await get_provider_link("Google");
					console.log(keys);
					openInBrowser(keys[0]);
				}}
			>
				Open Outlook in Browser
			</button>
		</div>
	);
};

export default OpenLink;
