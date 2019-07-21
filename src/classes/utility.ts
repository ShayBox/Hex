import type { TinyColor } from "@ctrl/tinycolor";
import { random } from "@ctrl/tinycolor";
import type { InteractionReplyOptions, InteractionUpdateOptions } from "discord.js";
import { ActionRowBuilder, ButtonBuilder, ButtonStyle, EmbedBuilder } from "discord.js";

const row = new ActionRowBuilder<ButtonBuilder>()
	.addComponents(
		new ButtonBuilder()
			.setCustomId("randomize")
			.setLabel("Randomize")
			.setStyle(ButtonStyle.Primary)
			.setEmoji("🎲"),
		new ButtonBuilder()
			.setCustomId("lighten")
			.setLabel("Lighten")
			.setStyle(ButtonStyle.Secondary)
			.setEmoji("🔆"),
		new ButtonBuilder()
			.setCustomId("darken")
			.setLabel("Darken")
			.setStyle(ButtonStyle.Secondary)
			.setEmoji("🔅"),
		new ButtonBuilder()
			.setCustomId("submit")
			.setLabel("Submit")
			.setStyle(ButtonStyle.Success)
			.setEmoji("✅"),
	);

function getEmbed(color: TinyColor): EmbedBuilder {
	const bg = color.toHexString().replace("#", "");
	const text = color.complement().toHexString().replace("#", "");
	return new EmbedBuilder()
		.setColor(color.toNumber())
		.setImage(`https://dummyimage.com/600x200/${bg}/${text}&text=${bg}`);
}

export function randomize(color?: TinyColor): InteractionReplyOptions | InteractionUpdateOptions {
	if (!color) {
		color = random();
	}

	const embed = getEmbed(color);
	return {
		components: [row],
		embeds: [embed],
	};
}

export function lighten(color: TinyColor): InteractionReplyOptions | InteractionUpdateOptions {
	color = color.lighten();
	const embed = getEmbed(color);
	return {
		components: [row],
		embeds: [embed],
	};
}

export function darken(color: TinyColor): InteractionReplyOptions | InteractionUpdateOptions {
	color = color.darken();
	const embed = getEmbed(color);
	return {
		components: [row],
		embeds: [embed],
	};
}
