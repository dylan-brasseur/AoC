pub struct Node<'a> {
    adjacent: Vec<&'a Self>
}

pub struct Graph<'a> {
    nodes: Vec<Node<'a>>
}