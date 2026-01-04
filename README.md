# WARheads
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
 
 once the repo is initialized, in the main directory run this command. This will be used for warheads to generate source data files to avoid unnecessary requests to NBA server. 
 
 ```
 echo "DATA=$(pwd)/data" >> .env
 ```

3) **download and install corrections**

 As the data source is not prefect and is missing some records I have gone through the effort of correcting all of the necesary fields for games. I have linked the most updated version of the corrections [here](https://drive.google.com/file/d/1MMhyBxpiXeBEeimBcM2cf8SignFkjC0m/view?usp=drive_link). Future improvement to the data will always be welcome. 😊
 
 Once the corrections are downloaded (assuming the downloads go to `~/Downloads/corrections.zip`) unzip the file via
 ```
 unzip corrections.zip
 ``` 
 
 Verify the output of the `unzip` command and then move the corrections into warheads data directory. 
 ```
 mv -f corrections path/to/warheads/data/nba/
 ```
 Force is required and will unfortunately overwrite existing corrections. If you would like to improve this feature feel free to create a pull request and add to the scripts directory.
 
4) **Add headers.json file**

Create a header json file in the main project directory with at least these 3 fields. More may be required for different features, but 'User-Agent', 'x-postal-code', and 'Ocp-Apim-Subscription-Key' are always required. 

You can find headers.json template [here](https://drive.google.com/file/d/1lbbGL15VlWI92mNxs3p7izBP6SQPFd49/view?usp=drive_link).

You can find these values by using the developer tools in your web browser. (Firefox highly recommended) 

Go to [nba.com](https://www.nba.com/games), open the developer tools, and look for the 'Network' tab. Find a request to the NBA API and look at the headers by filtering with the phrase **gamecardfeed** as below.

![How to find your headers on nba.com](https://lh3.googleusercontent.com/rd-d/ALs6j_ENIIcCFLNuW_IH9CbQIq-Ww7MdR8G1mAA05LxntOFfB9hetGgMWkY83X0d3qmU_YZgJyvJn0P03TC5ofB7Y2eR-ZzAKQoYE8e1kbW2FcOsOhO1I1Um3gpURAEwYD6h2n1S29dWOw9Lmz0uxnlZlb-RayCgoLGay7c7ow4TICI9v7Ps7IgTIB08GUhHpBETFXh5I7r97TRtCDh14FbfABv3MEoih3Dkj2tSqOZPUOgyLTGkNPzHnDNdhrhFlntdzAAEXeYtSTPi8EJZoW1jQBkWmemLX6XGszcMNbDtQhN1Z7nNtRTcbyWWuUTGDGJW7gKy4XDHKErtgmRFpRqpOOUUP4WCIJA5ar1XYgSOopJzIPexFT-HksEa692o12OYnXo_09Bu2ni-LTcoThpizWAe2j6ZU06PHiUZhBxr-UjafN8l_aodmViCGtmmH3tlBB_k4ACO-611ynWCYDOGqCmOGtu-5AmCNjCNRFx7S5LfMzL1TuvVFiQhenR09P-USdfJl9b569rKGChm_nGMdTndyhDU5Yop-7p1_WUF8K8ah8ws7OscZudrpkTMWZN0JHMBQI0Rp8DVo9pinGlRxpz7g70iPmD7GN3HHHFclODEZ2ON89YEPuPkeuQ4eF89lK4CVLy6ISUYO2OC0FsFfvf0MHzRiMXhhWRIwaMLKYG1vXUk-nhA9HmxSnY7Wbpgj2J6zypBADI3udrCQRkCUTRbW6YbenEf7MZNCqmO06uywMRYNHBGA2z6UK0hNYTUyxKnmeELNCsLhwgIpYtMZCG_CZW8xqs-Yjy4KVsbwmvSPg-Wq7lgr_r3DOdMYSTzRqp8ior6PjLaFhLEe8w7uzIbHqoATPEiq2LfH2BECz06FgIvI7MRmlaW4Ku-SBFd_U5KQkFHchuuDNxR7D4YhGxir4zIqhCyUckpjBpOe_FQsWPDYA2fUdNkEdz8oekE8U87JuE1OM-uIxHc9VDdJWbXqnrFONONVi5bp6O7D_AurUN-KlRLB9xdHcw-56SUG46QJfJOU4GqGvXh_N_AiHbv-dW-PXL6z3zGTBdDrFcVYEYgAkWQIqrv_DQHMJJTSUcRTWLueCVBpzPAzG_bMnNbS7xgsMjenCqxwQm1fRvotbMUADVg=w1865-h963?auditContext=prefetch)

Select the GET request and open the Request Headers tab. Copy the 3 needed field value's and paste them into your header.json files fields.

Copy the values for 'User-Agent', 'x-postal-code', and 'Ocp-Apim-Subscription-Key' and paste them into your headers.json file.

After this step your file structure should look like this:

```
.
├── Cargo.toml
├── headers.json
├── .env
├── README.md                                                   # you are here
├── data
│   ├── nba
│   │   ├── corrections
│   │   │   ├── ...
├── ...
```

5) **Run test suite or main program** 

Once you have completed all the steps above you can run the main program. It will first make a request to the NBA box scores API to fetch the data. It will then create a timeline of every game since the NBA was founded (1946-47 is the first year on record).

From the warheads main directory you can run cargo test to see the full test suite. Any failures found are encouraged to be reported to the issues page. 

To run the complete test suite:
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

## Results

todo: add a table of models v results (freq, logloss, cost) somehow and benchmarks
