# Trie Data Structure

## struct Trie { head: TrieNode }

`new() → Trie`
Calls on assemble_trie()

`assemble_trie(Vec<String>) → Result<TrieNode>`
This takes in a vector of strings which can be anything from words to phrases depending on how the Vec<String> is made up and assembles a TrieNode that is basically the content of the Trie. 

`get_suggestions() → Vec<String>`
Calls on NonTerminalNode::prefix_search() (see below) and then get_matches(&TrieNode, number_of_results: usize) -> Vec<String>. 
This returns number_of_results number of Strings that are made of up nodes that follow the node that NonTerminalNode::prefix_search() has returned to make up auto-completion suggestions.

## enum TrieNode

### TrieNode::TerminalNode { visits: u32 }
### TrieNode::NonTerminalNode { visits: u32, edges: HashMap<char, TrieNode> }
`prefix_search(&self, Vec<char>) → Option<TrieNode>`
This method takes in itself a vec of characters that is the prefix of what we want to auto-complete to return a NonTerminalNode in a Trie that matches the last character of the given prefix. 
