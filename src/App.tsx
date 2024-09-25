import { Button } from "@/components/ui/button";
import { invoke } from "@tauri-apps/api/core";
import { onOpenUrl } from "@tauri-apps/plugin-deep-link";
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

onOpenUrl((urls) => {
	console.log("web deep link:", urls);
});

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
					<CardTitle className="text-2xl">Connect Your eMail</CardTitle>
					<CardDescription>click the link below</CardDescription>
				</CardHeader>
				<CardContent>
					<div className="grid gap-4">
						<div className="grid gap-2">
							<div className="flex items-center space-x-1">
								<Avatar>
									<AvatarImage src="https://github.com/shadcn.png" />
									<AvatarFallback>Google</AvatarFallback>
								</Avatar>
								<Button className="w-4/5">
									<a href="https://www.baidu.com">Google</a>
								</Button>
							</div>

							<div className="flex items-center space-x-1">
								<Avatar>
									<AvatarImage src="https://github.com/shadcn.png" />
									<AvatarFallback>Outlook</AvatarFallback>
								</Avatar>
								<Button className="w-4/5">
									<a href="https://www.baidu.com">Outlook</a>
								</Button>
							</div>

							<OpenLink />
						</div>
					</div>
				</CardContent>
			</Card>
		</div>
	);
}

export default App;
