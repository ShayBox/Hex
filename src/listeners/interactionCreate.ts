import { TinyColor } from "@ctrl/tinycolor";
import { Listener } from "@sapphire/framework";
import type { Interaction, InteractionUpdateOptions } from "discord.js";
import { GuildMemberRoleManager } from "discord.js";
import { darken, lighten, randomize } from "../classes/utility";

export class InteractionCreateListener extends Listener {
	public constructor(context: Listener.Context, options: Listener.Options) {
		super(context, {
			...options,
			event: "interactionCreate",
		});
	}

	public async run(interaction: Interaction) {
		if (!interaction.isButton()) {
			return;
		}

		const embeds = interaction.message.embeds;
		if (embeds.length === 0) {
			return;
		}

		const color = embeds[0].color;
		if (!color) {
			return;
		}

		const tiny = new TinyColor(color);

		switch (interaction.customId) {
			case "randomize": {
				await interaction.update(randomize() as InteractionUpdateOptions);
				break;
			}

			case "lighten": {
				await interaction.update(lighten(tiny) as InteractionUpdateOptions);
				break;
			}

			case "darken": {
				await interaction.update(darken(tiny) as InteractionUpdateOptions);
				break;
			}

			case "submit": {
				const guild = interaction.guild;
				if (!guild) {
					return;
				}

				const member = interaction.member;
				if (!member) {
					return;
				}

				const managedRole = guild.members.me!.roles.cache.find((role) => role.managed);
				const highestRole = guild.members.me!.roles.highest;
				const data = {
					color,
					name: `USER-${interaction.user.id}`,
					permissions: [],
					position: managedRole ? managedRole.position : 0,
				};
				const role = guild.roles.cache.find((role) => role.name === data.name);
				const memberRoles = member.roles;

				if (role) {
					if (role.position > highestRole.position) {
						return interaction.update({
							content: `${role.name} is above ${highestRole.name}`,
							components: [],
							embeds: [],
						});
					}

					// Discord Bug: Positions are +1 when editing roles compared to creating
					await role.edit({
						...data,
						position: managedRole ? data.position - 1 : data.position,
					});
					if (memberRoles instanceof GuildMemberRoleManager && !memberRoles.cache.has(role.id)) {
						await memberRoles.add(role);
					}
				} else {
					const hexRole = await guild.roles.create(data);
					if (memberRoles instanceof GuildMemberRoleManager) {
						await memberRoles.add(hexRole);
					}
				}

				await interaction.update({
					content: "Done",
					components: [],
					embeds: [],
				});
				break;
			}
		}
	}
}
