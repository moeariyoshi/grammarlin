use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use serde::{ Deserialize, Serialize };

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Trie {
    head: TrieNode,
}

impl Trie {
    // api-oriented Trie constructor. Takes any list of Strings, but may behave unusually if they
    // include charecters which use more than one byte, such as emojis
    // notably, the constructor can handle multi word phrases.
    pub fn new(string_list: Vec<String>) -> Self {
        Trie {
            head: Self::assemble_trie(string_list).unwrap(),
        }
    }

    // private Trie assembly function, called by api-oriented constructor.
    fn assemble_trie(wordlist: Vec<String>) -> Result<TrieNode> {
        // head now has no '$' value, as I removed the charecter field from nodes
        let mut head: TrieNode = TrieNode::NonTerminalNode(NonTerminalNode::new());

        for word in wordlist.iter() {
            let mut chars = word.chars();
            let mut current_node: &mut TrieNode = &mut head;

            // What a mess! Sorry if you have to decipher or alter this. The enums make it tricky!
            // In essence, we read the next charecter of our input string and:
            //
            // if it's not the end of the string, we walk to (or create, then walk to) the
            // node of that charecter, then set that as the current node
            //
            // if it is the end of the string, we walk to or create a terminal node
            // increase its visits by one, and restart at the head with our next input string
            loop {
                current_node = match chars.next() {
                    Some(c) => match current_node {
                        TrieNode::TerminalNode(_) => {
                            return Err("Terminal nodes should not be accessible here!".into())
                        }

                        TrieNode::NonTerminalNode(current_non_terminal) => {
                            let entry = current_non_terminal
                                .edges
                                .entry(c.to_ascii_lowercase())
                                .or_insert(TrieNode::NonTerminalNode(NonTerminalNode::new()));
                            match entry {
                                TrieNode::NonTerminalNode(current_non_terminal) => {
                                    current_non_terminal.visits += 1;
                                    entry
                                }
                                TrieNode::TerminalNode(_) => {
                                    return Err(
                                        "Terminal nodes should not be accessible here!".into()
                                    )
                                }
                            }
                        }
                    },
                    None => match current_node {
                        TrieNode::TerminalNode(_) => {
                            return Err("Terminal nodes should not be accessible here!".into())
                        }
                        TrieNode::NonTerminalNode(current_non_terminal) => {
                            let entry = current_non_terminal
                                .edges
                                .entry('T')
                                .or_insert(TrieNode::TerminalNode(TerminalNode::new()));
                            match entry {
                                TrieNode::TerminalNode(current_terminal) => {
                                    current_terminal.visits += 1;
                                    break;
                                }
                                TrieNode::NonTerminalNode(_) => {
                                    return Err(
                                        "Non terminal nodes should not be accessible here!".into()
                                    )
                                }
                            }
                        }
                    },
                };
            }
        }

        Ok(head)
    }

