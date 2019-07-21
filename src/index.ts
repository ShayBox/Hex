import { LogLevel, SapphireClient } from "@sapphire/framework";

const client = new SapphireClient({
	intents: ["Guilds"],
	logger: { level: LogLevel.Debug },
	presence: { activities: [{ name: "with Rainbows!" }] },
	shards: "auto",
});

void client.login();
