import type React from "react";
import { useState } from "react";

interface LoginProps {
	onLogin: (username: string, accessToken: string) => void;
}

const Login: React.FC<LoginProps> = ({ onLogin }) => {
	const [username, setUsername] = useState("");
	const [password, setPassword] = useState("");
	const [error, setError] = useState("");

	const handleSubmit = async (e: React.FormEvent) => {
		e.preventDefault();
		setError("");

		if (!username || !password) {
			setError("请输入用户名和密码");
			return;
		}

		try {
			// 这里应该是实际的登录API调用
			// 以下是模拟的登录过程
			const response = await simulateLoginApi(username, password);
			console.log(response);
			onLogin(username, response.accessToken);
		} catch (err) {
			setError("登录失败，请检查用户名和密码");
		}
	};

	// 模拟登录API
	const simulateLoginApi = async (
		username: string,
		password: string,
	): Promise<{ accessToken: string }> => {
		// 这里应该是实际的API调用
		// 为了演示，我们只是返回一个模拟的accessToken
		return new Promise((resolve) => {
			setTimeout(() => {
				if (username === "test" && password === "password") {
					resolve({ accessToken: "simulated_access_token" });
				} else {
					throw new Error("Invalid credentials");
				}
			}, 1000);
		});
	};

	return (
		<div className="flex justify-center items-center h-screen bg-gray-100">
			<form
				onSubmit={handleSubmit}
				className="bg-white p-8 rounded shadow-md w-96"
			>
				<h2 className="text-2xl font-bold mb-6 text-center">登录</h2>
				{error && <p className="text-red-500 mb-4">{error}</p>}
				<div className="mb-4">
					<label
						htmlFor="username"
						className="block mb-2 text-sm font-medium text-gray-600"
					>
						用户名
					</label>
					<input
						type="text"
						id="username"
						value={username}
						onChange={(e) => setUsername(e.target.value)}
						className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
						required
					/>
				</div>
				<div className="mb-6">
					<label
						htmlFor="password"
						className="block mb-2 text-sm font-medium text-gray-600"
					>
						密码
					</label>
					<input
						type="password"
						id="password"
						value={password}
						onChange={(e) => setPassword(e.target.value)}
						className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
						required
					/>
				</div>
				<button
					type="submit"
					className="w-full bg-blue-500 text-white py-2 px-4 rounded-md hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-opacity-50"
				>
					登录
				</button>
			</form>
		</div>
	);
};

export default Login;
