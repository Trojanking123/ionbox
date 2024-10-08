import type React from "react";
import { Link, Route, Routes } from "react-router-dom";
import EmailDashboard from "./EmailDashboard";
import Settings from "./Settings";
import type { User } from "../types";

interface MainLayoutProps {
	user: User;
	onLogout: () => void;
}

const MainLayout: React.FC<MainLayoutProps> = ({ user, onLogout }) => {
	return (
		<div className="flex">
			<nav className="w-48 p-4 bg-gray-100">
				<ul>
					<li>
						<Link to="/inbox" className="block py-2">
							收件箱
						</Link>
					</li>
					<li>
						<Link to="/settings" className="block py-2">
							设置
						</Link>
					</li>
				</ul>
				<button
					type="button"
					onClick={onLogout}
					className="mt-4 px-4 py-2 bg-red-500 text-white rounded"
				>
					登出
				</button>
			</nav>
			<main className="flex-1 p-4">
				<Routes>
					<Route path="/inbox" element={<EmailDashboard user={user} />} />
					<Route path="/settings" element={<Settings user={user} />} />
				</Routes>
			</main>
		</div>
	);
};

export default MainLayout;
