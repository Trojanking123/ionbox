import { MailComponent } from "@/components/Mail";
import { accounts, mails } from "@/components/FakeMail";

export default function MailPage() {
	const defaultLayout = [20, 32, 48];
	const defaultCollapsed = false;

	return (
		<div className="hidden flex-col md:flex">
			<MailComponent
				accounts={accounts}
				mails={mails}
				defaultLayout={defaultLayout}
				defaultCollapsed={defaultCollapsed}
				navCollapsedSize={4}
			/>
		</div>
	);
}
