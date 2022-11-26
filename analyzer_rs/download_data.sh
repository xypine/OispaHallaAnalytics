echo "Downloading data..."
wget -T 0 https://analytics.oispahalla.com/api/data -O data/data$(date -d "today" +"%Y%m%d%H%M").json
