# OispaHallaAnalytics

## Tietokannan luominen
Asenna [sqlx](https://github.com/launchbadge/sqlx) ja aja seuraavat komennot projektin juurikansiossa:

```export DATABASE_URL="sqlite:db/analytics.db"``` kertoo sqlx:lle ja OHA:lle missä tietokanta sijaitsee.

```sqlx db create``` luo tietokannan annettuun sijaintiin.

```sqlx migrate run``` luo tietokantaan taulukon "OHA" ja tarvittavat sarakkeet.

Vaihtoehtoisesti kansiossa "db" olevan tiedoston "template.db" voi koittaa kopioida uuteen tiedostoon "analytics.db".

## Kokoaminen ja suorittaminen

Kokoa komennolla ```cargo build --release``` (luo suoritettavan tiedoston target/release/oispa_halla_analytics)

Kokoa & Aja komennolla ```cargo run --release```
### HTTPS-tuki
aja palvelin komennolla ```TLS_CERT="/etc/letsencrypt/live/hac.hallacoin.ml/fullchain.pem" TLS_KEY="/etc/letsencrypt/live/hac.hallacoin.ml/privkey.pem" ./target/release/oispa_halla_analytics```
## API:n Käyttö
Kts. 
[swagger (live)](https://hac.oispahalla.com:8002/overwatch)
[swagger (localhost)](https://localhost:8002/overwatch)
