Sys.setenv(LANG = "fi")
w=2048
h=2048
resolution=200
cat_order = c("Ylös", "Oikealle", "Alas", "Vasemmalle")

message("Connecting to database...")
library(RSQLite)
library(DBI)
db <- dbConnect(RSQLite::SQLite(), dbname="db/anal.db")

message("Starting plotter...")
# X11()
library(anytime)
library(gridExtra)

message("Loading data...")
data <- read.csv("out_all.csv")
data_move_distr <- read.csv("out_frequency_moves.csv")
data_move_distr <- data_move_distr[-nrow(data_move_distr),]
data_move_score <- read.csv("out_first_move_vs_score.csv")
# data_moves <- read.csv("out_moves.csv")
# data_moves_flat <- read.csv("out_moves_flat.csv")
message("Data loaded!")
message("Plotting...")

png('r_plots/data_all.png',width=w,height=h,res=resolution)
plot(data,
    main="All data"
)

# png('r_plots/winrate.png',width=w,height=h,res=resolution)
# hist(factor(data$won),
#     main="Score distribution"
# )
png('r_plots/length_distribution.png',width=w,height=h,res=resolution)
hist(data$game_length,
    main="Pelin pituuden jakauma",
    xlab="Pelin pituus",
    col="orange",
    prob = FALSE
)

png('r_plots/score_distribution.png',width=w,height=h,res=resolution)
hist(data$score,
    main="Pisteiden jakauma",
    xlab="Pisteet",
    col="orange",
    prob = FALSE
)

png('r_plots/score_distribution_grouped.png',width=w,height=h,res=resolution)
group_size = 12
mx = max(data$score)
hist(data$score,
    main=paste("Pisteiden jakauma (", group_size, " väliä)"),
    xlab="Pisteet",
    breaks=seq(0,mx,mx/group_size),
    xaxp=c(0,mx,group_size),
    col=rainbow(group_size),
    prob = FALSE
)


png('r_plots/score_over_time.png',width=w,height=h,res=resolution)
client <- order(data$client)
plot(x = client, y = data$score,
    main="Pisteet vs Aika",
    xlab="Aika",
    ylab="Pisteet"
)
abline(lm(data$score ~ client), col = "blue", lwd = 2)
# lines(x = order(data_with_time$time), y = data_with_time$score, type="b")

png('r_plots/length_vs_score.png',width=w,height=h,res=resolution)
plot(data$game_length, data$score,
    main="Siirtojen määrä vs Pisteet",
    ylab="Pisteet",
    xlab="Siirtojen määrä"
)
abline(lm(data$score ~ data$game_length), col = "blue", lwd = 2)

# png('r_plots/move_distribution.png',width=w,height=h,res=resolution)
# plot(factor(data_moves_flat$move),
#     main="General move distribution",
#     xlab="Move played"
# )

png('r_plots/general_move_distribution.png',width=w,height=h,res=resolution)
barplot(
    main="Siirtojen yleinen jakauma",
    height=data_move_distr$f_per,
    names=data_move_distr$Suunta,
    col=rainbow(4)
)

png('r_plots/first_move_distribution.png',width=w,height=h,res=resolution)
jakauma_tiedot <- as.data.frame(table(data$move_first) / nrow(data))
jakauma_tiedot <- jakauma_tiedot[order(factor(jakauma_tiedot$Var1, levels = cat_order)),]
head(jakauma_tiedot)
eka_jakauma <- factor(data$move_first,
    levels=cat_order
)
barplot(
    names=data_move_distr$Suunta,
    height=jakauma_tiedot$Freq,
    main="Ensimmäisen siirron jakauma",
    xlab="Ensimmäinen siirto",
    col=rainbow(4)
)

png('r_plots/first_move_vs_score.png',width=w*2,height=h,res=resolution)
plot(eka_jakauma, data$score,
    main="Ensimmäinen siirto vs pisteet",
    ylab="Pisteet",
    xlab="Ensimmäinen siirto",
    ylim=c(0,20000),
    col=rainbow(4)
)

png('r_plots/first_move_vs_score_table.png',width=w,height=h,res=resolution)
# Create a, b, c, d variables
b <- data_move_score$Suunta
e <- data_move_score$Pisteet_avg
# Join the variables to create a data frame
df <- data.frame(b,e)
names(df) <- c('Ensimmäinen siirto', 'Pisteiden keskiarvo')
grid.table(df)

png('r_plots/last_move_distribution.png',width=w,height=h,res=resolution)
plot(factor(data$move_last, levels=cat_order),
    main="Viimeisen siirron jakauma",
    xlab="Viimeinen siirto",
    col=rainbow(4)
)

png('r_plots/last_move_vs_score.png',width=w,height=h,res=resolution)
plot(factor(data$move_last), data$score,
    main="Viimeinen siirto vs pisteet",
    ylab="Pisteet",
    xlab="Viimeinen siirto",
    col=rainbow(4)
)

png('r_plots/first_vs_last_move.png',width=w,height=h,res=resolution)
plot(factor(data$move_first), factor(data$move_last),
    main="Ensimmäinen vs viimeinen siirto",
    xlab="Ensimmäinen siirto",
    ylab="Viimeinen siirto",
    col=rainbow(4)
)

