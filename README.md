# grammarlin
CS 241 Final Project (Forked)

Names: 
- Rachel Lilley
- Moe Ariyoshi
- Nico Stuart
- Charlie McLaughlin

## Project Overview

Our project will simulate a word processor prediction feature. We are going to implement a trie with a list of frequently used words or sequence of words. If we have surplus time, it will be nice to have a user interface but if not, a simple program in the command line will be sufficient. The user interface can look like this, with a simple React front-end.

We aim to populate a data structure with a list of frequently used words or phrases. This list must contain duplicate strings to enable ‘weighted’ searches. Because this program will aim to make predictions based on large samples of text, the list of words will not need to be updated frequently. However, computation must be fast–specifically, to be useful, it must be faster than users type. To do this, whatever data structure is used will need to sit in memory, so it’s essential that the program has both low space complexity and very low search time. However, because the word list is not updated often, we can assemble a structure with high construction time and no option to add or remove specific words.

The most appropriate data structure to do this is likely a trie–a multiway tree structure which stores data about strings over a set alphabet. Ours will include edge weights based on the frequency with which each letter follows the ones before it. Upon initialization with a list of words, the structure will assign integer edge weights for every edge in the structure, and then once the tree is fully populated, fractional weights will be computed for each node’s exit edges. 

We are interested in developing a search algorithm that doesn’t require traversal of every path–one that stops considering traversals which have extremely low probability visible from their first few edges. This may require some tolerance for imprecision in searches. 

An essential goal for lowering memory complexity will be using a constrained alphabet–every node has to hold an array of size equal to the number of characters in our alphabet, so even a single extra character means every node’s edge array will require an additional field. Modifying the tree to include extra characters may require modification to its source code.

In order to complete this project, we will need to learn more about tries and their implementation, as well as how to implement them using Rust. Since one of the goals of our project is to prioritize time and potentially memory efficiency, we will need to learn more about how to implement time and memory efficient algorithms, which may be more difficult for us since none of us have taken Algorithms. We will also need to find out what makes the most sense to use for our sample text. This will depend on what we are trying to use our program for; for example, if we want it to simulate words/phrases that would be suggested when typing an email that might be different than if we were trying to simulate suggested Google searches. If we are trying to make a user interface rather than having it just run in the command line, we will also need to learn how to do that. If we decide to use React, we will need to learn more about it, but Moe knows about it, which will make it much easier if we decide to go that route.

## Proposed Schedule
Fri 12/1/23: Constructor function of the trie
Wed 12/6/23: Implementation of the search function
Sat 12/16/23: Application and optimization of the above functions, which will be worked on until the end of reading period 12/16/23.
Wed 12/20/23: Assembly of the presentation by 12/20/23.

