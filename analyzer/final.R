message("Starting plotter...")
# X11()
library(anytime)

message("Loading data...")
data_with_time <- read.csv("out_time.csv")
data_notime <- read.csv("out_notime.csv")
data <- read.csv("out_complete.csv")
data_moves <- read.csv("out_moves.csv")
data_moves_flat <- read.csv("out_moves_flat.csv")
message("Data loaded!")
message("Plotting...")

svg('r_plots/data_all.svg')
plot(data,
    main="All data"
)

svg('r_plots/data_all_with_time.svg')
plot(data_with_time,
    main="All data with recorded time"
)

svg('r_plots/data_all_no_time.svg')
plot(data_notime,
    main="All data without recorded time"
)

# svg('r_plots/winrate.svg')
# hist(factor(data$won),
#     main="Score distribution"
# )

svg('r_plots/score_distribution.svg')
hist(data$score,
    main="Score distribution",
    xlab="Score"
)


svg('r_plots/score_over_time.svg')
plot(x = anytime(order(data_with_time$time/1000)), y = data_with_time$score,
    main="score over time",
    xlab="time",
    ylab="score"
)
# lines(x = order(data_with_time$time), y = data_with_time$score, type="b")

svg('r_plots/length_vs_score.svg')
plot(data$game_length, data$score,
    main="Score vs n. moves",
    ylab="Score",
    xlab="Number of moves"
)

svg('r_plots/move_distribution.svg')
plot(factor(data_moves_flat$move),
    main="General move distribution",
    xlab="Move played"
)

svg('r_plots/first_move_distribution.svg')
plot(factor(data$move_first),
    main="First move distribution",
    xlab="First move played"
)

svg('r_plots/first_move_vs_score.svg')
plot(factor(data$move_first), data$score,
    main="First move vs score",
    ylab="Score",
    xlab="First move played"
)

svg('r_plots/last_move_distribution.svg')
plot(factor(data$move_last),
    main="Last move distribution",
    xlab="Last move played"
)

svg('r_plots/last_move_vs_score.svg')
plot(factor(data$move_last), data$score,
    main="Last move vs score",
    ylab="Score",
    xlab="Last move played"
)

svg('r_plots/first_vs_last_move.svg')
plot(factor(data$move_first), factor(data$move_last),
    main="First move vs Last move",
    xlab="First move",
    ylab="Last move"
)

svg('r_plots/moves_river.svg', width = 20, height = 10)
library(riverplot)
#
edges = data_moves
#
nodes = data.frame(ID = unique(c(edges$N1, edges$N2)), stringsAsFactors = FALSE)
nodes$x = as.integer(substr(nodes$ID, 2, 3))
nodes$y = as.integer(sapply(substr(nodes$ID, 1, 1), charToRaw)) - 65
rownames(nodes) = nodes$ID
#
library(RColorBrewer)
#
palette = paste0(brewer.pal(4, "Set1"), "60")
#
styles = lapply(nodes$y, function(n) {
  list(col = palette[n+1], lty = 0, textcol = "black")
})
names(styles) = nodes$ID

rp <- makeRiver(nodes, edges) #, styles=styles

message("Rendering riverplot...")
plot(rp, plot_area = .95, yscale=0.06,
    main="First n moves",
    xlab="A = UP, B = RIGHT, C = DOWN, D = LEFT"
)

message("Rendering cover slide...")
svg('r_plots/cover.svg')
par(mar = c(0, 0, 0, 0))
plot(x = 0:10, y = 0:10, ann = F,bty = "n",type = "n",
     xaxt = "n", yaxt = "n")
text(x = 5,y = 5, paste("Oispa Halla\nn = ", nrow(data)))

message("Everything done!")