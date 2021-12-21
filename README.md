# Graph Algorithms in Rust
Interesting algorithms that use a underling graph structure.

## Description

This is a port of the graph algorithms of the following git repository from Python to Rust. <br>

* **GitHub - TheAlgorithms - Python** <br>
  [https://github.com/TheAlgorithms/Python/blob/master/DIRECTORY.md#graphs](https://github.com/TheAlgorithms/Python/blob/master/DIRECTORY.md#graphs)

The reason that I made this port was to remember same old algorithms and to learn new ones. A port between different languages is always a opportunity to learn the details in depth of an implementation of a algorithm. All this algorithms are implemented with an underling graph structure, sometimes it's a tree structure, but a tree is also a graph. Like the original implementation, the implementations in my port aren't the fastest implementations but they are elucidative of each algorithm. I tried to maintain them clean and simple. This helped me and I hope that it will help others. <br>
The algorithms are listed bellow.


## Graph's described in the following way

See the reference **Macro to implement a clean syntax graphs** below. <br>
<br>

```Rust
        // Adjacency list of graph.
       
       // Without Macro.
        let data_1: GraphAP = HashMap::from([
            (0, vec![1, 2]),
            (1, vec![0, 2]),
            (2, vec![0, 1, 3, 5]),
            (3, vec![2, 4]),
            (4, vec![3]),
            (5, vec![2, 6, 8]),
            (6, vec![5, 7]),
            (7, vec![6, 8]),
            (8, vec![5, 7]),
        ]);
        
        // With Macro.
        let data_2 = graph![
            0 => 1, 2;
            1 => 0, 2;
            2 => 0, 1, 3, 5;
            3 => 2, 4;
            4 => 3;
            5 => 2, 6, 8;
            6 => 5, 7;
            7 => 6, 8;
            8 => 5, 7
        ];

        assert_eq!(data_1, data_2);
```


## List of graph algorithms

1. **A star search algorithm** <br>
   **Wikipedia** <br>
   [https://en.wikipedia.org/wiki/A*_search_algorithm](https://en.wikipedia.org/wiki/A*_search_algorithm) <br>
   **Code** <br>
   [a_star.rs](./src/a_star.rs)

2. **Finding Articulation Points (or Cut Vertices) in Undirected Graph** <br>
   geekforgeeks theory page <br>
   [https://www.geeksforgeeks.org/articulation-points-or-cut-vertices-in-a-graph/](https://www.geeksforgeeks.org/articulation-points-or-cut-vertices-in-a-graph/) <br>
   **Wikipedia** <br>
   [https://en.wikipedia.org/wiki/Biconnected_component](https://en.wikipedia.org/wiki/Biconnected_component) <br>
   **Code** <br>
   [Articulation points](./src/articulation_points.rs)


## References

1. **Implement graph-like data structure in Rust** <br>
   [https://stackoverflow.com/questions/34747464/implement-graph-like-data-structure-in-rust](https://stackoverflow.com/questions/34747464/implement-graph-like-data-structure-in-rust)

2. **Macro to implement a clean syntax graphs** <br>
   [https://users.rust-lang.org/t/3-things-that-the-rust-standard-library-should-have/68825/9](https://users.rust-lang.org/t/3-things-that-the-rust-standard-library-should-have/68825/9)

3. **Macro visibility rules in Rust** <br>
   [https://users.rust-lang.org/t/3-things-that-the-rust-standard-library-should-have/68825/11](https://users.rust-lang.org/t/3-things-that-the-rust-standard-library-should-have/68825/11)


## License
MIT Open Source License, like the original license of the github repository TheAlgorithms. <br>
[https://github.com/TheAlgorithms/Python/blob/master/DIRECTORY.md#graphs](https://github.com/TheAlgorithms/Python/blob/master/DIRECTORY.md#graphs)


## Have fun!
Best regards, <br>
Joao Nuno Carvalho