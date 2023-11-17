Sys.setenv(LANG = "fi")
w <- 4096
h <- 4096
resolution <- 200
cat_order <- c("Ylös", "Oikealle", "Alas", "Vasemmalle")

message("Connecting to database...")
library(RSQLite)
library(DBI)
db <- dbConnect(RSQLite::SQLite(), dbname = "db/anal.db")

message("Starting plotter...")
library(anytime)
library(gridExtra)


message("Loading data...")
games <- dbGetQuery(db, "SELECT \"client\", \"hash\", \"created_at\" FROM games")
message("Games loaded!")
validations <- dbGetQuery(db, "SELECT \"game_hash\", \"score\", \"score_end\", \"score_margin\", \"breaks\", \"length\" FROM validations")
message("Validations loaded!")
moves <- dbGetQuery(db, "SELECT \"game_hash\", \"move_index\", \"direction\" FROM moves")
# Filter out the 4th direction
actual_moves <- moves[moves$direction != 4, ]
message("Moves loaded!")


# combine games and validations
data <- merge(games, validations, by.x = "hash", by.y = "game_hash")
message("Games combined!")

# Filter for the first moves
first_moves <- moves[moves$move_index == 0, ]
# Select only the game_hash and direction columns
first_moves <- first_moves[, c("game_hash", "direction")]
message("First moves aggregated!")

merged_moves_validations <- merge(moves, validations, by.x = "game_hash", by.y = "game_hash")

# Calculate the index of the last move
merged_moves_validations$last_move_index <- merged_moves_validations$length - 1

# Function to get the appropriate last move
get_appropriate_last_move <- function(data) {
    # Filter for the last move
    last_move <- data[data$move_index == data$last_move_index, ]

    # Check if the direction of the last move is 4
    if (last_move$direction == 4) {
        # If so, take the second last move
        second_last_index <- data$last_move_index - 1
        return(data[data$move_index == second_last_index, ])
    } else {
        # Otherwise, take the last move
        return(last_move)
    }
}

# Apply the function to each group of game
last_moves <- do.call(rbind, lapply(split(merged_moves_validations, merged_moves_validations$game_hash), get_appropriate_last_move))

# Select only the game_hash and direction columns
last_moves <- last_moves[, c("game_hash", "direction")]
last_moves$direction[last_moves$direction == 4] <- NA # or another value
message("Last moves aggregated!")

message("Data loaded!")
message("Plotting...")

# png("r_plots/data_all.png", width = w, height = h, res = resolution)
# plot(data,
#    main = "All data"
# )
# message("data_all.png rendered!")


# png('r_plots/winrate.png',width=w,height=h,res=resolution)
# hist(factor(data$won),
#     main="Score distribution"
# )
png("r_plots/length_distribution.png", width = w, height = h, res = resolution)
hist(data$length,
    main = "Pelin pituuden jakauma",
    xlab = "Pelin pituus",
    col = "orange",
    prob = FALSE
)
message("length_distribution.png rendered!")

png("r_plots/score_distribution.png", width = w, height = h, res = resolution)
hist(data$score,
    main = "Pisteiden jakauma",
    xlab = "Pisteet",
    col = "orange",
    prob = FALSE
)
message("score_distribution.png rendered!")

png("r_plots/score_distribution_grouped.png", width = w, height = h, res = resolution)
group_size <- 12
mx <- max(data$score)
hist(data$score,
    main = paste("Pisteiden jakauma (", group_size, " väliä)"),
    xlab = "Pisteet",
    breaks = seq(0, mx, mx / group_size),
    xaxp = c(0, mx, group_size),
    col = rainbow(group_size),
    prob = FALSE
)
message("score_distribution_grouped.png rendered!")


png("r_plots/score_over_time.png", width = w, height = h, res = resolution)
client <- order(data$client)
plot(
    x = client, y = data$score,
    main = "Pisteet vs Aika",
    xlab = "Aika",
    ylab = "Pisteet"
)
abline(lm(data$score ~ client), col = "blue", lwd = 2)
# lines(x = order(data_with_time$time), y = data_with_time$score, type="b")
message("score_over_time.png rendered!")

png("r_plots/length_vs_score.png", width = w, height = h, res = resolution)
plot(data$length, data$score,
    main = "Siirtojen määrä vs Pisteet",
    ylab = "Pisteet",
    xlab = "Siirtojen määrä"
)
abline(lm(data$score ~ data$length), col = "blue", lwd = 2)
message("length_vs_score.png rendered!")


png("r_plots/general_move_distribution.png", width = w, height = h, res = resolution)
data_move_distr <- as.data.frame(table(factor(actual_moves$direction, labels = cat_order)))
barplot(
    names.arg = data_move_distr$Var1,
    height = data_move_distr$Freq,
    main = "Siirtojen yleinen jakauma",
    xlab = "Siirto",
    col = rainbow(4)
)
message("general_move_distribution.png rendered!")

png("r_plots/first_move_distribution.png", width = w, height = h, res = resolution)

