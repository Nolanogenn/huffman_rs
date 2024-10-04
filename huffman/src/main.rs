#![allow(warnings)]

use std::path::Path;
use anyhow;
use std::collections::HashMap;
use std::fmt;
use serde::{Serialize, Deserialize};
use bincode;

fn string_to_binary(input: &str) -> Vec<u8>{
    input.bytes().flat_map(|byte|{
        (0..8).rev().map(move |i| (byte >> i) & 1)
    })
    .collect()
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug)]
struct Node{
    freq: u32,
    ch: Option<char>,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>
}

impl fmt::Display for Node{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        let default_c: char = 'n';
        match &self.left {
            Some(left)=>{
                write!(f,"{{ freq: {}, left: {}, ",self.freq,left)?
            }
            None => {
                write!(f,"{{ freq: {},ch: {:?}",self.freq,self.ch)?
            }
        }
        match &self.right{
            Some(right)=>{
                write!(f, "right: {} }}",right)
            }
            None => {
                write!(f, " }}")
            }
        }
    }
}

impl Node{
    pub fn new(freq:u32, ch:Option<char>, left:Option<Box<Node>>, right:Option<Box<Node>>) -> Self{
        return Node{
            freq:freq,
            ch:ch,
            left:left,
            right:right
        }
    }
    pub fn freq(&self) -> &u32{
        return &self.freq
    }
    pub fn ch(&self) -> &Option<char>{
        return &self.ch
    }
    pub fn left(&self) -> &Option<Box<Node>>{
        return &self.left
    }
    pub fn right(&self) -> &Option<Box<Node>>{
        return &self.right
    }
}

fn new_box(n: Node) -> Box<Node>{
    Box::new(n)
}

fn join(a: Node, b: Node) -> Node {
    let newFreq: u32 =  a.freq +  b.freq;
    if  a.freq <=  b.freq{
        return Node::new(newFreq, None, Some(new_box(a)), Some(new_box(b)))
    }else{
        return Node::new(newFreq, None, Some(new_box(b)), Some(new_box(a)))
    }
}

fn frequency(s: &str) -> HashMap<char, u32> {
    let mut h = HashMap::new();
    for ch in s.chars(){
        let counter = h.entry(ch).or_insert(0);
        *counter += 1;
    }
    return h
}

fn assign_code(node: &Box<Node>, h: &mut HashMap<char, String>, s: String){
    if let Some(ch) = node.ch{
        h.insert(ch, s);
    } else {
        if let Some(ref l) = node.left {
            assign_code(l, h, (s.clone()+"0"));
        }
        if let Some(ref r) = node.right {
            assign_code(r, h, s.clone()+"1");
        }
    }
}

fn encode_string(s: &str, h: &HashMap<char, String>) -> String {
    let mut r = "".to_string();
    let mut t: Option<&String>;

    for ch in s.chars() {
        t = h.get(&ch);
        r.push_str(t.unwrap());
    }
    return r
}

fn decode_string(s: &str, root: &Box<Node>) -> String {
    let mut retval = "".to_string();
    let mut nodeptr = root;

    for x in s.chars(){
        if x == '0' {
            if let Some(ref l) = nodeptr.left {
                nodeptr = l;
            }
        } else {
            if let Some(ref r) = nodeptr.right {
                nodeptr = r;
            }
        }
        if let Some(ch) = nodeptr.ch {
            retval.push(ch);
            nodeptr = root
        }
    }
    return retval
}

fn generate_nodes(a:&String) ->Vec<Box<Node>>{
    let h = frequency(&a);
    let mut vecNodes: Vec<Box<Node>> = 
        h.iter()
        .map(|x| new_box(Node::new(*(x.1), Some(*x.0), None, None)))
        .collect();

    while vecNodes.len() > 1{
        vecNodes.sort_by(|a, b| (&(b.freq)).cmp(&(a.freq)));
        let a = vecNodes.pop().unwrap();
        let b = vecNodes.pop().unwrap();
        let mut c = new_box(join(*a, *b));
        vecNodes.push(c);
    }
    return vecNodes
}

fn huffman_encode(a:String) -> (Vec<u8>, Vec<u8>) {
    let mut vecNodes = generate_nodes(&a);
    let mut nodes_bytes: Vec<u8> = bincode::serialize(&vecNodes).unwrap();
    let mut h:HashMap<char, String> = HashMap::new();
    let root = vecNodes.pop().unwrap();
    assign_code(&root, &mut h, "".to_string());
    let encoded_string = encode_string(&a, &h);
    let mut string_bytes: Vec<u8> = bincode::serialize(&encoded_string).unwrap();
    return (nodes_bytes, string_bytes);
}


fn huffman_decode(n: Vec<u8> , s: Vec<u8>) -> String{ 
    let encoded_string: String = bincode::deserialize(&s[..]).unwrap();
    let mut encoded_tree: Vec<Box<Node>> = bincode::deserialize(&n[..]).unwrap();
    let root = encoded_tree.pop().unwrap();
    let decoded_string = decode_string(&encoded_string, &root);
    return decoded_string
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let s: String = "My name is Giovanni Giorgio but everybody calls me Giorgio".into();
        let (enc_n, enc_s) = huffman_encode(s);
        let x = huffman_decode(enc_n, enc_s);
    }
}
