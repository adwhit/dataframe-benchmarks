library(data.table)

args <- commandArgs(trailingOnly=TRUE)
if (length(args) == 0) {
  stop("No file provided")
}

system.time(dt <- fread(args))
print("sum floats")
system.time(sum(dt[,floats]))
print("sum floats_nan")
system.time(sum(dt[,floats_nan], na.rm=TRUE))
print("Lowercase strings")
system.time(dt[,strings := tolower(strings)])
