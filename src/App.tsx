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
import type { User } from "./types";

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
