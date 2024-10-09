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
			<Router>
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
						element={user ? <MailPage /> : <Navigate to="/login" replace />}
					/>
				</Routes>
			</Router>
		</div>
	);
};

export default App;
