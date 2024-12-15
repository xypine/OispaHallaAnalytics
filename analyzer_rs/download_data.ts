type Game = {
  hash: string;
  data_raw: string;
  client?: string;
  created_at: string;
};

async function download_page(page: number, page_size: number) {
  console.log(`Downloading page ${page}...`);
  const response = await fetch(
    `https://analytics.oispahalla.com/api/data?page=${page}&page_size=${page_size}`
  );
  if (!response.ok) {
    throw new Error(`Failed to fetch page ${page}, ${response.status}`);
  }
  try {
    const json = await response.json();
    return json as Game[];
  } catch (e) {
    console.log(`Failed to parse page ${page}, ${e}`);
    throw e;
  }
}

function getFilename(): string {
  // Create a new Date object for the current time
  const now = new Date();

  // Pad single digits with leading zero
  const pad = (num: number) => num.toString().padStart(2, "0");

  // Format the date as YYYYMMDDHHMM
  const year = now.getFullYear();
  const month = pad(now.getMonth() + 1); // getMonth() returns 0-11
  const day = pad(now.getDate());
  const hours = pad(now.getHours());
  const minutes = pad(now.getMinutes());

  const filename = `data/data${year}${month}${day}${hours}${minutes}.json`;

  return filename;
}

async function write_games(games: Game[]) {
  console.log(`Writing ${games.length} games to disk`);
  const filename = getFilename();
  const encoder = new TextEncoder();
  // JSON.stringify won't work for strings over ~500mb, so we have to construct the json by hand
  console.log("Serializing into JSON & writing...");
  const file = await Deno.create(filename);
  const writer = file.writable.getWriter();
  await writer.write(encoder.encode(`{"data": [`));
  const comma = encoder.encode(",");

  let is_first = true;
  for(const game of games) {
    if(!is_first) {
      await writer.write(comma);
    }
    const game_json = JSON.stringify(game);
    const encoded = encoder.encode(game_json);
    await writer.write(encoded);
    is_first = false;
  }

  await writer.write(new TextEncoder().encode(`]}`));
}

async function sleep(ms: number) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

async function download_data(page_size: number) {
  console.log("Downloading data...");
  console.log("Fetching info...");
  const info = await fetch("https://analytics.oispahalla.com/api/stats");
  if (!info.ok) {
    throw new Error("Failed to fetch info");
  }
  const info_json = await info.json();
  let recorded_games = info_json["recorded_games"];
  if (recorded_games === undefined) {
    throw new Error("Failed to fetch recorded_games");
  }
  recorded_games = +recorded_games;
  console.log(`Recorded games: ${recorded_games}`);
  const pages = Math.ceil(recorded_games / page_size);
  console.log(`Pages to download: ${pages}`);
  const games: Game[] = [];
  for (let page = 0; page < pages; page++) {
    const page_games = await download_page(page, page_size);
    games.push(...page_games);
    // Sleep for 10ms to not overload the server
    await sleep(10);
  }
  console.log(`Downloaded ${games.length} games`);
  await write_games(games);
}

download_data(500);
