# 🦫 BusyBeaverReduction
The Busy Beaver problem is a pivotal challenge in computability theory, highlighting the extremes of algorithmic behavior. It involves determining the maximum number of steps **S(N)** and the maximum output generated **&Sigma;(N)** by a *N-state halting Turing machine* and offering insights to gain a clearer comprehension of the big numbers.

This repository contains the code and the document for my Bachelor thesis for University of Bucharest, that I have written in the year *2023-2024*. This thesis emphasizes reducing the possible Turing machines that need to be examined to determine **S(N)** and **&Sigma;(N)** by defining *filtering methods* for machines that are not useful in the context of the busy beaver, respectively that are non-halting. The aim was to build a program in Rust that would execute, analyze, and classify Turing machines efficiently and be easily expandible for further research. 

## Filters
To find the *N-state busy beaver* winner, all the Turing machines with N states should be executed and analyzed. The total number of Turing machines with N states is equal to **[4(N+1)]<sup>2N</sup>**. For the smallest unknown N-state busy beaver, which is 5, the total number of machines is equal to 63,403,381,000,000. Running all of these Turing machines would not be the most efficient method to find the winner, thus filtering the machines is a must.

Not only running, but also generating such a big number of machines is unfeasible. During the generation process of all of these Turing machine configurations, some of them can be discarded right away, without knowing their full configuration, only after the generation of a few transitions for the transition functions. The filters that tackled this situation were called **generation filters**. The filters applied when the full configuration of the Turing machines was known were called **compile filters**, because the full transition function was compiled by the time the filters were applied. Lastly, when running the machines, the filters that identified the non-halting behavior of the machines were called **runtime filters**.

## Architecture
![architecture](https://github.com/VladWero08/BusyBeaverReduction/assets/77508081/d66dcf6a-48e4-44d1-8d37-70eb7c57fb31)

## Results
