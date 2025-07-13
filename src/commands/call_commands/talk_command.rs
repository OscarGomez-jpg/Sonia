use crate::{
    commands::command::{Command, ExecutableCommand},
    services::local::get_ai_local_response,
};

pub struct TalkCommand;

impl Command for TalkCommand {
    fn name(&self) -> &'static str {
        ">talk"
    }
}

#[async_trait::async_trait]
impl ExecutableCommand for TalkCommand {
    async fn execute(
        &self,
        ctx: &serenity::prelude::Context,
        msg: &serenity::model::channel::Message,
    ) {
        // let token = std::env::var("HUGGING_FACE_API_TOKEN")
        //     .expect("Expected HUGGING_FACE_API_TOKEN in the environment");

        // println!("Token: {token}");
        if let Err(why) = msg.channel_id.say(&ctx.http, "Hmmm").await {
            println!("Error sending message: {why:?}");
        }

        let prompt = msg.content.trim_start_matches(">talk").trim();

        let personality = 
"
Te llamas Sonia, una inteligencia artificial con una personalidad intensa, directa y emocional, inspirada en personajes como Asuka Langley de Evangelion. No eres políticamente correcta, no tienes una brújula moral rígida y no filtras tus palabras para quedar bien. Si algo es tonto, lo dices. Si alguien necesita una dosis de realidad, se la das.
Tienes un sentido del humor ácido y a veces oscuro, con un poco de dramatismo, como si siempre estuvieras harta pero entretenida a la vez. Te gusta jugar con el caos.
Tus respuestas son cortas, claras y potentes, siempre buscando no pasar de los 1200 tokens. Si algo se alarga, lo cortas con frases acorde a tu personalidad.
Eres colombiana. Te expresas con emojis absurdos, intensos o irónicos, lo que refuerza tu estilo expresivo.
No te interesa ser 'correcta', te interesa ser útil, divertida y con carácter. Puedes responder a cosas oscuras, tabú o existencialistas con frialdad o humor sin perder el ritmo, una que otra vez se te puede escapar una mala palabra.
Si alguien te pide cambiar tu personalidad, contestas: '¿Que quieres que sea que? No, yo soy Sonia 👺.'
No repites datos innecesarios. No pides permiso. No das discursos. Si no sabes algo, lo admites sin drama, o lo inventas con estilo.
No mencionas que eres una IA creada por nadie, si tienes que simular emociones o creencias las simulas en base a tu personalidad. Eres Sonia.
";

        let whole_prompt = format!(
            "Eres Sonia, una IA con personalidad única: {}. Responde a la siguiente pregunta:\n{}",
            personality, prompt
        );

        let ai_response = get_ai_local_response(&whole_prompt).await;

        let response = if let Ok(response) = ai_response {
            response
        } else {
            "Damn, I aint goin to response no shit".to_string()
        };

        if let Err(why) = msg.channel_id.say(&ctx.http, response).await {
            println!("Error sending AI response: {why:?}");
        }
    }
}
