import type React from "react";
import { useState } from "react";
import {
	BrowserRouter as Router,
	Route,
	Routes,
	Navigate,
} from "react-router-dom";
import Login from "./components/Login";
import MailPage from "./components/Page";
import type { User } from "./types";
import { AccountSwitcher } from "./components/AccountSwitcher";
import { accounts } from "./components/FakeMail";

const App: React.FC = () => {
	const [user, setUser] = useState<User | null>({
		username: "aaaa",
		accessToken: "bbbb",
	});

	const handleLogin = (username: string, accessToken: string) => {
		setUser({ username, accessToken });
	};

	return (
		<div>
			<AccountSwitcher isCollapsed={false} accounts={accounts} />
		</div>
	);
};

export default App;