# library("lubridate")
# png('r_plots/hour_distribution.png',width=w,height=h,res=resolution)
# datetime <- as_datetime((data$time/1000) + 3600*2)
# hist(hour(datetime),
#     main="Meneillään olevan tunnin jakauma pelin loppuessa",
#     xlab="Tunti (max 24)",
#     breaks=seq(0,24,1),
#     xaxp=c(0,24,24),
#     col=rainbow(24),
#     prob = FALSE
# )

# png('r_plots/hour_vs_score.png',width=w,height=h,res=resolution)
# datetime <- as_datetime((data$time/1000) + 3600*2)
# plot(factor(hour(datetime)),
#     data$score,
#     main="Tunti vs pisteet",
#     xlab="Tunti (max 24)",
#     ylab="Pisteet",
#     breaks=seq(0,24,1),
#     xaxp=c(0,24,24),
#     col=rainbow(24)
# )


# png('r_plots/weekday_distribution.png',width=w,height=h,res=resolution)
# days_ordered <- factor(
#     weekdays(datetime),
#     levels=c("maanantai", "tiistai", "keskiviikko", "torstai", "perjantai", "lauantai","sunnuntai")
# )
# levels(days_ordered)[levels(days_ordered)=="maanantai"] <- "MA"
# levels(days_ordered)[levels(days_ordered)=="tiistai"] <- "TI"
# levels(days_ordered)[levels(days_ordered)=="keskiviikko"] <- "KE"
# levels(days_ordered)[levels(days_ordered)=="torstai"] <- "TO"
# levels(days_ordered)[levels(days_ordered)=="perjantai"] <- "PE"
# levels(days_ordered)[levels(days_ordered)=="lauantai"] <- "LA"
# levels(days_ordered)[levels(days_ordered)=="sunnuntai"] <- "SU"
# plot(
#     days_ordered,
#     data$score,
#     main="Viikonpäivä vs Pisteet",
#     xlab="Viikonpäivä",
#     ylab="Pisteet",
#     col=rainbow(7),
# )

# png('r_plots/moves_river.png', width = 20, height = 10,width=w,height=h,res=resolution)
# library(riverplot)
# #
# edges = data_moves
# #
# nodes = data.frame(ID = unique(c(edges$N1, edges$N2)), stringsAsFactors = FALSE)
# nodes$x = as.integer(substr(nodes$ID, 2, 3))
# nodes$y = as.integer(sapply(substr(nodes$ID, 1, 1), charToRaw)) - 65
# rownames(nodes) = nodes$ID
# #
# library(RColorBrewer)
# #
# palette = paste0(brewer.pal(4, "Set1"), "60")
# #
# styles = lapply(nodes$y, function(n) {
#   list(col = palette[n+1], lty = 0, textcol = "black")
# })
# names(styles) = nodes$ID

# rp <- makeRiver(nodes, edges) #, styles=styles

# message("Rendering riverplot...")
# plot(rp, plot_area = .95, yscale=0.06,
#     main="First n moves",
#     xlab="A = UP, B = RIGHT, C = DOWN, D = LEFT"
# )

message("Rendering cover slide...")
png('r_plots/cover.png',width=w,height=h,res=resolution)
par(mar = c(0, 0, 0, 0))
plot(x = 0:10, y = 0:10, ann = F,bty = "n",type = "n",
     xaxt = "n", yaxt = "n")
text(x = 5,y = 5, paste("Oispa Halla\nn = ", nrow(data)))

# message("Rendering cover slide 2...")
# png('r_plots/cover2.png',width=w,height=h,res=resolution)
# # Create a, b, c, d variables
# b <- c('kaikki pelit', 'ei-hylätyt', 'hylätyt')
# d <- c(mean(data$score), mean(data_completed$score), mean(data_abandoned$score))
# f <- c(median(data$score), median(data_completed$score), median(data_abandoned$score))
# # Laske moodi tämän postauksen mukaan: https://stackoverflow.com/a/2547551
# g <- c(names(sort(-table(data$score)))[1], names(sort(-table(data_completed$score)))[1], names(sort(-table(data_abandoned$score)))[1])
# # Join the variables to create a data frame
# df <- data.frame(b,d,f,g)
# names(df) <- c('', 'pisteiden keskiarvo', 'pisteiden mediaani', 'pisteiden moodi')
# grid.table(df)

# message("Rendering cover slide 3...")
# png('r_plots/cover3.png',width=w,height=h,res=resolution)
# # Create a, b, c, d variables
# b <- c('kaikki pelit', 'ei-hylätyt', 'hylätyt')
# d <- c(mean(data$game_length), mean(data_completed$game_length), mean(data_abandoned$game_length))
# f <- c(median(data$game_length), median(data_completed$game_length), median(data_abandoned$game_length))
# g <- c(names(sort(-table(data$game_length)))[1], names(sort(-table(data_completed$game_length)))[1], names(sort(-table(data_abandoned$game_length)))[1])
# # Join the variables to create a data frame
# df <- data.frame(b,d,f,g)
# names(df) <- c('', 'pelin pituuden keskiarvo', 'pelin pituuden mediaani', 'pelin pituuden moodi')
# grid.table(df)

message("Kaikki valmista!")
