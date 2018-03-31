library(data.table)

args <- commandArgs(trailingOnly=TRUE)
if (length(args) == 0) {
  stop("No file provided")
}

system.time(dt <- fread(args))
system.time(sum(dt[,floats]))
system.time(sum(dt[,floats_nan], na.rm=TRUE))
