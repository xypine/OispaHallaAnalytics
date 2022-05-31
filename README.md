# OispaHallaAnalytics
## Lataaminen
Tämä repo käyttää git -submoduuleja, jotka pitää ladata ennen projektin koontia.

Voit ladata ne automaattisesti repon kanssa samaan aikaan käyttämällä komentoa ```git clone --recurse-submodules git@github.com:hallabois/OispaHallaAnalytics.git``` tai suorittamalla komennon ```git submodule update --init --recursive``` aiemmin kloonatussa repossa.

Submoduulit voi päivittää myöhemmin komennolla ```git submodule update --remote --merge```.
## Koonti ja suorittaminen
Käyttää rustia, asenna se aluksi: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

Kokoa komennolla ```cargo build --release``` (luo suoritettavan tiedoston target/release/oispa_halla_analytics)

Kokoa & Aja komennolla ```cargo run --release```
## HTTPS-tuki
aja palvelin komennolla ```TLS_CERT="/path/to/cert" TLS_KEY="/path/to/key" ./target/release/oispa_halla_analytics```
## API:n Käyttö
Kts. 
[swagger (live)](https://hac.oispahalla.com:8002/overwatch)
[swagger (localhost)](https://localhost:8002/overwatch)

## Datan analysointi
Kts. [analyzer_rs](analyzer_rs)
