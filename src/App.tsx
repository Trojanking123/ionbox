import { Button } from "@/components/ui/button";
import { invoke } from "@tauri-apps/api/core";
//import { onOpenUrl } from "@tauri-apps/plugin-deep-link";
import { useState } from "react";

import "./styles.css";
import OpenLink from "@/components/openlink";
import {
	Card,
	CardContent,
	CardDescription,
	CardFooter,
	CardHeader,
	CardTitle,
} from "@/components/ui/card";

import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";

// onOpenUrl((urls) => {
// 	console.log("web deep link:", urls);
// });

function App() {
	const [greetMsg, setGreetMsg] = useState("");
	const [name, setName] = useState("");

	async function greet() {
		// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
		setGreetMsg(await invoke("greet", { name }));
	}

	return (
		<div className="container">
			<h1>Welcome to Tauri!</h1>

			<form
				className="row"
				onSubmit={(e) => {
					e.preventDefault();
					greet();
				}}
			>
				<input
					id="greet-input"
					onChange={(e) => setName(e.currentTarget.value)}
					placeholder="Enter a name..."
				/>
				<Button type="submit">Greet</Button>
			</form>

			<p>{greetMsg}</p>
			<Card className="mx-auto max-w-sm">
				<CardHeader>
					<CardTitle className="text-2xl">连接您的电子邮件</CardTitle>
					<CardDescription>点击下方链接</CardDescription>
				</CardHeader>
				<CardContent>
					<div className="grid gap-4">
						<div className="grid gap-2">
							<OpenLink provider="Google"  avatarSrc="google.svg" />
							<OpenLink provider="Outlook" avatarSrc="outlook.svg"/>
						</div>
					</div>
				</CardContent>
			</Card>
		</div>
	);
}

export default App;
