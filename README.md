# 🦫 BusyBeaverReduction
The Busy Beaver problem is a fundamental challenge in computability theory, showcasing the extremes of algorithmic behavior. It involves finding the maximum number of steps **S(N)** and the maximum output **Σ(N)** produced by an N-state halting Turing machine, providing insights into the nature of large numbers.

This repository contains the code and documentation for my Bachelor's thesis at the University of Bucharest, written during the *2023-2024* academic year. The thesis focuses on reducing the number of Turing machines that need to be examined to determine S(N) and Σ(N) by defining methods to *filter out non-useful, non-halting machines*. The goal was to develop a Rust program that efficiently executes, analyzes, and classifies Turing machines, while being easily expandable for future research.

## Filters
To find the *N-state busy beaver* winner, all the Turing machines with N states should be executed and analyzed. The total number of Turing machines with N states is equal to **[4(N+1)]<sup>2N</sup>**. For the smallest unknown N-state busy beaver, which is 5, the total number of machines is equal to 63,403,381,000,000. Running all of these Turing machines would not be the most efficient method to find the winner, thus filtering the machines is a must.

Not only running, but also generating such a big number of machines is unfeasible. During the generation process of all of these Turing machine configurations, some of them can be discarded right away, without knowing their full configuration, only after the generation of a few transitions for the transition functions. The filters that tackled this situation were called **generation filters**. The filters applied when the full configuration of the Turing machines was known were called **compile filters**, because the full transition function was compiled by the time the filters were applied. Lastly, when running the machines, the filters that identified the non-halting behavior of the machines were called **runtime filters**.

## Architecture
![architecture](https://github.com/VladWero08/BusyBeaverReduction/assets/77508081/d66dcf6a-48e4-44d1-8d37-70eb7c57fb31)

## Results

The analysis of the filter's success involved applying the generation and compile filters to all Turing machines, and subsequently attempting to verify the conjectures for *BB(2) = 4*, *BB(3) = 6*, and *BB(4) = 13*.

### Generation and compile filters:

The generation and compile filters were successful and, for all instances of the busy beaver tackled, they filtered at least **94%** of the Turing machines. For the rest, the following step was to run only the machines for the number of steps in which the Turing machines for **S(2)**, **S(3)**, and **S(4)** halted.

|                         | BB(2)   | BB(3)     | BB(4)         |
|-------------------------|---------|-----------|---------------|
| Halting skippers        | 68.36%  | 71.23%    | 72.75%        |
| Start state loopers     | 14.06%  | 8.85%     | 6.41%         |
| Neighbour loopers       | 3.12%   | 2.72%     | 2.26%         |
| Naive beavers           | 3.52%   | 2.21%     | 1.60%         |
| Never halters           | 6.94%   | 9.73%     | 10.97%        |
| Never scores            | 0.46%   | 0.15%     | 0.04%         |
| Total                   | 96.46%  | 94.89%    | 94.04%        |
| Remaining               | 732     | 854,488   | 1,525,760,000 |

### Classification of the remaining Turing machines:

The winners were among halting machines for both beavers, they were not filtered out. Out of the following Turing machines, only the *non-halting* machines need to be executed for a larger number of steps to be proven that they will never halt, and afterward, the conjecture will be demonstrated to be true.

|              | BB(2)  | BB(3)     |
|--------------|--------|-----------|
| Steps        | 6      | 21        |
| Halting      | 416    | 402,156   |
| Non-halting  | 316    | 452,332   |

### Runtime filters results for holdouts with *1,000 steps*:

After running, only a *few* machines were left to be analyzed for *BB(3)*, and all the machines were filtered for *BB(2)*, meaning the conjecture was true.

|                      | BB(2)   | BB(3)    |
|----------------------|---------|----------|
| Short escapers       | 21.52%  | 19.67%   |
| Long escapers        | 43.67%  | 7.99%    |
| Cyclers              | 13.29%  | 12.06%   |
| Translated cyclers   | 21.52%  | 59.11%   |
| Total                | 100%    | 98.82%   |
| Remaining            | 0       | 5,352    |
