use nyoom_core::discord::Discord;

fn main() {
    let discord = nyoom_core::discord::DiscordStable::new();
    let token = discord.get_token();
    
    println!("{}", token);
}
