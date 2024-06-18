pub mod telegram {
    use teloxide::Bot;
    use teloxide::{prelude::*, utils::command::BotCommands};
    use crate::parse::parser::parse_html;

    #[derive(BotCommands, Clone)]
    #[allow(non_camel_case_types, unused)]
    #[command(rename_rule = "lowercase", description = "Команды для работы бота:")]
    enum Command {
        #[command(description = "Помощь")]
        Help,
        #[command(description = "off")]
        Start,
        #[command(description = "Получить гороскоп на сегодня для знака (Пример: /astro_now рак)")]
        Astro_Now(String),
        #[command(description = "Получить гороскоп на завтра для знака (Пример: /astro рак)")]
        Astro(String),
    }

    /*
        # Unix-like
        $ export TELOXIDE_TOKEN=<Your token here>
     */
    pub async fn start(){
        let bot = Bot::from_env();

        Command::repl(bot, answer).await;
    }

    async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
        match cmd {
            Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
            Command::Astro(zodiac) => {
                if let Some(astro) = parse_html(zodiac.to_lowercase(), true).await {
                    bot.send_message(msg.chat.id, astro.replace("сегодня", "завтра")).await?;
                    return Ok(())
                };

                bot.send_message(msg.chat.id, "Такого знака нет").await?

            }
            Command::Astro_Now(zodiac) => {
                if let Some(astro) = parse_html(zodiac.to_lowercase(), false).await {
                    bot.send_message(msg.chat.id, astro).await?;
                    return Ok(())
                };

                bot.send_message(msg.chat.id, "Такого знака нет").await?
            }
            Command::Start => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?
        };

        Ok(())
    }
}