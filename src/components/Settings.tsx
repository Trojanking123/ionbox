import type React from "react";
import type { User } from "../types";

interface SettingsProps {
	user: User;
}

const Settings: React.FC<SettingsProps> = ({ user }) => {
	return (
		<div>
			<h2>设置</h2>
			<p>用户：{user.username}</p>
			{/* 这里添加设置选项和其他相关功能 */}
		</div>
	);
};

export default Settings;
