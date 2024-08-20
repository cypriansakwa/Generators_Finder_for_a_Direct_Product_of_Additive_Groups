## Overview
This project contains a Rust implementation for finding generators of the direct product of two cyclic groups $\mathbb{Z}_n \times \mathbb{Z}_m$. The primary use is to identify pairs of elements that generate the entire group, which can be useful in various mathematical and cryptographic applications. A cyclic group $\mathbb{Z}_n$ is a group of integers modulo $n$, with the group operation being addition modulo $n$. The direct product $\mathbb{Z}_n \times \mathbb{Z}_m$ is the set of ordered pairs of elements from these two groups, with the group operation being component-wise addition. 
In this program, we define a structure `ZnZm` to represent the direct product of two cyclic groups $\mathbb{Z}_n$ and $\mathbb{Z}_m$, and implement methods to find all generators of this product group.

## Code Structure

### `ZnZm` Struct

- **Fields:**
  - `n: u32`: The order of the first cyclic group $\mathbb{Z}_n$.
  - `m: u32`: The order of the second cyclic group $\mathbb{Z}_m$.

- **Methods:**
  - `new(n: u32, m: u32) -> Self`: Constructs a new `ZnZm` instance.
  - `find_generators(&self) -> Vec<(u32, u32)>`: Finds and returns a vector of all generators of the group $\mathbb{Z}_n \times \mathbb{Z}_m$.
  - `is_generator(&self, a: u32, b: u32) -> bool`: Determines if the pair $(a, b)$ is a generator of the group.
## How It Works
- Initialization: The program initializes the group $\mathbb{Z}_n \times \mathbb{Z}_m$ with the provided values of n and m.
- Finding Generators:
   - The find_generators method iterates over all possible pairs $(i,j)\in \mathbb{Z}_n \times \mathbb{Z}_m$.
   - For each pair, the is_generator method checks if the pair can generate the entire group. A pair is a generator if it can visit all elements of $\mathbb{Z}_n \times \mathbb{Z}_m$ exactly once before repeating.
- Output: The list of all generator pairs is printed to the console.
 ## Contributing
  - If you intend to contribute to this project, fork the repository and make a pull request.

  ## Installation

- To use this project, you need to have Rust installed on your machine.
- If Rust is not installed, follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install) to install it.
- After installing Rust, clone this repository or copy the code into a Rust project, Compile and run the code using cargo run.
## Usage
- To use this code, you can clone the repository.
- You can change the values of $n$ and $m$ in the main function to test different cases.
- Compile the Rust code using cargo:
>```
>cargo build
>cargo run
- This will compile and run the program, printing the generators of the specified $\mathbb{Z}_n \times \mathbb{Z}_m$.
## Acknowledgments
- Rust
### Clone the repository or copy the source code into a Rust project.
```bash

   git clone https://github.com/cypriansakwa/Generators_Finder_for_a_Direct_Product_of_Additive_Groups.git
   cd Generators_Finder_for_a_Direct_Product_of_Additive_Groups
​
 .