# Get the distribution of first moves in each direction
first_move_distr <- as.data.frame(table(factor(first_moves$direction, labels = cat_order)))
barplot(
    names.arg = first_move_distr$Var1,
    height = first_move_distr$Freq,
    main = "Ensimmäisen siirron jakauma",
    xlab = "Ensimmäinen siirto",
    col = rainbow(4)
)
message("first_move_distribution.png rendered!")

png("r_plots/first_move_vs_score.png", width = w, height = h, res = resolution)
# Merge first_moves with validations
first_move_scores <- merge(first_moves, validations, by.x = "game_hash", by.y = "game_hash")
first_move_scores$direction <- factor(first_move_scores$direction, labels = cat_order)

# Calculate average score for each direction
average_scores <- aggregate(score ~ direction, data = first_move_scores, mean)

# Create bar plot
barplot(
    names = average_scores$direction,
    height = average_scores$score,
    main = "Ensimmäinen siirto vs Pisteet",
    xlab = "Ensimmäinen siirto",
    ylab = "Pisteet",
    col = rainbow(nrow(average_scores))
)
message("first_move_vs_score.png rendered!")


png("r_plots/last_move_distribution.png", width = w, height = h, res = resolution)
last_move_distr <- as.data.frame(table(factor(last_moves$direction, labels = cat_order)))
barplot(
    names.arg = last_move_distr$Var1,
    height = last_move_distr$Freq,
    main = "Viimeisen siirron jakauma",
    xlab = "Viimeinen siirto",
    col = rainbow(4)
)
message("last_move_distribution.png rendered!")

png("r_plots/last_move_vs_score.png", width = w, height = h, res = resolution)
# Merge last_moves with validations
last_move_scores <- merge(last_moves, validations, by.x = "game_hash", by.y = "game_hash")
last_move_scores$direction <- factor(last_move_scores$direction, labels = cat_order)

# Calculate average score for each direction
average_scores <- aggregate(score ~ direction, data = last_move_scores, mean)

# Create bar plot
barplot(
    names = average_scores$direction,
    height = average_scores$score,
    main = "Viimeinen siirto vs Pisteet",
    xlab = "Viimeinen siirto",
    ylab = "Pisteet",
    col = rainbow(nrow(average_scores))
)
message("last_move_vs_score.png rendered!")

png("r_plots/first_vs_last_move.png", width = w, height = h, res = resolution)
# Merge first_moves and last_moves
combined_moves <- merge(first_moves, last_moves, by = "game_hash")
combined_moves$direction.x <- factor(combined_moves$direction.x, labels = cat_order)
combined_moves$direction.y <- factor(combined_moves$direction.y, labels = cat_order)

# Create a contingency table for the first and last moves
move_combinations <- table(combined_moves$direction.x, combined_moves$direction.y)

# Create a mosaic plot
mosaicplot(
    move_combinations,
    main = "Ensimmäinen siirto vs Viimeinen siirto",
    xlab = "Ensimmäinen siirto",
    ylab = "Viimeinen siirto",
    col = rainbow(length(move_combinations))
)
message("first_vs_last_move.png rendered!")


message("Rendering cover slide...")
png("r_plots/cover.png", width = w, height = h, res = resolution)
par(mar = c(0, 0, 0, 0))
plot(
    x = 0:10, y = 0:10, ann = F, bty = "n", type = "n",
    xaxt = "n", yaxt = "n"
)
text(x = 5, y = 5, paste("Oispa Halla\nn = ", nrow(data)))

message("cover.png rendered!")

library(ggplot2)

png("r_plots/paths.png", width = w, height = h, res = resolution)
library(ggplot2)

# Assuming 'moves' is your data frame with columns 'game_hash', 'move_index', and 'direction'

# Function to convert direction to coordinates change
get_coord_change <- function(direction) {
    switch(as.character(direction),
        "0" = c(0, 1), # Up
        "1" = c(1, 0), # Right
        "2" = c(0, -1), # Down
        "3" = c(-1, 0) # Left
    )
}

# Function to accumulate coordinates
accumulate_coords <- function(moves) {
    coords <- matrix(c(0, 0), nrow = 1, ncol = 2)
    for (i in 1:nrow(moves)) {
        change <- get_coord_change(moves$direction[i])
        coords <- rbind(coords, coords[nrow(coords), ] + change)
    }
    return(coords[-1, , drop = FALSE]) # Remove the starting point
}

# Split the data by game and accumulate coordinates
paths <- do.call(rbind, lapply(split(moves, moves$game_hash), accumulate_coords))

# Create a data frame for the paths
path_points <- as.data.frame(paths)
colnames(path_points) <- c("x", "y")

# Count the frequency of each coordinate
path_points$freq <- ave(rep(1, nrow(path_points)), path_points$x, path_points$y, FUN = sum)

# Plotting
ggplot(path_points, aes(x = x, y = y)) +
    geom_tile(aes(fill = freq), color = NA) +
    scale_fill_gradient(low = "blue", high = "red") +
    labs(title = "Game Move Heatmap", x = "X Coordinate", y = "Y Coordinate") +
    theme_minimal()



message("Kaikki valmista!")
