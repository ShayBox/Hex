import { TinyColor } from "@ctrl/tinycolor";
import { SlashCommandBuilder } from "@discordjs/builders";
import type { ApplicationCommandRegistry } from "@sapphire/framework";
import { Command, RegisterBehavior } from "@sapphire/framework";
import type { ChatInputCommandInteraction, InteractionReplyOptions } from "discord.js";
import { randomize } from "../classes/utility";

export class PingCommand extends Command {
	public constructor(context: Command.Context, options: Command.Options) {
		super(context, {
			...options,
			name: "hex",
			description: "Hex",
		});
	}

	public override registerApplicationCommands(registry: ApplicationCommandRegistry) {
		const builder = new SlashCommandBuilder()
			.setName(this.name)
			.setDescription(this.description);

		builder.addStringOption((string) => string
			.setName("color")
			.setDescription("Hexidecimal RGB, RGB, HSL, HSV, CSV"));

		registry.registerChatInputCommand(builder, {
			behaviorWhenNotIdentical: RegisterBehavior.Overwrite,
			// guildIds: ['590767367357857808'],
			registerCommandIfMissing: true,
		});
	}

	public async chatInputRun(interaction: ChatInputCommandInteraction) {
		const color = interaction.options.getString("color");
		await interaction.reply({
			ephemeral: true,
			...randomize(color ? new TinyColor(color) : undefined) as InteractionReplyOptions,
		});
	}
}
