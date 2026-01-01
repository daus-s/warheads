# warheads
#### Daus Carmichael

## Overview
warheads is a NBA prediction engine built off of the Elo algorithm and other machine learning algorithms. 

## Highlights
- **62% accuracy and a log loss of 0.640**
- **nelder mead optimization of `k` and `s` parameters**

## Usage
this section will go over how to run the program and generate predictions for the next several days. 

## Dependencies
 ```
 cargo (tested/developed on 1.92.0)
 ```

 ### Steps
 1) **Clone the repo**
 
 ```
 git clone git@github.com:daus-s/warheads.git
 ```
 
 2) **Create .env file**
 
 once the repo is initialized, in the main directory run this command. this will be used for warheads to generate source files to avoid unnecesary requests to nba server. 
 
 ```
 echo "DATA=$(pwd)/data" >> .env
 ```

3) **download and install corrections**

 As the data source is not prefect and is missing some records I have gone through the effort of correcting all of the necesary fields for games. I have linked the most updated version of the corrections [here](https://drive.google.com/file/d/1MMhyBxpiXeBEeimBcM2cf8SignFkjC0m/view?usp=drive_link). Future improvement to the data will always be welcome. 😊
 
 Once the corrections are downloaded (assuming the downloads go to `~/Downloads/corrections.zip`) unzip the file via
 ```
 unzip corrections.zip
 ``` 
 
 Verify te output of the  and then move the corrections into warheads data directory. 
 ```
 mv -f corrections path/to/warheads/data/nba/
 ```
 Force is required and will unfortunately overwrite existing corrections. (If you would like to improve this feature feel free to create a pull request and add to the scripts)
 
4) **Add headers.json file**

Create a header json file in the main project directory with at least these 3 fields. More may be required for different features, but 'User-Agent', 'x-postal-code', and 'Ocp-Apim-Subscription-Key' are always required. The last fields listed's value is left as a exercise to the reader.

5) **Run test suite or main program** 

From the warheads main directory you can run cargo test to see the full test suite. Any failures found are encouraged to be reported to the issues page. 

To test:
```
cargo test
```

To run:
```
cargo run
```
This will run the program, load the entire history of the nba into a Chronology object and train the Elo rating system for all NBA Players.

Once the ratings are created the program will query the NBA game api for the next 7 days of games.

As of 12/31/2025, elo v1 scores a 62.1% accuracy and a log loss value of 0.640.
