import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-shell";
import type React from "react";
import { useState } from "react";
import { Button } from "@/components/ui/button";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";

type OauthKeys = [string, string, string | null];
type Tokens = {
	access_token: string;
	refresh_token: string;
};

async function get_provider_link(provider: string): Promise<OauthKeys> {
	const keys: OauthKeys = await invoke("get_provider_link", { provider });
	return keys;
}

async function register(
	provider: string,
	state: string,
	verifier: string | null,
): Promise<void> {
	await invoke("register", { provider, state, verifier });
}

async function poll(state: string): Promise<Tokens> {
	console.log("polling");
	return await invoke("poll", { state });
}

interface OpenLinkProps {
	provider: string;
	avatarSrc: string;
}

const OpenLink: React.FC<OpenLinkProps> = ({ provider, avatarSrc }) => {
	const [loading, setLoading] = useState(false);
	const [tokens, setTokens] = useState<Tokens | null>(null);

	const openInBrowser = async (url: string): Promise<void> => {
		await open(url);
	};

	const handleLogin = async () => {
		setLoading(true);
		setTokens(null);
		try {
			const [url, state, verifier] = await get_provider_link(provider);
			console.log("Auth URL:", url);
			await register(provider, state, verifier);
			await openInBrowser(url);

			const receivedTokens = await poll(state);
			setTokens(receivedTokens);
		} catch (error) {
			console.error(`${provider}登录过程中出错:`, error);
		} finally {
			setLoading(false);
		}
	};

	return (
		<div className="flex items-center space-x-1">
			<Avatar className="rounded-none">
				<AvatarImage src={avatarSrc} alt={provider} className="object-contain" />
				<AvatarFallback>{provider}</AvatarFallback>
			</Avatar>
			<Button className="w-4/5" onClick={handleLogin} disabled={loading}>
				{loading ? "登录中..." : `使用${provider}登录`}
			</Button>
			{tokens && (
					<div>
						<h3>获取的令牌:</h3>
						<p>访问令牌: {tokens.access_token}</p>
						<p>刷新令牌: {tokens.refresh_token}</p>
					</div>
				)}
		</div>
	);
};

export default OpenLink;