    pub fn get_suggestions(&self, search_string: String, number_of_results: usize) -> Vec<String> {
        fn get_matches(node: &TrieNode, number_of_results: usize) -> Vec<String> {
            // private struct to be used in worklist
            // holds a reference to a node and copy of the entire String up to that node
            struct WorklistObject<'a> {
                current_string: String,
                node: &'a TrieNode,
            }

            // implements Ord (and its requirements) for WorklistObject
            // orders WorklistObject based on the total field of the node it points to
            impl Ord for WorklistObject<'_> {
                fn cmp(&self, other: &Self) -> Ordering {
                    let self_total = match self.node {
                        TrieNode::TerminalNode(node) => node.visits,
                        TrieNode::NonTerminalNode(node) => node.visits,
                    };
                    let other_total = match other.node {
                        TrieNode::TerminalNode(node) => node.visits,
                        TrieNode::NonTerminalNode(node) => node.visits,
                    };
                    self_total.cmp(&other_total)
                }
            }
            impl PartialOrd for WorklistObject<'_> {
                fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                    Some(self.cmp(other))
                }
            }
            impl Eq for WorklistObject<'_> {}
            impl PartialEq for WorklistObject<'_> {
                fn eq(&self, other: &Self) -> bool {
                    let self_total = match self.node {
                        TrieNode::TerminalNode(node) => node.visits,
                        TrieNode::NonTerminalNode(node) => node.visits,
                    };
                    let other_total = match other.node {
                        TrieNode::TerminalNode(node) => node.visits,
                        TrieNode::NonTerminalNode(node) => node.visits,
                    };
                    self_total == other_total
                }
            }

            // simple constructor function
            impl<'a> WorklistObject<'a> {
                fn new(node: &'a TrieNode, current_string: String) -> Self {
                    WorklistObject {
                        current_string,
                        node,
                    }
                }
            }

            // initiliazes worklist with the head node and an empty string
            // worklist is a BinaryHeap that will pop the node with the highest total
            let mut worklist: BinaryHeap<WorklistObject> = BinaryHeap::new();
            worklist.push(WorklistObject::new(node, String::new()));

            let mut result_vector = Vec::new();

            //edge case of asking for no results
            if number_of_results == 0 {
                return result_vector;
            }

            // loops through the worklist until it is empty or result_vector is full
            loop {
                // gets the next node
                let working_worklist_object: WorklistObject = match worklist.pop() {
                    None => break, // breaks the loop if worklist is empty (all nodes explored)
                    Some(t) => t,
                };

                match working_worklist_object.node {
                    // terminal node case:
                    // adds the current_string to the results
                    // then breaks loop if result_vector is desired length
                    // else continues loop
                    TrieNode::TerminalNode(_) => {
                        // moves the current_string and removes the trailing 'T' character
                        let mut result_string = working_worklist_object.current_string;
                        result_string.pop();

                        result_vector.push(result_string);
                        if result_vector.len() >= number_of_results {
                            break;
                        }
                        continue;
                    }

                    // pushs all child nodes to the worklist with proper strings
                    TrieNode::NonTerminalNode(node) => {
                        for (ch, next_node) in node.edges.iter() {
                            // clones string and adds the next letter
                            let mut updated_string = working_worklist_object.current_string.clone();
                            updated_string.push(*ch);

                            worklist.push(WorklistObject::new(next_node, updated_string));
                        }
                    }
                }
            }

            result_vector
        }

        //edge case to account for empty search strings which would crash prefix_search
        if search_string.is_empty() {
            return get_matches(&self.head, number_of_results);
        }

        let prefix = search_string.to_lowercase().chars().collect();
        let target_node = if let TrieNode::NonTerminalNode(node) = &self.head {
            match node.prefix_search(prefix) {
                Some(value) => value,
                None => return Vec::<String>::new(),
            }
        } else {
            return vec![];
        };

        get_matches(target_node, number_of_results)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
enum TrieNode {
    TerminalNode(TerminalNode),
    NonTerminalNode(NonTerminalNode),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct TerminalNode {
    visits: u32,
}

impl TerminalNode {
    fn new() -> Self {
        TerminalNode { visits: 0 }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct NonTerminalNode {
    visits: u32,
    edges: HashMap<char, TrieNode>,
}

impl NonTerminalNode {
    fn new() -> Self {
        NonTerminalNode {
            visits: 0,
            edges: HashMap::new(),
        }
    }

    // Returns the last TrieNode in the match
    fn prefix_search<'a>(&'a self, mut search_string: Vec<char>) -> Option<&'a TrieNode> {
        match self.edges.get_key_value(&search_string[0]) {
            Some((_c, node)) => {
                if let TrieNode::NonTerminalNode(next_node) = node {
                    if search_string.len() > 1 {
                        search_string.remove(0);
                        return next_node.prefix_search(search_string);
                    } else {
                        return Some(node);
                    }
                } else {
                    return None;
                }
            }
            _ => None,
        }
    } // Vec will be length of number of results
}
