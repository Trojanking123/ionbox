import type React from "react";
import type { User } from "../types";

interface EmailDashboardProps {
	user: User;
}

const EmailDashboard: React.FC<EmailDashboardProps> = ({ user }) => {
	return (
		<div>
			<h2>收件箱</h2>
			<p>欢迎，{user.username}！</p>
			{/* 这里添加邮件列表和其他相关功能 */}
		</div>
	);
};

export default EmailDashboard;
