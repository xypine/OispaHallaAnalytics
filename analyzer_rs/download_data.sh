echo "Downloading data..."
wget -T 0 https://hac.oispahalla.com:8002/overwatch/api/data -O data/data$(date -d "today" +"%Y%m%d%H%M").json
