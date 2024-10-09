import type React from "react";
import { useState } from "react";
import {
	BrowserRouter as Router,
	Route,
	Routes,
	Navigate,
} from "react-router-dom";
import Login from "./components/Login";
import MainLayout from "./components/MainLayout";
import { MailNav } from "./components/MailNav";
import type { User } from "./types";
import { Archive, ArchiveX, Inbox, Send, Trash2, File } from "lucide-react";

const App: React.FC = () => {
	const [user, setUser] = useState<User | null>(null);

	const handleLogin = (username: string, accessToken: string) => {
		setUser({ username, accessToken });
	};

	const handleLogout = () => {
		setUser(null);
	};

	return (
		<Router>
			<MailNav
				isCollapsed={false}
				links={[
					{
						title: "Inbox",
						label: "128",
						icon: Inbox,
						variant: "default",
					},
					{
						title: "Drafts",
						label: "9",
						icon: File,
						variant: "ghost",
					},
					{
						title: "Sent",
						label: "",
						icon: Send,
						variant: "ghost",
					},
					{
						title: "Junk",
						label: "23",
						icon: ArchiveX,
						variant: "ghost",
					},
					{
						title: "Trash",
						label: "",
						icon: Trash2,
						variant: "ghost",
					},
					{
						title: "Archive",
						label: "",
						icon: Archive,
						variant: "ghost",
					},
				]}
			/>
			<Routes>
				<Route
					path="/login"
					element={
						user ? (
							<Navigate to="/inbox" replace />
						) : (
							<Login onLogin={handleLogin} />
						)
					}
				/>
				<Route
					path="/*"
					element={
						user ? (
							<MainLayout user={user} onLogout={handleLogout} />
						) : (
							<Navigate to="/login" replace />
						)
					}
				/>
			</Routes>
		</Router>
	);
};

export default App;
