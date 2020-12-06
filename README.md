# elbowgrease

Reformats AWS Elastic Load Balancer log files from the non-standardly delimited format into a TSV-like format where each field is separated with an Unit Separator character.

It can also filter the fields to be kept.

## Usage

```
cargo run --release --  -i access-logs/ -o test.tsv
```

## Performance

It's pretty fast: below, on my laptop, a gigabyte of compressed logs becomes 4 gigabytes of filtered uncompressed data in 37 seconds.

Switching to a non-regexp line parser would probably help, and ensuring less copies are being made.

```
$ du -hs logs/
1.0G	logs/
$ find logs/ -name '*.gz' | wc -l
   22002
$ time cargo run --release --  -i logs/ -o test.tsv -k proto -k time -k elb_status_code -k target_status_code -k request -k user_agent -k received_bytes -k sent_bytes
Found 22002 log files
Parsed 13270688 log records
Wrote 13270688 log records
Executed in   36.59 secs   fish           external
   usr time  462.00 secs  257.96 millis  461.74 secs
   sys time   12.23 secs  223.76 millis   12.01 secs
$ du -hs test.tsv
3.7G	test.tsv
```
