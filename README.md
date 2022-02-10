# Building An Async CSV Parser

Recently, I wrote how to parse a big CSV file in Rust [here](https://dfrasca.hashnode.dev/rust-csv-processing), and I have also made a comparison against other languages, especially with javascript [here](https://dfrasca.hashnode.dev/rust-for-javascript-developers-csv-comparison) and running locally on my machine Rust resulted ten times faster.

This project is concentrated on the backend side of the application.

## How it works ##

![picture](https://github.com/ymwjbxxq/serverless_csv_processing/blob/main/readme/csv.jpeg)

- The [IMDb Datasets](https://www.imdb.com/interfaces/) file title.basics.tsv.gz is uploaded
- Amazon S3 emit the event to Amazon EventBridge
- Amazon EventBridge event will trigger AWS Lambda
- AWS Lambda downalod the file in memory and processe it.

## HOW TO DEPLOY ##

The project will deploy the following AWS services:

- Amazon S3
- Amazon EventBridge
- AWS Lambda

Assuming that your computer is setup, you need to build

```
make build
```

We can deploy all the applications. I use 

> --profile test 

Inside my MakeFile, and you may remove it.

```
make deploy
```

### Cleanup ###
```
make delete
```