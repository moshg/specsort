# bool

## v1

test bench_random_100     ... bench:         126 ns/iter (+/- 5)
test bench_random_10000   ... bench:       6,008 ns/iter (+/- 312)
test bench_random_1000000 ... bench:     615,075 ns/iter (+/- 41,967)
test bench_sorted_100     ... bench:         125 ns/iter (+/- 7)
test bench_sorted_10000   ... bench:       5,989 ns/iter (+/- 394)
test bench_sorted_1000000 ... bench:     617,310 ns/iter (+/- 69,423)

## by_key

test bench_random_100     ... bench:         131 ns/iter (+/- 15)
test bench_random_10000   ... bench:      31,393 ns/iter (+/- 4,177)
test bench_random_1000000 ... bench:   3,972,270 ns/iter (+/- 139,134)
test bench_sorted_100     ... bench:         120 ns/iter (+/- 11)
test bench_sorted_10000   ... bench:       5,970 ns/iter (+/- 706)
test bench_sorted_1000000 ... bench:     599,520 ns/iter (+/- 45,317)

## bucket

test bench_random_100     ... bench:         240 ns/iter (+/- 20)
test bench_random_10000   ... bench:      21,254 ns/iter (+/- 4,715)
test bench_random_1000000 ... bench:   4,057,670 ns/iter (+/- 182,575)
test bench_sorted_100     ... bench:         231 ns/iter (+/- 19)
test bench_sorted_10000   ... bench:       4,465 ns/iter (+/- 57)
test bench_sorted_1000000 ... bench:     780,685 ns/iter (+/- 59,525)

## v2

test bench_random_100     ... bench:         112 ns/iter (+/- 15)
test bench_random_10000   ... bench:      18,080 ns/iter (+/- 5,025)
test bench_random_1000000 ... bench:   3,051,970 ns/iter (+/- 79,452)
test bench_sorted_100     ... bench:         102 ns/iter (+/- 17)
test bench_sorted_10000   ... bench:       3,319 ns/iter (+/- 49)
test bench_sorted_1000000 ... bench:     407,964 ns/iter (+/- 12,477)

## std

test bench_random_100     ... bench:         322 ns/iter (+/- 28)
test bench_random_10000   ... bench:      11,788 ns/iter (+/- 972)
test bench_random_1000000 ... bench:   1,616,345 ns/iter (+/- 89,701)
test bench_sorted_100     ... bench:         133 ns/iter (+/- 3)
test bench_sorted_10000   ... bench:       4,615 ns/iter (+/- 179)
test bench_sorted_1000000 ... bench:     464,398 ns/iter (+/- 17,288)

## std stable

test bench_random_100     ... bench:       1,404 ns/iter (+/- 91)
test bench_random_10000   ... bench:     310,143 ns/iter (+/- 18,616)
test bench_random_1000000 ... bench:  47,354,000 ns/iter (+/- 4,160,509)
test bench_sorted_100     ... bench:         234 ns/iter (+/- 22)
test bench_sorted_10000   ... bench:       7,383 ns/iter (+/- 1,184)
test bench_sorted_1000000 ... bench:     731,770 ns/iter (+/- 106,383)

# u8

## bucket sort

test bench_random_100     ... bench:         473 ns/iter (+/- 401)
test bench_random_10000   ... bench:       6,028 ns/iter (+/- 683)
test bench_random_1000000 ... bench:     553,730 ns/iter (+/- 40,024)
test bench_sorted_100     ... bench:         499 ns/iter (+/- 483)
test bench_sorted_10000   ... bench:      13,277 ns/iter (+/- 1,039)
test bench_sorted_1000000 ... bench:   1,588,590 ns/iter (+/- 115,517)

## std

test bench_random_100     ... bench:         621 ns/iter (+/- 42)
test bench_random_10000   ... bench:     105,898 ns/iter (+/- 10,461)
test bench_random_1000000 ... bench:   8,760,640 ns/iter (+/- 197,420)
test bench_sorted_100     ... bench:         135 ns/iter (+/- 6)
test bench_sorted_10000   ... bench:       4,655 ns/iter (+/- 188)
test bench_sorted_1000000 ... bench:     461,256 ns/iter (+/- 12,514)