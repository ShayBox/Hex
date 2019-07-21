#![allow(clippy::multiple_crate_versions)]

use std::time::Duration;

use anyhow::{Error, Result};
use csscolorparser::Color;
use poise::{
    builtins::register_globally,
    serenity_prelude::{
        ActivityData,
        ActivityType,
        ButtonStyle,
        ClientBuilder,
        ComponentInteractionCollector,
        CreateActionRow,
        CreateButton,
        CreateEmbed,
        CreateEmbedAuthor,
        CreateEmbedFooter,
        CreateInteractionResponse as CIR,
        EditRole,
        GatewayIntents,
        OnlineStatus,
        Permissions,
        Timestamp,
    },
    Context,
    CreateReply,
    Framework,
    FrameworkOptions,
};
#[cfg(debug_assertions)]
use poise::{samples::register_in_guild, serenity_prelude::GuildId};
use random_color::RandomColor;
use tracing::Level;

struct Data;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().with_max_level(Level::WARN).init();

    #[cfg(feature = "dotenv")]
    let _ = dotenvy::dotenv();

    let framework = Framework::builder()
        .options(FrameworkOptions {
            commands: vec![hex(), help()],
            ..Default::default()
        })
        .setup(|ctx, ready, framework| {
            Box::pin(async move {
                #[cfg(debug_assertions)]
                let guild_id = GuildId::new(824_865_729_445_888_041);
                let commands = &framework.options().commands;
                let shard = ready.shard.expect("start_autosharded");

                println!(
                    "[{}/{}] {} is ready in {} guilds",
                    shard.id,
                    shard.total,
                    ready.user.name,
                    ready.guilds.len()
                );

                #[cfg(debug_assertions)]
                register_in_guild(ctx, commands, guild_id).await?;
                register_globally(ctx, commands).await?;

                ctx.set_presence(
                    Some(ActivityData {
                        kind:  ActivityType::Playing,
                        name:  String::from("with Rainbows!"),
                        state: None,
                        url:   None,
                    }),
                    OnlineStatus::Online,
                );

                Ok(Data)
            })
        })
        .build();

    let token = std::env::var("DISCORD_TOKEN")?;
    let intents = GatewayIntents::non_privileged() | GatewayIntents::GUILDS;
    let mut client = ClientBuilder::new(token, intents)
        .framework(framework)
        .await?;

    client.start_autosharded().await?;

    Ok(())
}

/// Change your name color
#[allow(clippy::too_many_lines)]
#[poise::command(slash_command, track_edits)]
async fn hex(
    ctx: Context<'_, Data, Error>,
    #[description = "HEX, RGB, HSL, HSV, HWB, LAB, LCH"] color: Option<Color>,
) -> Result<()> {
    let mut color = match color {
        Some(color) => color,
        None => RandomColor::new().to_hex().parse()?,
    };

    let embed = CreateEmbed::default().title("⏳");
    let builder = CreateReply::default().embed(embed).ephemeral(true);
    let reply = ctx.send(builder).await?;

    'submit: loop {
        let components = vec![CreateActionRow::Buttons(vec![
            CreateButton::new("randomize")
                .emoji('🎲')
                .label("Randomize")
                .style(ButtonStyle::Primary),
            CreateButton::new("lighten")
                .emoji('🔆')
                .label("Lighten")
                .style(ButtonStyle::Secondary),
            CreateButton::new("darken")
                .emoji('🔅')
                .label("Darken")
                .style(ButtonStyle::Secondary),
            CreateButton::new("submit")
                .emoji('✅')
                .label("Submit")
                .style(ButtonStyle::Secondary),
        ])];

        let hex = color.to_hex_string().replace('#', "");
        let embed = CreateEmbed::default()
            .color(color.to_rgb8_tuple())
            .image(format!(
                "https://dummyimage.com/400x100/{hex}.png?text={hex}"
            ));

        let builder = CreateReply::default()
            .components(components)
            .embed(embed)
            .ephemeral(true);

        reply.edit(ctx, builder).await?;

        'edit: while let Some(mci) = ComponentInteractionCollector::new(ctx)
            .author_id(ctx.author().id)
            .channel_id(ctx.channel_id())
            .timeout(Duration::from_secs(60 * 60 * 24))
            .await
        {
            mci.create_response(ctx, CIR::Acknowledge).await?;
            match mci.data.custom_id.as_ref() {
                "randomize" => {
                    color = RandomColor::new().to_hex().parse()?;
                    break 'edit;
                }
                "lighten" => {
                    color = color.lighten(10.0);
                    break 'edit;
                }
                "darken" => {
                    color = color.darken(10.0);
                    break 'edit;
                }
                "submit" => {
                    break 'submit;
                }
                _ => {}
            }
        }
    }

    let guild = match ctx.guild() {
        Some(guild) => guild.clone(),
        None => return Ok(()),
    };

    let Some(author) = ctx.author_member().await else {
        return Ok(());
    };

    let Some(me) = guild.members.get(&ctx.framework().bot_id) else {
        return Ok(());
    };

    let Some(roles) = me.roles(ctx.cache()) else {
        return Ok(());
    };

    let position = roles
        .iter()
        .find(|r| r.managed)
        .map_or(0, |role| role.position - 1);

    let builder = EditRole::new()
        .name(format!("USER-{}", author.user.id))
        .colour(color.to_rgb8_tuple())
        .hoist(false)
        .mentionable(false)
        .permissions(Permissions::empty())
        .position(position)
        .audit_log_reason("User Changed Name Color");

    if let Some((_id, mut role)) = guild
        .roles
        .clone()
        .into_iter()
        .find(|(_, role)| role.name == format!("USER-{}", author.user.id))
    {
        role.edit(ctx, builder).await?;
        author.add_role(ctx, role.id).await?;
    } else {
        let role = guild.create_role(ctx, builder).await?;
        author.add_role(ctx, role.id).await?;
    }

    reply.delete(ctx).await?;

    Ok(())
}

/// Information about Hex
#[poise::command(slash_command, track_edits)]
async fn help(ctx: Context<'_, Data, Error>) -> Result<()> {
    let embed = CreateEmbed::default()
        .title("GitHub Source Code")
        .url("https://github.com/ShayBox/Hex")
        .description("Hex allows server members to change their name color")
        .timestamp(Timestamp::now())
        .author(CreateEmbedAuthor::new("").name("Shayne Hartford (ShayBox)").url("https://shaybox.com").icon_url("https://avatars1.githubusercontent.com/u/9505196"))
        .field("Commands", "", false)
        .field("Hex", "Change your name color", true)
        .field("Help", "Information about hex", true)
        .footer(CreateEmbedFooter::new("").text("Hex").icon_url("https://cdn.discordapp.com/avatars/600436180864991233/e16826d0d7ab4a74e703a72458e37757"));

    let builder = CreateReply::default().embed(embed).ephemeral(true);

    ctx.send(builder).await?;

    Ok(())
}
