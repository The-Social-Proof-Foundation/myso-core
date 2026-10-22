import { defineRailway, project, service } from "railway/iac";

// Owns only the faucet. Other services in this environment stay on their own railway.toml files.
export const partial = "myso-faucet";

export default defineRailway(() => {
	const faucet = service("myso-faucet", {
		build: {
			builder: "DOCKERFILE",
			dockerfilePath: "crates/myso-faucet/Dockerfile",
		},
		start: "/app/bin/setup-wallet.sh",
	});

	return project("heartfelt-elegance", { resources: [faucet] });
});
